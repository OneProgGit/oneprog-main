FROM rust:1.92 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

RUN dx bundle --web --release

FROM python:3.13-alpine

WORKDIR /srv
COPY --from=builder /app/target/dx/oneprog-main/release/web /srv

EXPOSE 8080
CMD ["python", "-m", "http.server", "8080", "--bind", "127.0.0.1"]