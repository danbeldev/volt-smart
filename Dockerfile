# Используем минимальный образ для запуска скомпилированного бинарника
FROM debian:bullseye-slim

# Устанавливаем зависимости для работы приложения (например, сертификаты)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Копируем скомпилированный бинарник в контейнер
COPY target/release/volt-smart /usr/local/bin/volt-smart

# Копируем .env файл
COPY .env /app/.env

# Устанавливаем рабочую директорию
WORKDIR /app

# Запускаем приложение
CMD ["/usr/local/bin/volt-smart"]