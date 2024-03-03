use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_ssm as ssm;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
    access_token: String,
    admin: bool,
}

struct StravaClient {
    id: String,
    secret: String,
}

#[derive(Deserialize)]
struct AuthenticationResponse {
    athlete: Athlete,
    access_token: String,
}

#[derive(Deserialize)]
struct Athlete {
    id: u32,
    firstname: String,
    lastname: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(authenticate)).await
}

async fn authenticate(event: Request) -> Result<Response<Body>, Error> {
    let request_path = event.uri().path();
    let code = request_path.split("/").last().expect("No code found");

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ssm_client = ssm::Client::new(&aws_config);
    let reqwest_client = ReqwestClient::new();

    let strava_client = get_client_details(&ssm_client).await?;

    let user = handle_authentication(&reqwest_client, code, &strava_client).await?;
    let is_admin = check_if_admin(&ssm_client, &user.athlete.id.to_string()).await?;
    let user = User {
        id: user.athlete.id.to_string(),
        name: format!("{} {}", user.athlete.firstname, user.athlete.lastname),
        access_token: user.access_token,
        admin: is_admin,
    };

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(json!(user).to_string().into())
        .map_err(Box::new)?)
}

async fn get_client_details(ssm_client: &ssm::Client) -> Result<StravaClient, Error> {
    let id_param = "ecc-strava-client-id";
    let secret_param = "ecc-strava-client-secret";

    let ssm_resp = ssm_client
        .get_parameters()
        .names(id_param)
        .names(secret_param)
        .with_decryption(true)
        .send()
        .await?;

    let client: StravaClient = ssm_resp.parameters().iter().fold(
        StravaClient {
            id: String::new(),
            secret: String::new(),
        },
        |mut client, param| {
            let name = param.name.clone().expect("Parameter has no name");
            if name == id_param {
                client.id = param.value.clone().expect("No value found");
            } else if name == secret_param {
                client.secret = param.value.clone().expect("No value found");
            }
            client
        },
    );

    Ok(client)
}

async fn handle_authentication(
    reqwest_client: &ReqwestClient,
    code: &str,
    client: &StravaClient,
) -> Result<AuthenticationResponse, Error> {
    let endpoint = "https://www.strava.com/oauth/token";
    let url = format!(
        "{}?client_id={}&client_secret={}&code={}&grant_type=authorization_code",
        endpoint, client.id, client.secret, code
    );

    let auth_resp = reqwest_client.post(url).send().await?;
    let auth_json = auth_resp.json().await?;

    Ok(auth_json)
}

async fn check_if_admin(ssm_client: &ssm::Client, id: &str) -> Result<bool, Error> {
    let ssm_resp = ssm_client
        .get_parameter()
        .name("ecc-admin-strava-ids")
        .with_decryption(true)
        .send()
        .await?;

    let ssm_value = ssm_resp
        .parameter
        .expect("No parameter found")
        .value
        .expect("No value found");

    for admin_id in ssm_value.split(",") {
        if admin_id == id {
            return Ok(true);
        }
    }
    Ok(false)
}
