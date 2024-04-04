# API

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
  spots(query: {
    time: "2023-10-15T12:53:56Z",
    timezone: "Europe/Berlin",
    lowerLeft: { lat: 48.81909, lon: 9.59523 },
    upperRight: { lat: 48.90207, lon: 9.69243 },
  }) {
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
  spots(query: {
    time: "2023-10-15T12:53:56Z",
    timezone: "Europe/Berlin",
    lowerLeft: { lat: 0, lon: 0 },
    upperRight: { lat: 1, lon: 1 },
  }) {
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
      "extensions": "{\"input\":\"Message { message: Message { subject: \\\"SEARCH.request\\\", reply: Some(\\\"$JS.ACK.SEARCH.spot-finder.1.2.2.1712252121708503552.0\\\"), payload: b\\\"{\\\\\\\"request_id\\\\\\\":\\\\\\\"f4d3af02-2e7a-47b5-bf95-b2805c859643\\\\\\\",\\\\\\\"search_query\\\\\\\":{\\\\\\\"time\\\\\\\":\\\\\\\"2024-04-04T17:35:21.676Z\\\\\\\",\\\\\\\"timezone\\\\\\\":\\\\\\\"Europe/Berlin\\\\\\\",\\\\\\\"lower_left\\\\\\\":{\\\\\\\"lat\\\\\\\":48.69800285823639,\\\\\\\"lon\\\\\\\":9.339309113585992},\\\\\\\"upper_right\\\\\\\":{\\\\\\\"lat\\\\\\\":48.911247502160954,\\\\\\\"lon\\\\\\\":9.77932287169325}}}\\\", headers: None, status: None, description: None, length: 326 }, context: Context { client: Client { info: Receiver { shared: Shared { value: RwLock { data: ServerInfo { server_id: \\\"NBQKEGVM4NKJ5DPBS7DHX2253MQIJODWDTPSTVLGHOU4KXNZHW2BAFRT\\\", server_name: \\\"main\\\", host: \\\"0.0.0.0\\\", port: 4222, version: \\\"2.10.12\\\", auth_required: false, tls_required: false, max_payload: 1048576, proto: 1, client_id: 34, go: \\\"go1.21.8\\\", nonce: \\\"\\\", connect_urls: [], client_ip: \\\"172.18.0.4\\\", headers: true, lame_duck_mode: false }, poisoned: false, .. }, version: Version(0), is_closed: false, ref_count_rx: 5 }, version: Version(0) }, state: Receiver { shared: Shared { value: RwLock { data: Connected, poisoned: false, .. }, version: Version(2), is_closed: false, ref_count_rx: 5 }, version: Version(0) }, sender: Sender { chan: Tx { inner: Chan { tx: Tx { block_tail: 0x55f4804bb9a0, tail_position: 22 }, semaphore: Semaphore { semaphore: Semaphore { permits: 128 }, bound: 128 }, rx_waker: AtomicWaker, tx_count: 6, rx_fields: \\\"...\\\" } } }, next_subscription_id: 2, subscription_capacity: 4096, inbox_prefix: \\\"_INBOX\\\", request_timeout: Some(10s) }, prefix: \\\"$JS.API\\\", timeout: 5s } }\",\"reason\":\"search area too big, diagonal was larger than 5000 meters\",\"request_id\":\"f4d3af02-2e7a-47b5-bf95-b2805c859643\",\"sender\":\"spot-finder\"}"
    }
  ]
}
```

TODO: Maybe display `message` in the UI.
