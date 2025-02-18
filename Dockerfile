ARG RUST_VERSION=1.84.1
ARG APP_NAME=rust-fe-template

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev ca-certificates upx

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    upx --lzma --best ./target/release/$APP_NAME && \
    cp ./target/release/$APP_NAME /bin/server

######

FROM scratch AS final
COPY --from=build /bin/server /bin/

ENV APP_NAME_VERSION=rust-fe-template-0.1.0

CMD ["/bin/server"]
