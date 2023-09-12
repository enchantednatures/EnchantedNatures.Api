# Use the official Rust image as the base
FROM rust:1.69.0 as build


# Create a new empty shell project
RUN USER=root cargo new --bin api
WORKDIR /api

# Copy your project's Cargo.toml and Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source code

COPY ./.sqlx ./.sqlx
COPY ./src ./src
COPY ./sql ./sql
COPY ./migrations ./migrations

# Build the release binary
RUN rm ./target/release/deps/api*
ENV SQLX_OFFLINE true
ENV RUST_LOG=info,axum::rejection=trace 
RUN cargo install --path .

# Start a new stage for the runtime image
FROM debian:bullseye-slim as runtime

# Install necessary runtime libraries for PostgreSQL
RUN apt-get update && \
    apt-get install -y libpq-dev ca-certificates  && \
    rm -rf /var/lib/apt/lists/*

# Copy the release binary from the build stage
COPY --from=build /usr/local/cargo/bin/api /usr/local/bin/api
COPY ./api ./api

# Expose the port the API will run on
EXPOSE 6969
# Run the API

COPY ./config ./config
ENV APP_ENVIRONMENT production
ENV RUST_LOG=info,axum::rejection=trace 
CMD ["api"]

