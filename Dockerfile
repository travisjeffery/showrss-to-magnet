FROM rust:1.48 as builder

WORKDIR /usr/src/showrss-to-magnet
COPY . .
RUN cargo install --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/showrss-to-magnet .
USER 1000
CMD ["/showrss-to-magnet"]