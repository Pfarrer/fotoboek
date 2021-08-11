FROM alpine

WORKDIR /opt/fotoboek
COPY --from=fotoboek /opt/fotoboek/diesel .
COPY --from=fotoboek /opt/fotoboek/fotoboek .
COPY --from=fotoboek /opt/fotoboek/assets/ assets/
COPY --from=fotoboek /opt/fotoboek/migrations/ migrations/
COPY --from=fotoboek /opt/fotoboek/templates/ templates/
COPY .env.sample .env
COPY start.sh .

RUN mkdir /opt/fotoboek-database

CMD ["/bin/sh", "start.sh"]