# Frontend SSR

Axum host that server-renders the [`frontend`](../frontend) Yew app and ships as an AWS Lambda (`render-ui`).

Based on the Yew [`ssr_router`](https://github.com/yewstack/yew/tree/master/examples/ssr_router) example. UI source lives in `../frontend`; this crate only hosts it.

## Binaries

| Binary | Features | Role |
| --- | --- | --- |
| `server` | `ssr` | Axum SSR server (local + Lambda) |
| `client` | `hydration` | WASM hydration client |

## Develop

Install [Rust](https://www.rust-lang.org/), [Trunk](https://trunkrs.dev/), and (for deploy builds) [cargo-lambda](https://www.cargo-lambda.info/).

```bash
make          # default: dev — build WASM assets and run the server
make dev      # same as above
make build_wasm
make run      # cargo run --features=ssr --bin server -- --dir dist
```

`build_wasm` copies `public/`, `images/`, and `index.scss` from `../frontend`, then runs Trunk.

## Production / Lambda package

```bash
make prd      # build_wasm + Lambda zip
```

Produces `target/lambda/server/bootstrap.zip` (bootstrap + `dist/`), which backend Terraform deploys as the `render-ui` function.

Root `make` runs `make prd` here as part of the full project build.
