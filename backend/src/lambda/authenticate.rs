use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_ssm as ssm;
use ecc_lib::http::{json_response, path_param};
use ecc_lib::ssm as ssm_util;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

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

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ssm_client = ssm::Client::new(&aws_config);
    let reqwest_client = ReqwestClient::new();

    run(service_fn(move |event| {
        let ssm_client = ssm_client.clone();
        let reqwest_client = reqwest_client.clone();
        async move { authenticate(event, &ssm_client, &reqwest_client).await }
    }))
    .await
}

async fn authenticate(
    event: Request,
    ssm_client: &ssm::Client,
    reqwest_client: &ReqwestClient,
) -> Result<Response<Body>, Error> {
    let code = path_param(&event).ok_or("No code found")?;

    let strava_client = get_client_details(ssm_client).await?;
    let auth = handle_authentication(reqwest_client, code, &strava_client).await?;
    let is_admin = check_if_admin(ssm_client, &auth.athlete.id.to_string()).await?;

    let user = User {
        id: auth.athlete.id.to_string(),
        name: format!("{} {}", auth.athlete.firstname, auth.athlete.lastname),
        access_token: auth.access_token,
        admin: is_admin,
    };

    json_response(200, &user)
}

async fn get_client_details(ssm_client: &ssm::Client) -> Result<StravaClient, Error> {
    const ID_PARAM: &str = "ecc-strava-client-id";
    const SECRET_PARAM: &str = "ecc-strava-client-secret";

    let ssm_resp = ssm_client
        .get_parameters()
        .names(ID_PARAM)
        .names(SECRET_PARAM)
        .with_decryption(true)
        .send()
        .await?;

    let mut id = None;
    let mut secret = None;

    for param in ssm_resp.parameters() {
        let name = param.name.as_deref().unwrap_or_default();
        let value = param
            .value
            .clone()
            .ok_or_else(|| Error::from(format!("No value found for {name}")))?;

        match name {
            ID_PARAM => id = Some(value),
            SECRET_PARAM => secret = Some(value),
            _ => {}
        }
    }

    Ok(StravaClient {
        id: id.ok_or("Missing Strava client id")?,
        secret: secret.ok_or("Missing Strava client secret")?,
    })
}

async fn handle_authentication(
    reqwest_client: &ReqwestClient,
    code: &str,
    client: &StravaClient,
) -> Result<AuthenticationResponse, Error> {
    let url = format!(
        "https://www.strava.com/oauth/token?client_id={}&client_secret={}&code={}&grant_type=authorization_code",
        client.id, client.secret, code
    );

    Ok(reqwest_client.post(url).send().await?.json().await?)
}

async fn check_if_admin(ssm_client: &ssm::Client, id: &str) -> Result<bool, Error> {
    let admin_ids = ssm_util::get_csv_list(ssm_client, "ecc-admin-strava-ids").await?;
    Ok(admin_ids.iter().any(|admin_id| admin_id == id))
}
