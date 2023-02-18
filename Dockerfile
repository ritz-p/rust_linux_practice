FROM rust:latest

RUN apt update && \
    apt install -y build-essential

# add commands created by rust
RUN apt install -y exa bat hexyl fd-find ripgrep && \
    cargo install procs
RUN echo "alias gcc='gcc -Wall'" > ~/.bashrc && \
    echo "alias fd=/usr/lib/cargo/bin/fd" > ~/.bashrc && \
    mkdir -p ~/.local/bin && \
    ln -s /usr/bin/batcat ~/.local/bin/bat
WORKDIR /workspace