FROM rust:latest AS builder

WORKDIR /usr/src/frame-server

COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to allow Rust compiler to analyze the workspace if needed
RUN mkdir src

# Build dependencies (create dummy lib.rs to allow compiling dependencies)
RUN echo "// empty" > src/lib.rs

RUN cargo build --release || true

# Remove dummy src file to copy actual source files next
RUN rm -rf src

# Copy source files
COPY . .

RUN cargo build --release

# Stage 2: Create minimal runtime image with the built binary
FROM debian:buster-slim

# Install runtime dependencies for Rust binaries
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from build stage
COPY --from=builder /usr/src/frame-server/target/release/frame-server /usr/local/bin/frame-server

EXPOSE 3001

CMD ["frame-server"]
