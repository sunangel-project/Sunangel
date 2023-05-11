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
  spots(query: { location: { lat: 48.81909, lon: 9.59523 }, radius: 2000 }) {
    status
    spot {
      location {
        lat
        lon
      }
      kind
      events {
        sun {
          rise {
            time
            altitude
            azimuth
          }
          set {
            time
            altitude
            azimuth
          }
        }
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
          "lat": 48.8292947,
          "lon": 9.588803
        },
        "kind": "bench",
        "events": {
          "sun": {
            "rise": {
              "time": "2023-05-12T04:39:32.256939991+00:00",
              "altitude": 0.12915177391071947,
              "azimuth": -1.9033500393223863
            },
            "set": {
              "time": "2023-05-11T18:39:32.256939991+00:00",
              "altitude": 0.014008644862302033,
              "azimuth": 2.040210844272874
            }
          }
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
  spots(query: { location: { lat: 48.81909, lon: 9.59523 }, radius: -1 }) {
    status
    spot {
      location {
        lat
        lon
      }
      kind
      events {
        sun {
          rise {
            time
            altitude
            azimuth
          }
          set {
            time
            altitude
            azimuth
          }
        }
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
      "message": "Internal server error",
      "locations": [
        {
          "line": 2,
          "column": 3
        }
      ],
      "path": [
        "spots"
      ],
      "extensions": "{\"input\":\"Message { message: Message { subject: \\\"SEARCH.request\\\", reply: Some(\\\"$JS.ACK.SEARCH.spot-finder.1.3.3.1683829132121901470.0\\\"), payload: b\\\"{\\\\\\\"request_id\\\\\\\":\\\\\\\"a20f0c87-75bf-4347-a0b9-62786af303eb\\\\\\\",\\\\\\\"search_query\\\\\\\":{\\\\\\\"loc\\\\\\\":{\\\\\\\"lat\\\\\\\":48.81909,\\\\\\\"lon\\\\\\\":9.59523},\\\\\\\"rad\\\\\\\":-1}}\\\", headers: None, status: None, description: None, length: 184 }, context: Context { client: Client { info: Receiver { shared: Shared { value: RwLock { data: ServerInfo { server_id: \\\"NBEFROIOBP2XVPEHMU6SPDIFVVNQDEM2CEOYGTCZ3DO33W6LVJ3LEOZD\\\", server_name: \\\"main\\\", host: \\\"0.0.0.0\\\", port: 4222, version: \\\"2.9.14\\\", auth_required: false, tls_required: false, max_payload: 1048576, proto: 1, client_id: 32, go: \\\"go1.19.5\\\", nonce: \\\"\\\", connect_urls: [], client_ip: \\\"172.20.0.1\\\", headers: true, lame_duck_mode: false }, poisoned: false, .. }, version: Version(0), is_closed: false, ref_count_rx: 5 }, version: Version(0) }, state: Receiver { shared: Shared { value: RwLock { data: Connected, poisoned: false, .. }, version: Version(2), is_closed: false, ref_count_rx: 5 }, version: Version(0) }, sender: Sender { chan: Tx { inner: Chan { tx: Tx { block_tail: 0x55c8bee9cb00, tail_position: 593 }, semaphore: Semaphore { semaphore: Semaphore { permits: 128 }, bound: 128 }, rx_waker: AtomicWaker, tx_count: 6, rx_fields: \\\"...\\\" } } }, next_subscription_id: 107, subscription_capacity: 4096, inbox_prefix: \\\"_INBOX\\\", request_timeout: Some(10s) }, prefix: \\\"$JS.API\\\", timeout: 5s } }\",\"reason\":\"invalid value: integer `-1`, expected u32 at line 1 column 114\",\"request_id\":\"a20f0c87-75bf-4347-a0b9-62786af303eb\",\"sender\":\"spot-finder\"}"
    }
  ]
}
```

Maybe display `message` in the UI.
