# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Set the environment variable
ENV DATABASE_URL=postgresql://db_user:xUCuiMFpwr8jfinVWD6vU344vC5TcTtc@dpg-cr47r6jv2p9s73cnlnqg-a/hyperswitch_db

# Install necessary Rust tools and run migration
RUN cargo install diesel_cli --no-default-features --features postgres && \
    cargo install just

# Set the default command to run migrations
CMD ["bash", "-c", "just migrate"]