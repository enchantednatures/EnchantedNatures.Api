# Use the official Rust image as the base
FROM rust:1.73.0 as build


# Create a new empty shell project

WORKDIR /enchantednatures
RUN USER=root cargo new --bin api
# WORKDIR /enchantednatures/api

# Copy your project's Cargo.toml and Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./api/Cargo.toml ./api/Cargo.toml

# Cache dependencies
RUN cargo build --release
RUN rm api/src/*.rs

# Copy your source code

COPY ./.sqlx ./.sqlx
COPY ./api/src ./api/src
COPY ./api/migrations ./api/migrations

# Build the release binary
RUN rm ./target/release/deps/api*
ENV SQLX_OFFLINE true
ENV RUST_LOG=info,axum::rejection=trace 

RUN cargo install --path api
# Start a new stage for the runtime image
FROM debian:bullseye-slim as runtime

# Install necessary runtime libraries for PostgreSQL
RUN apt-get update && \
    apt-get install -y libpq-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the release binary from the build stage
COPY --from=build /usr/local/cargo/bin/api /usr/local/bin/api
COPY ./specs ./specs

# Expose the port the API will run on
EXPOSE 6969
# Run the API

COPY ./config ./config
ENV ENVIRONMENT production
ENV RUST_LOG=info,axum::rejection=trace 
CMD ["api"]

