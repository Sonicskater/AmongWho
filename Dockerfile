FROM rust:1.46 as builder

RUN USER=root
ADD . ./
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/AmongWho ./

CMD ["./AmongWho"]