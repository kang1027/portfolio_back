# Build stage
FROM rust:1.89 as builder

# Set working directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency cache
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src
COPY user_token.txt ./
COPY Rocket.toml ./

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
  ca-certificates \ 
  libssl3 \
  && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/portfolio_back /app/portfolio_back

# Copy configuration files
COPY --from=builder /app/Rocket.toml ./Rocket.toml
COPY --from=builder /app/user_token.txt ./user_token.txt

# Change ownership to app user
RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 1047

CMD ["./portfolio_back"]
