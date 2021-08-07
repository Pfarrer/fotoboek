FROM ubuntu as builder

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update
RUN apt-get install -y build-essential curl libsqlite3-dev libopencv-dev
    # llvm llvm-dev libclang-dev clang-tools clang libc++-dev \
    # sqlite3 libsqlite3-dev libopencv-dev

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get install -y llvm-dev clang libclang-dev
RUN cargo build --release

FROM alpine:3.14
RUN apk add --no-cache sqlite opencv

COPY --from=builder /usr/local/cargo/bin/fotoboek /usr/local/bin/fotoboek
COPY ./.env .env
CMD ["fotoboek"]