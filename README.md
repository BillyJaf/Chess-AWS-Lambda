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

Returns all legal moves from a current position (represented as a [FEN string](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)). Also returns whether the position is a stalemate, and if the position is a checkmate, who won.

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
| `moves` | `Vec<(String, String)>` | A list of tuples containing legal [UCI](https://en.wikipedia.org/wiki/Universal_Chess_Interface) moves and their corresponding resulting [FEN string](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation). |

**Example Response Body:**

```json
{
  "winner":  null,
  "stalemate":  false,
  "moves":  [
                [
                    "a2a3",
                    "7k/8/8/8/8/P7/8/K7 b - - 0 1"
                ],
                [
                    "a2a4",
                    "7k/8/8/8/P7/8/8/K7 b - - 0 1"
                ],
                [
                    "a1b1",
                    "7k/8/8/8/8/8/P7/1K6 b - - 1 1"
                ],
                [
                    "a1b2",
                    "7k/8/8/8/8/8/PK6/8 b - - 1 1"
                ]
            ]
}
```

**Status Code:** `400 BAD REQUEST`  
**Headers:** `{ "Content-Type": "application/json" }`  
**Body Parameters:**

| Field | Type   | Description                         |
|-------|--------|-------------------------------------|
| `error` | `String` | Debug message of `pleco::board::FenBuildError` |

**Example Response Body:**

```json
{
  "error": "invalid turn: p, expected 'w' or 'b'\n"
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