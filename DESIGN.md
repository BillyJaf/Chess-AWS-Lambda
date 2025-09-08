# Project Title: [Rust Chess Bot Backend]

## Authors
- Billy Jaffray

## Overview
A REST API that takes a FEN string and returns the best move.

## Background and Motivation
I enjoy both playing chess and programming - creating a chess bot is a natural extension.
The goal of the project is to become more familiar with Rust and low-level programming concepts.
I also intend to increase my confidence with RESTful APIs and general algorithms.

## Goals and Non-Goals
### Goals
- Create a chess bot that outputs reasonable 'moves' with a performance of >1800 elo.
- Become confident with Rust and low-level memory management.
- Learn backend design fundamentals / industry standards.

### Non-Goals
- Frontend UI design (this will be a separate project).
- Programming move generation (a high-level understanding is sufficient).

## Detailed Design

### System Architecture

### Components

### Data Models

## APIs

## `GET /status`

Performs a basic health check on the API. Useful to confirm that the service is running.

### Request

**Method:** `GET`  
**URL:** `/status`  
**Headers:** _None required_  

### Response

**Status Code:** `200 OK`  
**Headers:** `{ "Content-Type": "application/json" }` 
**Body:**
```json
{
  "status": "ok"
}
```

## `GET /legal_moves`

Returns all legal moves from a current position (represented as a [FEN string](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)). Useful when not playing against the bot.

### Request

**Method:** `POST`  
**URL:** `/legal_moves`  
**Headers:** `{ "Content-Type": "application/json" }`  
**Body Parameters:**

| Field | Type   | Required | Description                         |
|-------|--------|----------|-------------------------------------|
| `fen` | `String` | Yes      | A valid FEN string |

**Example Request Body:**
```json
{
  "fen": "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
}
```


### Responses

**Status Code:** `200 OK`  
**Headers:** `{ "Content-Type": "application/json" }`  
**Body Parameters:**

| Field | Type   | Description                         |
|-------|--------|-------------------------------------|
| `moves` | `Vec<String>` | A list of legal [UCI](https://en.wikipedia.org/wiki/Universal_Chess_Interface) moves. |

**Example Response Body:**

```json
{
  "moves": [ "a2a3", "b2b3", "c2c3", "d2d3", "e2e3" "f2f3", "g2g3", "h2h3", "a2a4", "b2b4", "c2c4", "d2d4", "e2e4", "f2f4", "g2g4", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3" ]
}
```

**Status Code:** `400 BAD REQUEST`  
**Headers:** `{ "Content-Type": "application/json" }`  
**Body Parameters:**

| Field | Type   | Description                         |
|-------|--------|-------------------------------------|
| `moves` | `Vec<String>` | A list of legal [UCI](https://en.wikipedia.org/wiki/Universal_Chess_Interface) moves. |

**Example Response Body:**

```json
{
  "moves": [ "a2a3", "b2b3", "c2c3", "d2d3", "e2e3" "f2f3", "g2g3", "h2h3", "a2a4", "b2b4", "c2c4", "d2d4", "e2e4", "f2f4", "g2g4", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3" ]
}
```

## Implementation Strategy

## Risks and Mitigations

## Testing Strategy

## Dependencies
- axum
- pleco
- serde
- tokio
- tower-http

---