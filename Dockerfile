FROM rust:1.75 as builder

# Create a new empty shell project
WORKDIR /app
COPY . .

# Build for release
RUN cargo build --release --features prod

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install OpenSSL, ca-certificates and other dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/supabase-axum /app/supabase-axum

# Expose the port
EXPOSE 3000

# Run the binary
CMD ["./supabase-axum"] 