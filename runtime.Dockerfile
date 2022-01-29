FROM rust:slim-bullseye

RUN ln -s /usr/bin/dpkg-split /usr/sbin/dpkg-split
RUN ln -s /usr/bin/dpkg-deb /usr/sbin/dpkg-deb
RUN ln -s /bin/rm /usr/sbin/rm
RUN ln -s /bin/tar /usr/sbin/tar

RUN apt-get update \
    && apt-get install -y \
        libsqlite3-0 libopencv-contrib4.5 \
        libopencv-superres4.5 libopencv-videostab4.5 \
        libopencv-stitching4.5 libopencv-shape4.5 \
        ffmpeg \
    && rm -rf /var/lib/apt/lists/*
