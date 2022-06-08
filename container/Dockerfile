FROM rust:slim AS builder

WORKDIR /opt
RUN USER=root cargo new --bin smtp-mailer

WORKDIR /opt/smtp-mailer
COPY ./.cargo/config ./.cargo/config
COPY ./vendored ./vendored
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release --locked
RUN rm ./src/*.rs
RUN rm ./target/release/deps/smtp-mailer*
ADD ./src ./src
ADD ./migrations ./migrations
RUN cargo build --release --locked


FROM debian:bullseye-slim
WORKDIR /opt/smtp-mailer
COPY /opt/smtp-mailer/resources ./resources
COPY --from=builder /opt/smtp-mailer/target/release/smtp-mailer .

CMD ["/opt/smtp-mailer/smtp-mailer"]
