#!/usr/bin/env bash

# Install cargo watch
cargo install cargo-watch
cargo install diesel_cli --no-default-features --features postgres

echo "export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}" >> ~/.bashrc

source ~/.bashrc
