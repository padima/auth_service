# Auth Service

JWT authentication microservice built with Rust and Axum.

The service provides two core operations:

- token generation for authenticated users;
- token validation for protected API calls.

It is designed as a lightweight stateless component that can be reused by other services in your system.

## Features

- JWT signing and verification with HS256.
- Automatic `iat` and `exp` handling when they are not provided.
- Optional custom signing key per request.
- Stateless API with simple JSON contracts.
- Health check endpoint for monitoring and readiness probes.

## How It Works

1. A client sends user data to `POST /generate`.
2. The service builds JWT claims and fills missing timestamps:
3. `iat` is set to current Unix time if `null`.
4. `exp` is set to current time + 12 hours if `null`.
5. The token is signed and returned to the client.
6. For validation, the client sends a token to `POST /validate`.
7. If signature and claims are valid, the service returns `is_valid: true` and decoded claims.

Default expiration window is 12 hours.

## Endpoints

- `GET /health` - returns `OK`.
- `POST /generate` - creates a signed JWT.
- `POST /validate` - validates a JWT and returns claims.

## Data Models

### GenerateRequest

```json
{
	"user_id": "string",
	"claims": {
		"sub": "string | null",
		"iss": "string | null",
		"aud": "string | null",
		"exp": "number | null",
		"iat": "number | null"
	},
	"key": "string | null"
}
```

### GenerateResponse

```json
{
	"token": "<jwt_token>"
}
```

### ValidateRequest

```json
{
	"token": "<jwt_token>",
	"key": "string | null"
}
```

### ValidateResponse

```json
{
	"is_valid": true,
	"claims": {
		"sub": "user123",
		"iss": null,
		"aud": null,
		"exp": 1750000000,
		"iat": 1749996400
	}
}
```

## Configuration

You can change service settings via a `.env` file in the project root.

Environment variables:

- `SERVER` - bind address (default: `127.0.0.1:8088`)
- `KEY` - default signing/validation key (default: `my_secret_key`)

Example `.env`:

```env
SERVER=127.0.0.1:8088
KEY=my_secret_key
```

If `.env` is not provided, the service uses defaults:

- `SERVER=127.0.0.1:8088`
- `KEY=my_secret_key`

## Key Usage

You can use two key strategies:

- Global key: set `KEY` in `.env` and reuse it for all requests.
- Per-request key: send `key` in request body for `POST /generate` and `POST /validate`.

Priority rule:

- If request `key` is provided, it is used.
- If request `key` is `null` or omitted, the service uses global `KEY` from `.env`.

## Run

```bash
cargo run
```

The service starts on the address from `SERVER`.

## Health Check

```http
GET /health
```

Response:

```text
OK
```

## Example: Generate Token

```http
POST /generate
Content-Type: application/json

{
	"user_id": "user123",
	"claims": {
		"sub": "user123",
		"iss": null,
		"aud": null,
		"exp": null,
		"iat": null
	},
	"key": null
}
```

Notes:

- If `claims` is omitted, default claims are created.
- If `iat` is `null`, current Unix time is used.
- If `exp` is `null`, expiration is set automatically.

Example response:

```json
{
	"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

## Example: Validate Token

```http
POST /validate
Content-Type: application/json

{
	"token": "<jwt_token>",
	"key": null
}
```

Example valid response:

```json
{
	"is_valid": true,
	"claims": {
		"sub": "user123",
		"iss": null,
		"aud": null,
		"exp": 1750000000,
		"iat": 1749996400
	}
}
```

Example invalid response:

```json
{
	"is_valid": false,
	"claims": null
}
```

## Status Codes

- `200 OK` for successful requests and validation results.
- `500 Internal Server Error` if token generation fails unexpectedly.
- `400 Bad Request` when request JSON is malformed.

## Security Notes

- Use a strong secret key in production.
- Keep the `KEY` value outside source control.
- Use HTTPS between clients and this service.
- Consider rotating keys and shortening token lifetime for sensitive systems.
