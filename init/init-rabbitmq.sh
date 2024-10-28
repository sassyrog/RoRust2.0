#!/bin/bash

set -e

# Wait for RabbitMQ to be ready
until rabbitmqctl status >/dev/null 2>&1; do
    echo "Waiting for RabbitMQ to start..."
    sleep 2
done

# Make sure we have admin access
rabbitmqctl await_startup

# Create vhost if it doesn't exist
rabbitmqctl add_vhost / || true

# Create user
rabbitmqctl add_user $RABBITMQ_USER $RABBITMQ_PASSWORD

# Set user tags
rabbitmqctl set_user_tags $RABBITMQ_USER administrator

# Set permissions
rabbitmqctl set_permissions -p / $RABBITMQ_USER ".*" ".*" ".*"

echo "RabbitMQ user and permissions configured"