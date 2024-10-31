pub mod db_queue;

use async_trait::async_trait;
use futures::StreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    ExchangeKind,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{env, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{error, info};

// Generic error type for queue operations
#[derive(Debug, Error)]
pub enum QueueError {
    #[error("RabbitMQ error: {0}")]
    Rabbit(#[from] lapin::Error),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Clone, Debug)]
pub struct QueueConfig {
    pub amqp_url: String,
    pub queue_name: String,
    pub exchange_name: String,
    pub consumer_tag: String,
    pub retry_delay: u32,
    pub message_ttl: u32,
    pub prefetch_count: u16,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            amqp_url: env::var("AMQP_URL")
                .unwrap_or_else(|_| "amqp://guests:guest@localhost:5672".to_string()),
            queue_name: "default_queue".to_string(),
            exchange_name: "default_exchange".to_string(),
            consumer_tag: "default_consumer".to_string(),
            retry_delay: 30000,   // 30 seconds
            message_ttl: 1800000, // 30 minutes
            prefetch_count: 10,
        }
    }
}

// Builder for QueueConfig
#[derive(Default)]
pub struct QueueConfigBuilder {
    amqp_url: Option<String>,
    queue_name: Option<String>,
    exchange_name: Option<String>,
    consumer_tag: Option<String>,
    retry_delay: Option<u32>,
    message_ttl: Option<u32>,
    prefetch_count: Option<u16>,
}

impl QueueConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn amqp_url(mut self, url: impl Into<String>) -> Self {
        self.amqp_url = Some(url.into());
        self
    }

    pub fn queue_name(mut self, name: impl Into<String>) -> Self {
        self.queue_name = Some(name.into());
        self
    }

    pub fn exchange_name(mut self, name: impl Into<String>) -> Self {
        self.exchange_name = Some(name.into());
        self
    }

    pub fn consumer_tag(mut self, tag: impl Into<String>) -> Self {
        self.consumer_tag = Some(tag.into());
        self
    }

    pub fn retry_delay(mut self, delay: u32) -> Self {
        self.retry_delay = Some(delay);
        self
    }

    pub fn message_ttl(mut self, ttl: u32) -> Self {
        self.message_ttl = Some(ttl);
        self
    }

    pub fn prefetch_count(mut self, count: u16) -> Self {
        self.prefetch_count = Some(count);
        self
    }

    pub fn build(self) -> QueueConfig {
        let defaults = QueueConfig::default();

        QueueConfig {
            amqp_url: self.amqp_url.unwrap_or(defaults.amqp_url),
            queue_name: self.queue_name.unwrap_or(defaults.queue_name),
            exchange_name: self.exchange_name.unwrap_or(defaults.exchange_name),
            consumer_tag: self.consumer_tag.unwrap_or(defaults.consumer_tag),
            retry_delay: self.retry_delay.unwrap_or(defaults.retry_delay),
            message_ttl: self.message_ttl.unwrap_or(defaults.message_ttl),
            prefetch_count: self.prefetch_count.unwrap_or(defaults.prefetch_count),
        }
    }
}

impl QueueConfig {
    pub fn builder() -> QueueConfigBuilder {
        QueueConfigBuilder::new()
    }
}

// Trait for operation processors
#[async_trait]
pub trait OperationProcessor: Send + Sync + 'static {
    type Operation: Send + Sync + Serialize + DeserializeOwned + Clone + 'static;

    async fn process(&self, operation: Self::Operation) -> Result<(), String>;
    fn operation_type(&self) -> &'static str;
}

// Generic queue implementation
pub struct Queue<P: OperationProcessor> {
    channel: Arc<Channel>,
    connection: Arc<Connection>,
    processor: Arc<P>,
    config: QueueConfig,
    consumer_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl<P: OperationProcessor> Queue<P> {
    pub async fn new(processor: P, config: QueueConfig) -> Result<Self, QueueError> {
        let connection = Connection::connect(
            &config.amqp_url,
            ConnectionProperties::default()
                .with_connection_name(format!("{}_connection", config.queue_name).into())
                .with_executor(tokio_executor_trait::Tokio::current())
                .with_reactor(tokio_reactor_trait::Tokio),
        )
        .await
        .map_err(|e| QueueError::Connection(e.to_string()))?;
        let connection = Arc::new(connection);

        // check connection status
        println!("RabbitMQ connection status: {:?}", connection.status());

        let channel = connection.create_channel().await?;
        let channel = Arc::new(channel);

        // Set up QoS
        channel
            .basic_qos(config.prefetch_count, BasicQosOptions::default())
            .await?;

        // Set up exchanges and queues
        Self::setup_infrastructure(&channel, &config).await?;

        let queue = Self {
            channel,
            connection,
            processor: Arc::new(processor),
            config,
            consumer_handle: Arc::new(Mutex::new(None)),
        };

        queue.start_consuming().await?;

        Ok(queue)
    }

    async fn setup_infrastructure(
        channel: &Channel,
        config: &QueueConfig,
    ) -> Result<(), QueueError> {
        // Main exchange
        channel
            .exchange_declare(
                &config.exchange_name,
                ExchangeKind::Direct,
                ExchangeDeclareOptions {
                    durable: true,
                    ..ExchangeDeclareOptions::default()
                },
                FieldTable::default(),
            )
            .await?;

        // Dead letter exchange
        let dlx_name = format!("{}_dlx", config.exchange_name);
        channel
            .exchange_declare(
                &dlx_name,
                ExchangeKind::Direct,
                ExchangeDeclareOptions {
                    durable: true,
                    ..ExchangeDeclareOptions::default()
                },
                FieldTable::default(),
            )
            .await?;

        // Main queue
        let mut queue_args = FieldTable::default();
        queue_args.insert(
            "x-dead-letter-exchange".into(),
            lapin::types::AMQPValue::LongString(dlx_name.clone().into()),
        );

        channel
            .queue_declare(
                &config.queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..QueueDeclareOptions::default()
                },
                queue_args,
            )
            .await?;

        // Retry queue
        let retry_queue = format!("{}_retry", config.queue_name);
        let mut retry_args = FieldTable::default();
        retry_args.insert(
            "x-dead-letter-exchange".into(),
            lapin::types::AMQPValue::LongString(config.exchange_name.clone().into()),
        );
        retry_args.insert(
            "x-message-ttl".into(),
            lapin::types::AMQPValue::LongUInt(config.retry_delay.into()),
        );

        channel
            .queue_declare(
                &retry_queue,
                QueueDeclareOptions {
                    durable: true,
                    ..QueueDeclareOptions::default()
                },
                retry_args,
            )
            .await?;

        // Bind queues
        channel
            .queue_bind(
                &config.queue_name,
                &config.exchange_name,
                &config.queue_name,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(())
    }

    pub async fn publish(&self, operation: P::Operation) -> Result<(), QueueError> {
        let payload = serde_json::to_vec(&operation)?;

        self.channel
            .basic_publish(
                &self.config.exchange_name,
                &self.config.queue_name,
                BasicPublishOptions {
                    mandatory: true,
                    ..BasicPublishOptions::default()
                },
                &payload,
                BasicProperties::default()
                    .with_delivery_mode(2) // persistent
                    .with_content_type("application/json".into())
                    .with_content_encoding("utf-8".into()),
            )
            .await?;

        Ok(())
    }

    async fn start_consuming(&self) -> Result<(), QueueError> {
        let channel = self.channel.clone();
        let processor = self.processor.clone();
        let config = self.config.clone();

        let consumer = channel
            .basic_consume(
                &config.queue_name,
                &config.consumer_tag,
                BasicConsumeOptions {
                    no_ack: false,
                    ..BasicConsumeOptions::default()
                },
                FieldTable::default(),
            )
            .await?;

        let handle = tokio::spawn(async move {
            info!("Starting consumer for {}", processor.operation_type());

            consumer
                .for_each(|delivery| async {
                    let delivery = match delivery {
                        Ok(delivery) => delivery,
                        Err(e) => {
                            error!("Failed to get delivery: {}", e);
                            return;
                        }
                    };

                    let operation: P::Operation = match serde_json::from_slice(&delivery.data) {
                        Ok(op) => op,
                        Err(e) => {
                            error!("Failed to deserialize operation: {}", e);
                            let _ = delivery.reject(BasicRejectOptions { requeue: false }).await;
                            return;
                        }
                    };

                    match processor.process(operation).await {
                        Ok(_) => {
                            if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                                error!("Failed to ack message: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Processing error: {}", e);
                            if let Err(e) =
                                delivery.reject(BasicRejectOptions { requeue: true }).await
                            {
                                error!("Failed to reject message: {}", e);
                            }
                        }
                    }
                })
                .await;
        });

        *self.consumer_handle.lock().await = Some(handle);
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), QueueError> {
        if let Some(handle) = self.consumer_handle.lock().await.take() {
            handle.abort();
        }
        self.channel.close(0, "Shutting down").await?;
        self.connection.close(0, "Shutting down").await?;
        Ok(())
    }
}
