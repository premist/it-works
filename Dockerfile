FROM clux/muslrust:stable as build
WORKDIR /tmp/it-works
COPY . /tmp/it-works
RUN cargo build --release

FROM alpine:3.6
WORKDIR /root/
COPY --from=build /tmp/it-works/target/x86_64-unknown-linux-musl/release/it-works .

EXPOSE 8080
CMD ["./it-works"]
