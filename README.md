# Exeter Cycling Club Website

![Last-Commit](https://img.shields.io/github/last-commit/Oliver-Bilbie/exeter-cycling-club)

## Overview

Website for Exeter Cycling Club.

The app lets club admins sign in with Strava OAuth2 and publish an upcoming ride from a route on their Strava account. Subscribed members are emailed when a route is set or cancelled, and can confirm attendance from the email. On the morning of the ride, admins receive an attendance report.

- Live site: [https://ecc.oliver-bilbie.co.uk](https://ecc.oliver-bilbie.co.uk)

## Demo

Some features are admin-only. The demo below shows setting and cancelling a route (both notify the mailing list).

![Demo](demo.gif)

## Project layout

| Directory | Description |
| --- | --- |
| [`frontend/`](frontend/) | Yew UI (CSR + SSR-capable components) |
| [`frontend-ssr/`](frontend-ssr/) | Axum SSR host; builds the Lambda that serves the site |
| [`backend/`](backend/) | API and scheduled Lambdas, shared `ecc_lib`, OpenAPI, Terraform |

## Stack

- **Frontend:** Rust, Yew, yew-router, Bounce, Trunk, Bulma
- **SSR host:** Axum on AWS Lambda
- **API:** Rust Lambdas (`lambda_http` / `lambda_runtime`), DynamoDB, SSM, SES, Strava API
- **Infra:** Terraform (API Gateway, CloudFront, Lambda, DynamoDB, EventBridge, SES, SSM, IAM)

## Build

From the repo root:

```bash
make        # frontend + frontend-ssr (prod) + backend
make fmt    # rustfmt all crates
make update # cargo update all crates
```

See each subdirectory README for local development.

## High-level flow

1. Admin authenticates via Strava (`/auth/{code}`).
2. Admin selects a Strava route and publishes it (`PUT /route`); members are emailed.
3. Members confirm or decline attendance (`PUT /status`) and can manage mailing-list subscription (`/email`).
4. EventBridge runs attendance report (Sunday 05:00 UTC) and clears the route (Sunday 12:00 UTC).
