# Rust Chess Bot Backend

## Authors
- Billy Jaffray

## Overview
A REST API for a chess client-server application.

## Background and Motivation
I enjoy both playing chess and programming - creating a chess bot is a natural extension.
The goal of the project is to become more familiar with Rust and low-level programming concepts.
I also intend to increase my confidence with RESTful APIs and general algorithms.

## Goals and Non-Goals
### Goals
- Create a chess bot that outputs reasonable moves with a performance of >1800 elo.
- Become confident with Rust and low-level memory management.
- Learn backend design fundamentals / industry standards.

### Non-Goals
- Frontend UI design (this will be a separate project).
- Programming move generation (a high-level understanding is sufficient).

## Endpoints

## `GET /health_check`

Performs a basic health check on the API. Useful to confirm that the service is running.

### Request

**Method:** `GET`  
**URL:** `/health_check`  
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

Returns all legal moves from a current position (represented as a [FEN string](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)) along with a field to indicate if the current game has ended (checkmate, stalemate or neither). Each element in the list of legal moves contains the move (represented as a [UCI move](https://en.wikipedia.org/wiki/Universal_Chess_Interface)), the resulting FEN string if the move is applied, and if the resulting game has ended.

### Request

**Method:** `POST`  
**URL:** `/legal_moves`  
**Headers:** `{ "Content-Type": "application/json" }`  
**Body Parameters:**

| Field | Type   | Required | Description                         |
|-------|--------|----------|-------------------------------------|
| `fen` | `String` | Yes | A valid FEN string |

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

Top-Level Fields:
| Field | Type   | Description                         |
|-------|--------|-------------------------------------|
| game_over | `Option<GameOver>` | `null` if the game is not over, otherwise the outcome of the game |
| legal_moves | `Vec<ResultingGameState>` | List of possible gamestates from the current gamestate |

`GameOver` enum:
| Variant       | Meaning           |
| ------------- | ----------------- |
| `"White"`     | White checkmates  |
| `"Black"`     | Black checkmates  |
| `"Stalemate"` | Stalemate reached |

`ResultingGameState` struct:
| Field  | Type     | Description                      |
| ------ | -------- | -------------------------------- |
| `uci_move` | `String` | legal UCI move string from current position |
| `resulting_fen` | `String` | resulting FEN string of gamestate if the move is applied |
| `game_over` | `Option<GameOver>` | `null` if the resulting game is not over, otherwise the outcome of the resulting game |



**Example Response Body:**

```json
{

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

## Dependencies
- axum
- pleco
- serde
- tokio
- tower-http

---
