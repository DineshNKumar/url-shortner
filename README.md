# URL Shortener

A small URL-shortening API built with Rust, Actix Web, and PostgreSQL.

The service accepts a long URL, generates a Base62 short code, stores it in PostgreSQL, and returns the short URL. Stored URLs are assigned an expiration time 30 days after creation.

## Requirements

- Rust and Cargo
- PostgreSQL

## Setup

1. Create the database table:

```sql
CREATE TABLE urls (
    id BIGSERIAL PRIMARY KEY,
    original_url TEXT NOT NULL,
    short_code TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL
);
```

2. Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://username:password@localhost:5432/url_shortener
PORT=8080
BASE_URL=http://localhost:8080
```

`DATABASE_URL` is required. `PORT` defaults to `8080`, and `BASE_URL` defaults to `http://localhost:8080`.

3. Start the server:

```bash
cargo run
```

The API will listen on `http://localhost:8080` by default.

## API

### Create a short URL

`POST /urls`

Request:

```bash
curl -X POST http://localhost:8080/urls \
  -H "Content-Type: application/json" \
  -d '{
    "original_url": "https://example.com/some-long-url"
  }'
```

Response:

```json
{
  "base_url": "http://localhost:8080/3jZ8q..."
}
```

The response field is currently named `base_url` and contains the complete generated short URL.

## Project layout

- `src/main.rs` - Loads configuration and starts the Actix Web server
- `src/routes.rs` - HTTP route handlers
- `src/database.rs` - PostgreSQL connection setup
- `src/shapes.rs` - Request and response types
- `src/utils.rs` - Base62 short-code generation

## Development

Format the code and run checks with:

```bash
cargo fmt
cargo check
```

## Current scope

This version provides URL creation and persistence. A redirect route for resolving a short code to its original URL is not implemented yet.
