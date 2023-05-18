FROM rust:1.69-buster

RUN apt update -y && apt install -y sqlite3 locales locales-all
RUN useradd -m -u 1000 rust -s /bin/bash
RUN rustup component add rustfmt
RUN cargo install cargo-watch
RUN cargo install sea-orm-cli --version 0.11.0 
RUN chown -R rust:rust /usr/local/cargo

ENV LC_ALL ja_JP.UTF-8
ENV LANG ja_JP.UTF-8
ENV LANGUAGE ja_JP.UTF-8
USER rust