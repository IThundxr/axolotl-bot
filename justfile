set dotenv-load := true

[group("docker")]
docker-up:
    docker compose up -d

[group("build")]
[working-directory: "crates/website/frontend"]
build-frontend:
    trunk build --release

[group("run")]
run-app: docker-up build-frontend
    cargo run -p app

[group("run")]
run-backend: docker-up
    cargo run -p website-backend

[group("run")]
[working-directory: "crates/website/frontend"]
run-frontend:
    trunk serve --open

[group("generate")]
[working-directory: "crates/database/entity"]
generate-entities:
    sea-orm-cli generate entity -o src --lib --with-serde=both --entity-format=dense