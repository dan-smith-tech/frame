# Stage 1: Build the Rust binary
FROM rust:latest AS builder

# Set working directory inside the container
WORKDIR /usr/src/frame-server

# Copy the Cargo.toml and Cargo.lock first to cache dependencies if unchanged
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to allow Rust compiler to analyze the workspace if needed
RUN mkdir src

# Build dependencies (create dummy lib.rs to allow compiling dependencies)
RUN echo "// empty" > src/lib.rs

# Fetching dependencies to cache
RUN cargo build --release || true

# Remove dummy src file to copy actual source files next
RUN rm -rf src

# Copy source files
COPY . .

# Build release optimized binary
RUN cargo build --release

# Stage 2: Create minimal runtime image with the built binary
FROM debian:buster-slim

# Install runtime dependencies for Rust binaries
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from build stage
COPY --from=builder /usr/src/frame-server/target/release/frame-server /usr/local/bin/frame-server

# Expose the port your app listens on (here 3001 per your Rust code)
EXPOSE 3001

# Default command to run the app
CMD ["frame-server"]

