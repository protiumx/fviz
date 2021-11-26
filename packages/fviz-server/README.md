# fviz-server

Rust http server implemented with warp and tokio.

## Conventions

We follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html) as much as possible.


## Folder structure

- `src/`: source code folder
  - `transport/`: transport layers handlers
  
## Development

Run
```bash
cargo run
```

build
```bash
cargo build
```

## TODO
- [ ] Terminate client connection properly
