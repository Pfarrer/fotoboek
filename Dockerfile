FROM ubuntu as builder

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    build-essential curl \
    libsqlite3-dev libopencv-dev \
    llvm-dev clang libclang-dev

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install diesel_cli --no-default-features --features "sqlite-bundled"

WORKDIR /opt/fotoboek
COPY . .
RUN cargo build --release


FROM ubuntu

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y libsqlite3-0 libopencv-contrib4.2

WORKDIR /opt/fotoboek
COPY --from=builder /root/.cargo/bin/diesel .
COPY --from=builder /opt/fotoboek/target/release/fotoboek .
COPY --from=builder /opt/fotoboek/assets/ assets/
COPY --from=builder /opt/fotoboek/migrations/ migrations/
COPY --from=builder /opt/fotoboek/templates/ templates/
COPY .env.sample .env
COPY start.sh .

RUN mkdir /opt/fotoboek-database

CMD ["/bin/sh", "start.sh"]