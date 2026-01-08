FROM --platform=$BUILDPLATFORM rust:1.92 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN cargo install dioxus-cli --root /.cargo --locked --force
ENV PATH="/.cargo/bin:$PATH"

RUN dx bundle --web --release

FROM python:3.14-alpine

WORKDIR /srv
COPY --from=builder /app/target/dx/oneprog-main/release/web /srv

EXPOSE 8080
CMD ["python", "-m", "http.server", "8080", "--bind", "0.0.0.0"]
