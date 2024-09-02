FROM rust:1-slim-bullseye as build

RUN apt-get update && apt-get install -y build-essential curl openssl ca-certificates pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app

RUN USER=root cargo init api

WORKDIR /usr/app/api

COPY ./Cargo.toml ./Cargo.toml

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source code

COPY . . 
# Build the release binary
RUN rm ./target/release/deps/api*
ENV SQLX_OFFLINE true
ENV RUST_LOG=info,axum::rejection=trace 

RUN cargo install --path .


FROM debian:bullseye-slim AS runtime

# Install necessary runtime libraries for PostgreSQL

RUN apt-get update && \
    apt-get install -y libpq-dev libssl1.1 openssl ca-certificates && \
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
