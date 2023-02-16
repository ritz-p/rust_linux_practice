FROM rust:latest

RUN apt update && \
    apt install -y build-essential

RUN echo "alias gcc='gcc -Wall'" > ~/.bashrc
WORKDIR /workspace