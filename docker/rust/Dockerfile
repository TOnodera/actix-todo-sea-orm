FROM rust:1.69-buster

RUN useradd -m -u 1000 rust -s /bin/bash
RUN rustup component add rustfmt
RUN cargo install cargo-watch sqlx-cli
RUN chown -R rust:rust /usr/local/cargo
USER rust