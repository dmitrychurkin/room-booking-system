migrate_info:
    sqlx migrate info --source="./src/migrations" --database-url="postgres://postgres:admin@localhost:5433/room-booking-system"

migrate_run:
    sqlx migrate run --source="./src/migrations" --database-url="postgres://postgres:admin@localhost:5433/room-booking-system"