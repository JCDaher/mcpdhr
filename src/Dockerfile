# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy source
COPY Cargo.toml .
COPY src/ ./src/

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \\
    ca-certificates \\
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/mcpdhr /app/mcpdhr

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \\
    CMD curl -f http://localhost:${PORT:-8080}/health || exit 1

# Expose port
EXPOSE ${PORT:-8080}

# Run
CMD ["/app/mcpdhr"]
