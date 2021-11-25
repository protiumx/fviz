# fviz-simulator

Go package to simulate different devices: drones, airplanes, etc.

## Conventions

We follow [Uber's Go Style guideline](https://github.com/uber-go/guide/blob/master/style.md) as much as possible.

### Folder structure

- `cmd/`: main applications for the project. Each folder represents a simulation for a device/actor
  - `drone/`: drone actor simulator
- `devices/`: devices simulation implementations
  - `drone/`: package containing drone simulation implementation
