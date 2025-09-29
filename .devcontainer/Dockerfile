# Use the latest Ubuntu image as the base image
FROM --platform=linux/amd64 ubuntu:latest

# Install build tools, dasm, ARM toolchain, and utilities
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    gcc-arm-none-eabi \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for development
RUN useradd -m -s /bin/bash dev \
    && usermod -aG sudo dev \
    && echo "dev ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Switch to the development user
USER dev
WORKDIR /home/dev

# Install Rust using rustup and add ARM Cortex-M0 target
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . ~/.cargo/env \
    && rustup target add thumbv6m-none-eabi