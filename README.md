# AICBM — AI City Builder in Minecraft

Выделить область → дать свободное задание → получить спроектированный и построенный район.

Актуальные границы MVP и этапы: [ТЗ и дорожная карта](docs/TZ_ROADMAP.md).
Предыдущее описание долгосрочной архитектуры сохранено в [ARCHITECTURE_VISION.md](docs/ARCHITECTURE_VISION.md).

## Реализовано: этап 0

Автономное Rust-ядро и CLI структурной проверки JSON-планов. Проверяются версия схемы, прямоугольные границы, диапазон высот, уникальность и непустые ID, пересечения зданий и защищённые зоны. Координаты XZ используют полуоткрытые интервалы `[min,max)`. Расчёт верхней границы здания защищён от переполнения.

Это ещё не проверка пригодности к строительству: дороги, входы, рельеф, вода, материалы и операции с блоками добавляются на следующих этапах. Подключение к Minecraft и LLM пока отсутствует.

## Запуск

Нужен Rust toolchain с Cargo. Первая сборка скачивает зависимости из crates.io; Cargo.lock фиксирует их версии.

```sh
cargo test --locked
cargo run --locked -- validate examples/plan.json
cargo run --locked -- validate examples/plan.json --json
cargo run --locked -- validate examples/invalid-plan.json --json
```

Последний пример намеренно завершается с кодом 1. Коды завершения: 0 — план прошёл структурные проверки; 1 — ошибки плана; 2 — ошибка аргументов, чтения файла или JSON. `--help` выводит справку.

JSON-отчёт содержит `scope: structural`, `status: valid | invalid | input_error`, `errors: string[]`.

## Docker

Нужен Docker с Linux-контейнерами. Сборка запускает тесты и создаёт release-бинарник; итоговый образ содержит только приложение и примеры, работает без root.

```sh
docker compose build
docker compose run --rm aicbm
docker compose run --rm aicbm validate examples/invalid-plan.json --json
```

Для проверки собственного файла смонтируйте его в контейнер только для чтения (замените абсолютный путь):

```sh
docker run --rm --network none --read-only --mount type=bind,source=/absolute/path/plan.json,target=/app/input.json,readonly aicbm:local validate /app/input.json --json
```

CLI завершается после проверки и возвращает те же коды 0/1/2. Это контейнер валидатора, постоянный сервер пока отсутствует.

## Структура проекта

- `src/lib.rs` — JSON-контракт и валидатор.
- `src/main.rs` — CLI и отчёты.
- `examples/` — корректный и ошибочный планы.
- `tests/cli.rs` — интеграционные проверки команд и кодов завершения.
- `docs/TZ_ROADMAP.md` — предоставленное ТЗ.

## Следующий этап

Выбрать версию Minecraft Java и платформу сервера, затем реализовать Java bridge: выделение области, снимок мира, тестовый пакет блоков, минимальные проверки повторов и отмены. Выбор платформы пока открыт в ТЗ. До полной проверки плана и операций реальное строительство не реализовано.
