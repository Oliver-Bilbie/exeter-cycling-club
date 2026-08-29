# Backend

Rust AWS Lambdas and Terraform for the Exeter Cycling Club API and background jobs.

Package name: `exeter-cycling-club-api`  
Shared library: `ecc_lib` (`src/lib.rs`)

## Layout

```
src/
  lib.rs              # ecc_lib entry
  ddb.rs              # DynamoDB attribute helpers
  email.rs            # SES helpers
  http.rs             # request/response helpers
  route.rs            # route data model
  ssm.rs              # SSM parameter helpers
  templates/          # email HTML templates
  lambda/
    authenticate.rs
    contact.rs
    attendance/
    email/
    route/
terraform/            # AWS infrastructure
openapi.yaml          # API Gateway OpenAPI definition
```

## Shared library (`ecc_lib`)

| Module | Role |
| --- | --- |
| `ddb` | Read typed attributes from DynamoDB items (`attr::<T>`) |
| `email` | Send mail via SES |
| `http` | JSON/text responses, body parsing, path params |
| `route` | `Route` type used when storing/serving ride data |
| `ssm` | Read/write SSM parameters (e.g. current route) |

## Lambdas

### HTTP API (`ecc-api-*`)

| Binary | Method / path | Purpose |
| --- | --- | --- |
| `ecc-api-authenticate` | `GET /auth/{code}` | Exchange Strava OAuth code; return user/admin state |
| `ecc-api-contact` | `POST /contact` | Contact form → admin email |
| `ecc-api-get-route` | `GET /route`, `GET /route/{auth_token}` | Current route (public or admin-authenticated) |
| `ecc-api-set-route` | `PUT /route` | Publish route; email subscribers |
| `ecc-api-cancel-route` | `DELETE /route` | Cancel route; email subscribers |
| `ecc-api-subscribe` | `PUT /email` | Start mailing-list signup |
| `ecc-api-confirm-subscribe` | `PATCH /email` | Confirm subscription |
| `ecc-api-unsubscribe` | `DELETE /email` | Unsubscribe |
| `ecc-api-set-attendance` | `PUT /status` | Set ride attendance status |

API surface is defined in [`openapi.yaml`](openapi.yaml) (used by API Gateway).

### Scheduled (`ecc-process-*`)

| Binary | Schedule (EventBridge) | Purpose |
| --- | --- | --- |
| `ecc-process-attendance-report` | `cron(0 5 ? * SUN *)` | Email admins member attendance; reset statuses |
| `ecc-process-clear-route` | `cron(0 12 ? * SUN *)` | Clear the published route after the ride |

## AWS resources (Terraform)

Provisioned under `terraform/`:

- Lambda functions (API + SSR UI zip from `frontend-ssr`)
- API Gateway (from OpenAPI)
- CloudFront
- DynamoDB (members / attendance)
- SSM (route payload, secrets/config as used by code)
- SES
- EventBridge schedules
- IAM roles/policies

### Variables

See [`terraform/variables.tf`](terraform/variables.tf). Typical inputs in `terraform/terraform.tfvars`:

- `region`, `domain`, `app-name`, `cert_arn`
- `strava-client-id`, `strava-client-secret`
- `admin-strava-ids`, `admin-emails`

Do not commit secrets. `terraform.tfvars` may contain sensitive values — keep it local or use a secret store.

## Build and deploy

Requires [Rust](https://www.rust-lang.org/), [cargo-lambda](https://www.cargo-lambda.info/), and [Terraform](https://www.terraform.io/).

```bash
make build     # cargo lambda build --release --output-format zip
make init      # terraform init
make validate
make plan
make apply
make destroy   # tear down stack
```

`make` (default) runs `build validate plan apply`.

Build artifacts land under `target/lambda/<binary>/bootstrap.zip` and are referenced by Terraform.

## Local checks

```bash
cargo fmt
cargo check --lib
```

Some binaries need compile-time env (e.g. `ADMIN_EMAIL` via `env!`) matching deployed configuration.
