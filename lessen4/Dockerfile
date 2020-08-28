FROM alpine:3.12
RUN apk upgrade \
  && apk add --no-cache \
    bash \
    curl \
    -u busybox \
    busybox-extras \
    rust \
    cargo
CMD [ "rustc --version" ]