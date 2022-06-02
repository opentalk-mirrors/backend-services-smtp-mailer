FROM rust:slim AS builder

WORKDIR /opt
RUN USER=root cargo new --bin smtp-mailer

WORKDIR /opt/smtp-mailer
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm ./src/*.rs
RUN rm ./target/release/deps/smtp-mailer*
ADD ./src ./src
ADD ./migrations ./migrations
RUN cargo build --release


FROM debian:bullseye-slim
WORKDIR /opt/smtp-mailer

COPY --from=builder /opt/smtp-mailer/target/release/smtp-mailer .

CMD ["/opt/smtp-mailer/smtp-mailer"]
