# Sunangel Home Edition


## Execution

### Backend

Run with `docker` or `podman`.

```
docker compose up
```

```
podman-compose up
```

For `podman` remember to install `podman-compose` and the [dnsname plugin](https://github.com/containers/dnsname/tree/maig) (package `cni-plugin-dnsname` on openSuse)

### Frontend

Currently, it is not in the `docker-compose` file.
Run with `npm`.

```
cd front
npm install # only needed once
npm run dev
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
