FROM pfarrer/fotoboek-builder:lastest AS rust-builder

RUN ln -s /usr/bin/dpkg-split /usr/sbin/dpkg-split
RUN ln -s /usr/bin/dpkg-deb /usr/sbin/dpkg-deb
RUN ln -s /bin/rm /usr/sbin/rm
RUN ln -s /bin/tar /usr/sbin/tar

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

FROM pfarrer/fotoboek-runtime:lastest AS runtime

RUN ln -s /usr/bin/dpkg-split /usr/sbin/dpkg-split
RUN ln -s /usr/bin/dpkg-deb /usr/sbin/dpkg-deb
RUN ln -s /bin/rm /usr/sbin/rm
RUN ln -s /bin/tar /usr/sbin/tar

RUN apt-get update \
    && apt-get install -y \
        libsqlite3-0 libopencv-contrib4.5 \
        libopencv-superres4.5 libopencv-videostab4.5 \
        libopencv-stitching4.5 libopencv-shape4.5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/fotoboek

COPY --from=rust-builder /opt/fotoboek/target/release/app .
COPY --from=angular-builder /opt/webapp/dist/webapp/ webapp/
COPY .env.sample .env

RUN mkdir /opt/media-source
RUN mkdir /opt/fotoboek-database
RUN mkdir /opt/fotoboek-storage

CMD ["/opt/fotoboek/app"]
