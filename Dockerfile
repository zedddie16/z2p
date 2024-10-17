FROM rust:1.81.0


WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIROMENT production
ENTRYPOINT ["./target/release/zero2prod"]
