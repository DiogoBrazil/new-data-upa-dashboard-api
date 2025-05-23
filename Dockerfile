# Estágio de build
FROM rust:1.84.1 as builder
WORKDIR /app

# Habilitar modo offline do SQLx
ENV SQLX_OFFLINE=true

COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /usr/local/bin

# Instalar dependências necessárias
RUN apt-get update && apt-get install -y \
    libpq-dev \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copia o binário compilado do estágio de build
COPY --from=builder /app/target/release/new-data-upa-dashboard-api .

# Copia o arquivo .env para o container
COPY --from=builder /app/.env .

# Defini as permissões de execução
RUN chmod +x ./new-data-upa-dashboard-api

# Comando para iniciar a aplicação
CMD ["./new-data-upa-dashboard-api"]