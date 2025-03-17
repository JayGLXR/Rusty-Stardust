FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    mingw-w64 \
    nasm \
    binutils-mingw-w64 \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set up Rust targets for cross-compilation
RUN rustup target add x86_64-pc-windows-gnu i686-pc-windows-gnu

# Create working directory
WORKDIR /stardust

# Set default command
CMD ["bash"]