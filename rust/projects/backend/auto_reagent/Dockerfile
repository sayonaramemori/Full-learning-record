# Stage 1: Build the Rust project with musl for static linking
FROM rust:latest as builder

# Install musl-tools and OpenSSL libraries for musl
RUN apt-get update && apt-get install -y musl-tools 

# Add the musl target for Rust
RUN rustup target add x86_64-unknown-linux-musl

# Create a new directory for the application code
WORKDIR /app

# Cache the Cargo.toml and Cargo.lock files (if you have a Cargo.lock)
# Only copy the Cargo.toml and Cargo.lock first, to cache dependencies
COPY Cargo.toml ./
COPY auto_reagent ./auto_reagent

# Build the dependencies
RUN cargo build --bin backend --release --target x86_64-unknown-linux-musl

# Stage 3: Use a minimal image with OpenSSL installed
FROM alpine:latest

# Install OpenSSL and CA certificates for TLS support
RUN apk --no-cache add ca-certificates 

# Set environment variables for your application
ENV APP_HOME=/app

# Create the working directory
WORKDIR $APP_HOME

# Copy the statically compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend .
COPY .env .

# Expose the necessary port (if needed)
EXPOSE 8080

# Start the application
CMD ["./backend"]

