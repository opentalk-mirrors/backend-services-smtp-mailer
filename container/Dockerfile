# SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
#
# SPDX-License-Identifier: EUPL-1.2

FROM rust:1.73.0-slim-bullseye AS builder

ARG GIT_SHA
ARG GIT_REF
ARG VERGEN_GIT_SHA=$GIT_SHA
ARG VERGEN_GIT_BRANCH=$GIT_REF

WORKDIR /opt
RUN USER=root cargo new --bin smtp-mailer --vcs none
RUN apt-get update && apt-get install -y git

WORKDIR /opt/smtp-mailer
COPY ./.cargo/config ./.cargo/config
COPY ./vendored ./vendored
COPY ./resources ./resources
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD ./build.rs ./build.rs
RUN cargo build --release --locked
RUN rm ./src/*.rs
RUN rm ./target/release/deps/smtp_mailer*
ADD ./src ./src
RUN cargo build --release --locked


FROM debian:bullseye-slim
WORKDIR /opt/smtp-mailer
COPY ./resources ./resources
COPY --from=builder /opt/smtp-mailer/target/release/smtp-mailer .

CMD ["/opt/smtp-mailer/smtp-mailer"]
