
LABEL maintainer="tacolopez1618033@outlook.com"
LABEL version="1.0"
LABEL description="Sistema de inventario en Rust"


# Etapa 1: Compilación
FROM rust:latest AS builder
WORKDIR ./sis-inventario-rust
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Etapa 2: Imagen final
FROM debian:bullseye-slim

# INSTALAR CERTIFICADOS: (Aun no los tenemos implementados en la version 1.0) 
#RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /sis-inventario-rust

# Copia el binario
COPY --from=builder /sis-inventario-rust/target/release/inventario ./inventario

# Copia la base de datos (independiente del binario)
COPY cafeteria ./cafeteria

CMD ["./inventario", "server"]

