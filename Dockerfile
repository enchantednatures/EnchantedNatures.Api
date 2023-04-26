# Use the official Rust image as the base
FROM rust:1.59.0 as build

# Create a new empty shell project
RUN USER=root cargo new --bin enchanted_natures
WORKDIR /enchanted_natures

# Copy your project's Cargo.toml and Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source code
COPY ./src ./src

# Build the release binary
RUN rm ./target/release/deps/enchanted_natures*
RUN cargo build --release

# Start a new stage for the runtime image
FROM debian:buster-slim as runtime

# Install necessary runtime libraries for PostgreSQL
RUN apt-get update && \
    apt-get install -y libpq-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the release binary from the build stage
COPY --from=build /enchanted_natures/target/release/enchanted_natures /usr/local/bin

# Expose the port the API will run on
EXPOSE 8080

# Run the API
CMD ["/usr/local/bin/enchanted_natures"]

