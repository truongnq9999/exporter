FROM rust:1.67.1-slim AS builder

COPY . /app

WORKDIR /app

RUN cargo build --release

FROM ubuntu:22.04

RUN  apt-get update && apt-get install -y fontconfig fontconfig-config fonts-dejavu-core libbsd0 \
     libexpat1 libfontconfig1 libfontenc1 libfreetype6 libjpeg-turbo8 libmd0 libpng16-16 libx11-6 \
     libx11-data libxau6 libxcb1 libxdmcp6 libxext6 libxrender1 ucf x11-common xfonts-75dpi xfonts-base \
     xfonts-encodings xfonts-utils wget && cd /tmp && \
     wget https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6.1-2/wkhtmltox_0.12.6.1-2.jammy_amd64.deb && \
     dpkg -i wkhtmltox_0.12.6.1-2.jammy_amd64.deb && rm wkhtmltox_0.12.6.1-2.jammy_amd64.deb

COPY --from=builder /app/application.yaml /
COPY --from=builder /app/target/release/exporter /app

CMD ["/app"]

