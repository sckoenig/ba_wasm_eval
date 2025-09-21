FROM docker.io/rust:1.85.1 AS builder
ARG FUNCTION
WORKDIR /build
COPY . .
RUN cargo build --release -p ${FUNCTION}

FROM gcr.io/distroless/cc-debian12
ARG FUNCTION
COPY --from=builder /build/target/release/${FUNCTION} /benchmark
ENTRYPOINT ["/benchmark"]