# Example URL Shortener backend service written in Rust.

# Tech Stack
- [PostgreSQL](https://www.postgresql.org/)
- [SeaORM](https://www.sea-ql.org/SeaORM/)
- [Rocket](https://rocket.rs/)
- [Open Telemetry Rust](https://github.com/open-telemetry/opentelemetry-rust)

# API Endpoints

## 1. Create a Shortened URL

**Endpoint:** `/shorten`

**Method:** `POST`

**Description:**
This endpoint allows creating a shortened URL from a URL provided in the request body.

**Example Request:**

```json
POST /shorten
{
  "url": "https://www.example.com"
}
```

**Example Response:**

```json
{
  "shortened_url": "https://short.ly/abc123"
}
```

## 2. Redirect using the Shortened URL

**Endpoint:** `/{shortened_url}`

**Method:** `GET`

**Description:**
This endpoint redirects to the original URL using the shortened URL.

**Example Request:**

```http
GET /abc123
```

**Redirects to:**

```http
https://www.example.com
```