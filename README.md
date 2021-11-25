# fviz

Simple drone flight visualization.

## Getting Started

### Requirements

- Cargo, Rust
- NodeJS
- Yarn
- Docker

Build a package

```bash
make build package=<server|ui>
```

Run a package

```bash
make run package=<server|ui>
```

Test a package

```bash
make test package=<server|ui>
```

### Folder structure

- `docs/`: text and image assets for documentation
- `packages/`:
  - `fviz-server/`: `rust` web server
  - `fviz-ui`/: `react` client app

## System communication

![diagram](./docs/diagram.png)

```mermaid
sequenceDiagram
    fviz-device->>+fviz-server: ws/status (postition, system health)
    fviz-server->>+fviz-ui: ws/device-status (postition, system health)
    fviz-server->>+fviz-device: ws/commands (land, new position)
    fviz-ui->>+fviz-server: ws/command (land, new position)
```

## Client handshake/registration

![diagram](./docs/handshake.png)

```mermaid
sequenceDiagram
    fviz-ui->>+fviz-server: HTTP POST /handshake { client: "fviz-ui-000" }
    fviz-server-->>fviz-ui: HTTP { session_uuid: "" }
    fviz-ui-->>fviz-server: HTTP POST /ws/{session_uuid}
    fviz-server-->>fviz-ui: Connection: upgrade WS
```

## TODO

- [x] Setup Rust CI
- [x] Setup React CI
- [x] SetReact Yarn 2
- [ ] Setup ESLint
- [ ] Setup Stylelint
- [ ] Create Releases
- [ ] Build docker images
- [ ] Setup deployments
- [ ] Setup renovate bot
- [ ] Setup commit lint
- [ ] Setup PR check (size, title, etc)
- [ ] Add logs to server
- [ ] Define Swagger documentation
- [ ] Load configuration in memory
- [ ] Setup server graceful shutdown
