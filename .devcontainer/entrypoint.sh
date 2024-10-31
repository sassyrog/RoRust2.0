#!/usr/bin/env bash

# Install cargo watch
cargo install watchexec-cli
# cargo install sccache --force
cargo install diesel_cli --no-default-features --features postgres



apt-get update && \
    apt-get install -y \
    inotify-tools \
    && rm -rf /var/lib/apt/lists/*

sudo echo "fs.inotify.max_user_watches=524288" >> /etc/sysctl.conf



echo "export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}" >> ~/.bashrc
echo "export AMQP_URL=amqp://${RABBITMQ_USER}:${RABBITMQ_PASSWORD}@${RABBITMQ_HOST}:${RABBITMQ_PORT}" >> ~/.bashrc
# echo "export CARGO_INCREMENTAL=1" >> ~/.bashrc
# echo "export RUSTC_WRAPPER=sccache" >> ~/.bashrc

source ~/.bashrc
