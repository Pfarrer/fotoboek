FROM rust:bullseye AS rust-builder

RUN apt-get update \
    && apt-get install -y \
        libsqlite3-dev libopencv-dev \
        llvm-dev clang libclang-dev

WORKDIR /opt/fotoboek
COPY . .
RUN cargo build --release

FROM node:lts AS angular-builder

WORKDIR /opt/webapp
COPY webapp/ .
RUN npm install
RUN npm run build

FROM rust:slim-bullseye as runtime

RUN apt-get update \
    && apt-get install -y \
        libsqlite3-0 libopencv-contrib4.5 \
        libopencv-superres4.5 libopencv-videostab4.5 \
        libopencv-stitching4.5 libopencv-shape4.5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/fotoboek

COPY --from=rust-builder /opt/fotoboek/target/release/app .
COPY --from=angular-builder /opt/webapp/dist/webapp/ webapp/
#COPY migrations/ migrations/
#COPY templates/ templates/
COPY .env.sample .env

RUN mkdir /opt/media-source
RUN mkdir /opt/fotoboek-database
RUN mkdir /opt/fotoboek-storage

CMD ["/opt/fotoboek/app"]
