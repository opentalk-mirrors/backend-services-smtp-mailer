# SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
#
# SPDX-License-Identifier: EUPL-1.2

FROM git.opentalk.dev:5050/opentalk/backend/containers/rust:1.83.0-bookworm AS builder

ARG GIT_SHA
ARG GIT_REF
ARG VERGEN_GIT_SHA=$GIT_SHA
ARG VERGEN_GIT_BRANCH=$GIT_REF

WORKDIR /opt
RUN USER=root cargo new --bin smtp-mailer --vcs none
RUN apt-get update && apt-get install -y git

WORKDIR /opt/smtp-mailer
COPY ./.cargo/config.toml ./.cargo/config.toml
COPY ./resources ./resources
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD ./build.rs ./build.rs
RUN cargo auditable build --release --locked
RUN rm ./src/*.rs
RUN rm ./target/release/deps/smtp_mailer*
ADD ./src ./src
RUN cargo auditable build --release --locked


FROM debian:bookworm-slim
WORKDIR /opt/smtp-mailer
COPY ./resources ./resources
COPY --from=builder /opt/smtp-mailer/target/release/smtp-mailer .

CMD ["/opt/smtp-mailer/smtp-mailer"]
