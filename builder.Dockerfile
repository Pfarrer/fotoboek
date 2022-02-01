FROM rust:bullseye

RUN ln -s /usr/bin/dpkg-split /usr/sbin/dpkg-split
RUN ln -s /usr/bin/dpkg-deb /usr/sbin/dpkg-deb
RUN ln -s /bin/rm /usr/sbin/rm
RUN ln -s /bin/tar /usr/sbin/tar

RUN apt-get update \
    && apt-get install -y \
        libsqlite3-dev libopencv-dev \
        llvm-dev clang libclang-dev

RUN cargo install cargo-chef
