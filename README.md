# Rust Chess Bot Backend

## Authors
- Billy Jaffray

## Overview
A REST API for a chess client-server application.   
Checkout the frontend [here](https://github.com/BillyJaf/Chess-Frontend-TS)!

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

## `POST /legal_moves`

Returns all legal moves from a current position (represented as a [FEN string](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)) along with a field to indicate if the current game has ended (checkmate, stalemate or neither). Each element in the list of legal moves contains both the [UCI](https://en.wikipedia.org/wiki/Universal_Chess_Interface) and [SAN](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)) notation of the legal move, the resulting FEN string if the move is applied, and if the resulting game has ended.

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
  "fen": "k7/8/8/8/8/7P/8/K7 w - - 0 1"
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
| `san_move` | `String` | legal SAN move string from current position |
| `resulting_fen` | `String` | resulting FEN string of gamestate if the move is applied |
| `game_over` | `Option<GameOver>` | `null` if the resulting game is not over, otherwise the outcome of the resulting game |



**Example Response Body:**

```json
{
    "game_over": null,
    "legal_moves": [
        {
            "uci_move": "h3h4",
            "san_move": "h4",
            "resulting_fen": "k7/8/8/8/7P/8/8/K7 b - - 0 1",
            "game_over": null
        },
        {
            "uci_move": "a1b1",
            "san_move": "Kb1",
            "resulting_fen": "k7/8/8/8/8/7P/8/1K6 b - - 1 1",
            "game_over": null
        },
        {
            "uci_move": "a1a2",
            "san_move": "Ka2",
            "resulting_fen": "k7/8/8/8/8/7P/K7/8 b - - 1 1",
            "game_over": null
        },
        {
            "uci_move": "a1b2",
            "san_move": "Kb2",
            "resulting_fen": "k7/8/8/8/8/7P/1K6/8 b - - 1 1",
            "game_over": null
        }
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
  "error": "Invalid FEN: invalid number of fen sections: 1, expected 6\n"
}
```

## `POST /validate_fen`

Checks if a given FEN string is a valid position. Returns true with no error if the position is valid, or false with the given FEN error.
As the purpose of this endpoint is validation, it replies with 200 even if the given string is not valid.

### Request

**Method:** `POST`  
**URL:** `/validate_fen`  
**Headers:** `{ "Content-Type": "application/json" }`  
**Body Parameters:**

| Field | Type   | Required | Description                         |
|-------|--------|----------|-------------------------------------|
| `fen` | `String` | Yes | A potentially invalid FEN string |

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
| valid | `bool` | `true` if the game is valid, `false` otherwise |
| error | `Option<String>` | `null` if the game is valid, debug message of `pleco::board::FenBuildError` otherwise |


**Example Response Body 1:**

```json
{
  "valid":true,
  "error":null
}
```

**Example Response Body 2:**

```json
{
  "valid":false,
  "error":"invalid number of fen sections: 1, expected 6\n"
}
```

## `POST /best_move`

Returns the bot's best move from a current position in both [UCI](https://en.wikipedia.org/wiki/Universal_Chess_Interface) and [SAN](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)) notation along with the resulting FEN string, a field to indicate if the current game has ended (checkmate, stalemate or neither) and a list of legal moves the player can make from the resulting position. 

### Request

**Method:** `POST`  
**URL:** `/best_move`  
**Headers:** `{ "Content-Type": "application/json" }`  
**Body Parameters:**

| Field | Type   | Required | Description                         |
|-------|--------|----------|-------------------------------------|
| `fen` | `String` | Yes | A valid FEN string |

**Example Request Body:**
```json
{
  "fen": "k7/8/8/8/8/8/6P1/K7 b - - 0 1"
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
| uci_move | `String` | UCI notation of the move the bot believes is best |
| san_move | `String` | SAN notation of the move the bot believes is best |
| resulting_fen | `String` | The resulting FEN string if the move is applied to the current game |
| resulting_legal_moves | `Vec<ResultingGameState>` | List of possible gamestates from the current gamestate |

`GameOver` enum:
| Variant       | Meaning           |
| ------------- | ----------------- |
| `"White"`     | White checkmates  |
| `"Black"`     | Black checkmates  |
| `"Stalemate"` | Stalemate reached |

`ResultingGameState` struct:
| Field  | Type     | Description                      |
| ------ | -------- | -------------------------------- |
| `uci_move` | `String` | legal UCI move string from the resulting position |
| `san_move` | `String` | legal SAN move string from the resulting position |
| `resulting_fen` | `String` | resulting FEN string of gamestate if the move is applied |
| `game_over` | `Option<GameOver>` | `null` if the resulting game is not over, otherwise the outcome of the resulting game |



**Example Response Body:**

```json
{
    "game_over": null,
    "uci_move": "a8b7",
    "san_move": "Kb7",
    "resulting_fen": "8/1k6/8/8/8/8/6P1/K7 w - - 1 2",
    "resulting_legal_moves": [
        {
            "uci_move": "g2g3",
            "san_move": "g3",
            "resulting_fen": "8/1k6/8/8/8/6P1/8/K7 b - - 0 2",
            "game_over": null
        },
        {
            "uci_move": "g2g4",
            "san_move": "g4",
            "resulting_fen": "8/1k6/8/8/6P1/8/8/K7 b - - 0 2",
            "game_over": null
        },
        {
            "uci_move": "a1b1",
            "san_move": "Kb1",
            "resulting_fen": "8/1k6/8/8/8/8/6P1/1K6 b - - 2 2",
            "game_over": null
        },
        {
            "uci_move": "a1a2",
            "san_move": "Ka2",
            "resulting_fen": "8/1k6/8/8/8/8/K5P1/8 b - - 2 2",
            "game_over": null
        },
        {
            "uci_move": "a1b2",
            "san_move": "Kb2",
            "resulting_fen": "8/1k6/8/8/8/8/1K4P1/8 b - - 2 2",
            "game_over": null
        }
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
  "error": "Invalid FEN: invalid number of fen sections: 1, expected 6\n"
}
```

## Dependencies
- pleco
- serde
- serde_json
- tokio
- shakmaty
- lambda_http
---
