FROM docker.io/rust:1-slim-bookworm AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM docker.io/debian:bookworm-slim
LABEL org.opencontainers.image.authors="Johann Queuniet"
LABEL org.opencontainers.image.source="https://github.com/jqueuniet/ecowitt_exporter"
LABEL org.opencontainers.image.description="Republish metrics sent with the Ecowitt weather station protocol to the prometheus format "
LABEL org.opencontainers.image.licenses="AGPL"

ENV ROCKET_ADDRESS=0.0.0.0

COPY --from=builder /usr/local/cargo/bin/ecowitt_exporter /usr/local/bin/

RUN adduser \
    --quiet \
    --system \
    --no-create-home \
    --group \
    --disabled-password \
    exporter

USER exporter

ENTRYPOINT ["ecowitt_exporter"]