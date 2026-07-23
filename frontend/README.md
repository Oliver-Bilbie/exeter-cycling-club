# Frontend

Yew UI for the Exeter Cycling Club site.

Built as a library (`exeter_cycling_club`) used by:

- CSR dev server via Trunk (`trunk serve`)
- SSR/hydration host in [`../frontend-ssr`](../frontend-ssr)

Kept separate from the SSR server so UI work stays easy to iterate on.

## Stack

- Rust, Yew 0.21 (CSR + SSR features)
- yew-router, Bounce
- reqwest (API calls)
- Bulma (cloned into `public/` on build)
- Trunk for bundling

## Layout

```
src/
  app.rs           # router and app shell
  components/      # pages and UI pieces
  helpers/         # API clients, auth, form helpers
  constants/
  bin/             # CSR entrypoint
index.html
index.scss
images/
public/
```

### Main routes

Home, About, Contact, Ride page, Sign-in/out, OAuth redirect, mailing-list confirm/unsubscribe, attendance status, admin route select/cancel.

## Develop

Install [Rust](https://www.rust-lang.org/) and [Trunk](https://trunkrs.dev/).

```bash
make dev     # trunk serve --open --port 8080
# or
trunk serve --open
```

```bash
make build   # release Trunk build (pulls Bulma 0.9.4 into public/)
```

Point API calls at a running backend or the deployed API as configured in the helpers/constants.
