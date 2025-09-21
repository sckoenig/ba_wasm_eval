FROM node:22 AS builder
ARG FUNCTION
WORKDIR /app
COPY ./bench ./bench
COPY ./${FUNCTION} ./${FUNCTION}
WORKDIR ./${FUNCTION}
RUN npm ci
RUN npm run build:main

FROM gcr.io/distroless/nodejs22-debian12
ARG FUNCTION
COPY --from=builder /app/${FUNCTION}/bundle/main_bundle.mjs /app/main_bundle.mjs
WORKDIR /app
ENTRYPOINT ["/nodejs/bin/node", "./main_bundle.mjs"]