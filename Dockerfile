FROM rust:1.45 AS build

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/passthrough-dns
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .


FROM scratch AS runtime

COPY --from=build /usr/local/cargo/bin/passthrough-dns .

USER 1000
EXPOSE 5553

CMD [ "./passthrough-dns" ]
