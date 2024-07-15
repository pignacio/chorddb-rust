# bookworm images are more secure compared to alpine
FROM rust:1.79 AS build

WORKDIR /app

COPY . ./

RUN cargo build --release --package chorddb && cargo build --release --package migration

FROM rust:1.79

WORKDIR /app

COPY --from=build /app/target/release/chorddb ./
COPY --from=build /app/target/release/migration ./
COPY migrate_and_run.sh ./

EXPOSE 8080

CMD ["./migrate_and_run.sh"]
