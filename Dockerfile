FROM rust:bullseye as builder

RUN apt-get update \
    && apt-get install -y \
        libsqlite3-dev libopencv-dev \
        llvm-dev clang libclang-dev

RUN cargo install diesel_cli \
    --no-default-features \
    --features "sqlite"

WORKDIR /opt/fotoboek
COPY . .
RUN cargo install --path .


FROM rust:slim-bullseye

RUN apt-get update \
    && apt-get install -y \
        libsqlite3-0 libopencv-contrib4.5 \
        libopencv-superres4.5 libopencv-videostab4.5 \
        libopencv-stitching4.5 libopencv-shape4.5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/fotoboek

COPY --from=builder /usr/local/cargo/bin/diesel .
COPY --from=builder /usr/local/cargo/bin/fotoboek .
COPY assets/ assets/
COPY migrations/ migrations/
COPY templates/ templates/
COPY .env.sample .env
COPY start.sh .

RUN mkdir /opt/fotoboek-database

CMD ["/bin/sh", "start.sh"]
