FROM python:3.12-bookworm

RUN apt-get update \
    && apt-get install -y clang \
    && adduser developer --disabled-password --gecos ""

USER developer

RUN cd $HOME \
    && git clone https://github.com/antoonpurnal/clangover.git \
    && cd clangover \
    && git submodule init && git submodule update \
    && sed -i 's/-O3/-Os/g' kyber/ref/Makefile \
    && CC=clang make -C kyber/ref shared \
    && make

WORKDIR /home/developer/clangover

COPY --chmod=777 entrypoint.sh /home/developer/clangover/entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]
