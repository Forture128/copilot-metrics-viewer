# ===== Builder Stage =====
FROM rust:1.82.0-slim as builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
RUN cargo new --bin horus-be

# Copy manifests
COPY Cargo.lock Cargo.toml ./horus-be/

# Set the working directory
WORKDIR /usr/src/app/horus-be

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY src ./src

# Build for release with optimizations
RUN cargo build --profile release
RUN rm src/*.rs

# ===== Final Stage =====
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    libssl3 \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the build artifact and configurations
COPY --from=builder /usr/src/app/horus-be/target/release/horus-be ./
COPY environment /app/environment/

# Expose the port
EXPOSE 3000

# Command to run the executable
CMD ["./horus-be"]