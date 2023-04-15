# API ![version](https://img.shields.io/badge/v0.0.0-blue.svg)

## Testing Queries

### Interactively

When API is running, open http://localhost:6660/playground in the browser.

### Real Applications

Do a `POST` request with the query as data to http://localhost:6660/graphql.

### Example Queries

#### Get Spots

##### Input

```
subscription spots {
  spots(query: { location: { lat: 48.0, lon: 9.0 }, radius: 2000 }) {
    status
    spot {
      location {
        lat
        lon
      }
      kind
      sunset {
        time
      	alt
        azi
      }
    }
  }
}
```

##### Output

Sequence of these `JSON` objects

```
{
  "data": {
    "spots": {
      "status": "RUNNING",
      "spot": {
        "location": {
          "lat": 48.07,
          "lon": 9.07
        },
        "kind": "bench",
        "sunset": {
          "time": "2023-03-11T22:08:51.885044784+00:00",
          "alt": 270,
          "azi": 1
        }
      }
    }
  }
}
```

The last spot will have `FINISHED` instead of `RUNNING` as `status`.

#### Provoking Error

##### Input

```
subscription spots {
  spots(query: { location: { lat: 48.0, lon: 9.0 }, radius: -1 }) {
    status
    spot {
      location {
        lat
        lon
      }
      kind
      sunset {
        time
        alt
        azi
      }
    }
  }
}
```

or simply
```
subscription spots {
  spots(query: { location: { lat: 48.0, lon: 9.0 }, radius: -1 }) {
    status
  }
}
```

##### Output

```
{
  "data": {
    "spots": null
  },
  "errors": [
    {
      "message": "search query illegal",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": [
        "spots"
      ],
      "extensions": -1
    }
  ]
}
```

Maybe display `message` in the UI.

## Incoming Messages

### Spot found

```
{
  "part": {
    "id": 44,
    "of": 45
  },
  "request_id": "d7a0e3f8-8e6e-4794-b8af-eba6f3f6cac3",
  "search_query": {
    "loc": {
      "lat": 48.81909,
      "lon": 9.59523
    },
    "rad": 2000
  },
  "spot": {
    "dir": null,
    "kind": "bench",
    "loc": {
      "lat": 48.8292947,
      "lon": 9.588803
    }
  }
}
```

### Error

```
{
  "input": "Message { subject: \"search\", reply: None, payload: b\"{\\\"request_id\\\":\\\"cf1b2b8c-abd9-4e15-b86b-014e6a8a0e8f\\\",\\\"search_query\\\":{\\\"loc\\\":{\\\"lat\\\":48.81909,\\\"lon\\\":9.59523},\\\"rad\\\":-1}}\", headers: None, status: None, description: None, length: 122 }",
  "reason": "invalid value: integer `-1`, expected u32 at line 1 column 114",
  "request_id": "\"cf1b2b8c-abd9-4e15-b86b-014e6a8a0e8f\"",
  "sender": "spot-finder"
}
```
