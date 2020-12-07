FROM clux/muslrust as builder
WORKDIR /usr/src
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/showrss-to-magnet /
USER 1000
ENV RUST_LOG=info
ENV DST=/dst
VOLUME ["/dst"]
ENTRYPOINT ["/showrss-to-magnet"]