# API Deployment Example

## Setup

1. Make sure [Docker](https://www.docker.com/) is installed and running
2. Make sure [sqlx cli](https://crates.io/crates/sqlx-cli) is installed
3. Create a `.env` file. This file will store environment variables. Specifically, `DATABASE_URL` and `POSTGRES_PASSWORD`. It should look like this:
    ```
    DATABASE_URL=postgres://postgres:postgrespw@localhost:5432
    POSTGRES_PASSWORD=postgrespw
    ```
    `NOTE:` When deploying the API, make sure to change the default PostgreSQL password.
4. Update `docker-compose.yml` and change `letsgetrusty` to your own Docker Hub username. 

## Run Locally
1. Run an instance of PostgreSQL. This can be done via Docker:
    ```bash
    docker pull postgres
    docker run --name example-db -e POSTGRES_PASSWORD=postgrespw -p 5432:5432 -d postgres
    ```
2. Run SQL migrations:
    ```bash
    sqlx migrate run
    ```
3. Start server:
    ```bash
    cargo run
    ```
4. Test routes. I like to use [Postman](https://www.postman.com/).

## Run Locally using Docker
1. Run API via Docker Compose:
    ```bash
    docker-compose up
    ```
2. Test routes. I like to use [Postman](https://www.postman.com/).