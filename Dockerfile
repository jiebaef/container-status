FROM rust:1.78

# RUN USER=container-status cargo new --bin container-status
RUN USER=container-status mkdir container-status
WORKDIR /home/container-status/container-status

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src
RUN echo "fn main() { }" >> ./src/main.rs

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./static ./static/
COPY ./templates ./templates/

RUN rm -rf ./target/release/deps*
RUN cargo install --path .

# CMD ["./target/release/container-status"]
CMD ["container-status"]

EXPOSE 42069
