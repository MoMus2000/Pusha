# Use the official Ubuntu 20.04 image
FROM --platform=linux/amd64 ubuntu:20.04

# Install necessary tools
RUN apt-get update && \
    apt-get install -y nasm gcc python3 vim git && \
    apt-get clean

# Set the working directory
WORKDIR /app

# Start a shell
CMD ["/bin/bash"]
