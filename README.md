# Sunangel Home Edition

![Frontend Version](https://img.shields.io/badge/frontend-v0.0.6-blue)
![Backend Version](https://img.shields.io/badge/backend-v0.1.3-blue)
[![Build and Test](https://github.com/sunangel-project/Sunangel/actions/workflows/test.yml/badge.svg?branch=develop)](https://github.com/sunangel-project/Sunangel/actions/workflows/test.yml)
[![Website](https://img.shields.io/badge/website-limegreen)](https://sunn.cloudsftp.de)

## Execution

### Backend

Run with `docker` or `podman`.

```
docker compose --profile all up
```

```
podman-compose --profile all up
```

For `podman` remember to install `podman-compose` and the [dnsname plugin](https://github.com/containers/dnsname/tree/maig) (package `cni-plugin-dnsname` on openSuse)

Currently, there are two profiles:
- api
    - nats
    - api
    - spot-finder
- compute
    - horizon-service
    - sky-service

### Frontend

Currently, it is not in the `docker-compose.yml` file.
Run with `npm` or `bun`.

```
cd front
npm install # only needed once
npm run dev
```

You can also compile it to `html` and `javascript`.

```
cd front
bun run build
```

## Execution

## Planned Architecture

![arch](architecture.png)

### Rationale

- `spot-finder` creates many messages from one request (one message per found location)
- `API` has to gather all messages that belong to the same request
- requests identified w/ UUID

Only API component has state.
All other components can scale horizontally w/o restrictions.
[Queues](https://en.wikipedia.org/wiki/Message_queue) used for communication for free load balancing ([competing consumer](https://learn.microsoft.com/en-us/azure/architecture/patterns/competing-consumers)).

In order to allow `API` component to scale, use [publish-subscribe](https://learn.microsoft.com/en-us/azure/architecture/patterns/publisher-subscriber) for relaying responses to the `API` components.
