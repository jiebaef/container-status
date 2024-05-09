FROM rust:1.78

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/container-status"]

EXPOSE 42069
