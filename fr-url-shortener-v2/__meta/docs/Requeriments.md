# Requirements

## Functional

- Allow to receive a URL and return a shorter URL
- Allow to receive a short URL and redirect automatically

## Non-functional
- At least short 100 URLs per second.
- Instrument the service with open telemetry.
- Collect the metrics, logs and traces using some tool like prometheus + grafana or SigNoz

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