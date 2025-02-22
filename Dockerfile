# Используем официальный Rust-образ
FROM rust:1.85.0 AS builder

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем Cargo-файлы отдельно, чтобы использовать кэширование
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Копируем исходный код
COPY . .

# Собираем финальный бинарник
RUN cargo build --release

# Создаем минимальный образ с Debian
FROM debian:bullseye-slim

# Устанавливаем зависимости для работы Rust-приложений
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Копируем скомпилированное приложение
COPY --from=builder /app/target/release/monitor_battery /usr/local/bin/monitor_battery

# Копируем .env файл
COPY .env /app/.env

# Устанавливаем рабочую директорию
WORKDIR /app

# Запускаем приложение
CMD ["/usr/local/bin/monitor_battery"]
