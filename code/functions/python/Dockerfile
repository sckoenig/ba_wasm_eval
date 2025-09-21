FROM python:3.12-slim AS build
ARG FUNCTION
WORKDIR /app
COPY ./bench/bench.py .
COPY ./${FUNCTION}/${FUNCTION}.py .
COPY ./${FUNCTION}/main.py .
COPY ./${FUNCTION}/requirements.txt .
RUN pip install --no-cache-dir --prefix=/app/deps -r requirements.txt

FROM gcr.io/distroless/python3-debian12
COPY --from=build /app /app
ENV PYTHONPATH=/app/deps/lib/python3.12/site-packages

ENTRYPOINT ["python3", "/app/main.py"]