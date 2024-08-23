# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Install necessary Rust tools and run migration
RUN cargo install diesel_cli --no-default-features --features postgres && \
    cargo install just

# Set the default command to run migrations
CMD ["bash", "-c", "just migrate"]