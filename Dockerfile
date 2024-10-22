# Use the official Ubuntu 20.04 image
FROM --platform=linux/amd64 ubuntu:20.04

# Install necessary tools
RUN apt-get update && \
    apt-get install -y nasm gcc python3 vim git curl && \
    apt-get clean

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="$HOME/.cargo/bin:$PATH"

# Set the working directory
WORKDIR /app

# Start a shell
CMD ["/bin/bash"]
