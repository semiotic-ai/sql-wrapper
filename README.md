# sql-wrapper

This is a thin layer between Clickhouse and clients that aims to allow safe share of a SQL endpoint.

## Routes available

| endpoint | method | params | output                   |   |
|----------|--------|--------|--------------------------|---|
| `/execute` | GET    | sql    | `{ "field": "value" }` |   |


## Environment Variables

You can use the following environment variables or set them using a `.env` file.

```
CLICKHOUSE_URL="tcp://<user>:<password>@<host>:<port>/<database>?secure=true&skip_verify=true&connection_timeout=20s"
```

## Contributing


### Prerequisites
- Rust and Cargo
- Docker and docker-compose (if you want a local clickhouse)


### Running a local Clickhouse

```
docker compose up -d
```

It will be available using: `tcp://default:default@localhost:9000/default`
