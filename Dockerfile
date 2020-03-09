FROM rust:1.41.1

WORKDIR /src/
COPY . .
CMD ["cargo", "run"]