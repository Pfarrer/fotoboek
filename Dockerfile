FROM pfarrer/fotoboek-builder:latest AS rust-builder

WORKDIR /opt/fotoboek
COPY . .
RUN cargo build --release

FROM node:lts AS angular-builder

WORKDIR /opt/webapp
COPY webapp/ .
RUN npm install
RUN npm run build

FROM pfarrer/fotoboek-runtime:latest AS runtime

WORKDIR /opt/fotoboek

COPY --from=rust-builder /opt/fotoboek/target/release/app .
COPY --from=angular-builder /opt/webapp/dist/webapp/ webapp/
COPY .env.sample .env

RUN mkdir /opt/media-source
RUN mkdir /opt/fotoboek-database
RUN mkdir /opt/fotoboek-storage

CMD ["/opt/fotoboek/app"]
