FROM golang:1.24.3 AS build
ARG FUNCTION
WORKDIR /go/src/app
COPY . .
WORKDIR /go/src/app/${FUNCTION}
RUN go mod download
RUN CGO_ENABLED=0 go build -o /go/bin/app

FROM gcr.io/distroless/static-debian12
COPY --from=build /go/bin/app /
ENTRYPOINT ["/app"]