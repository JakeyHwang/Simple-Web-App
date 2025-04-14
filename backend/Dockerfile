# Build stage
FROM rust:1.81-slim as builder

WORKDIR /usr/src/app

# Install build dependencies and musl target
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rm -rf /var/lib/apt/lists/* && \
    rustup target add x86_64-unknown-linux-musl

# Copy manifests
COPY Cargo.toml ./
COPY .env ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl

# Copy the actual source code
COPY . .

# Set SQLX_OFFLINE to true for the build
ENV SQLX_OFFLINE=true

# Build the application
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:latest

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/backend ./

# Copy .env file
COPY --from=builder /usr/src/app/.env ./

# Make the binary executable
RUN chmod +x ./backend

# Expose the port the app runs on
EXPOSE 8080

# Command to run the application
CMD ["./backend"] 