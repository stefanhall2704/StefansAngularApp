FROM rustlang/rust:nightly as builder

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy the manifests and Diesel configuration
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./diesel.toml ./diesel.toml

# Copy source code
COPY ./src ./src

# Build for release
RUN cargo +nightly build --release

# Final base image
FROM debian:buster-slim

# Install SQLite library
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage and SQLite DB
COPY --from=builder /app/target/release/my-binary .
COPY ./house.sqlite ./house.sqlite

ENV DATABASE_URL=./house.sqlite

# Set the startup command
CMD ["./my-binary"]