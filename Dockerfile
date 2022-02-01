FROM pfarrer/fotoboek-builder:latest AS rust-chef

WORKDIR /opt/fotoboek


FROM rust-chef AS rust-planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM rust-chef AS rust-builder

COPY --from=rust-planner /opt/fotoboek/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
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
