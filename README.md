# chen-api

## Установка зависимостей

Chen-api требует

* rust 1.69.0 и выше. Инструкция по установке [здесь](https://www.rust-lang.org/tools/install)
* Docker, инструкцию по установке которого можно найти [здесь](https://docs.docker.com/install/linux/docker-ce/ubuntu/#install-docker-engine---community-1).

Скачиваем утилиту для миграций

```bash
$ cargo install sea-orm-cli
```

## Подготовка к запуску и запуск

Создайте файл `.env` со следующим содержимым:

```dotenv
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
DB_NAME=actix
DB_HOST=127.0.0.1
DB_PORT=5432
DB_USER=actix
DB_PASSWORD=actix

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
```

запускаем docker compose с базой postgres

```bash
$ cd docker && docker compose up -d
```

Накатываем миграции

```bash
$ sea-orm-cli migrate up
```

Запускаем проект

```bash
$ cargo run
```
