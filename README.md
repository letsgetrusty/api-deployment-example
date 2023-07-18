# API Deployment Example

## Setup & Run Locally

1. Make sure [Docker](https://www.docker.com/) is installed and running
2. Make sure [sqlx cli](https://crates.io/crates/sqlx-cli) is installed
3. Create a `.env` file. This file will store environment variables. Specifically, `DATABASE_URL` and `POSTGRES_PASSWORD`. It should look like this:
    ```
    DATABASE_URL=postgres://postgres:postgrespw@localhost:5432
    POSTGRES_PASSWORD=postgrespw
    ```
    `NOTE:` When deploying the API, make sure to change the default PostgreSQL password.