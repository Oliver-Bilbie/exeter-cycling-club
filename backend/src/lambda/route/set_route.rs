use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use aws_sdk_ssm as ssm;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use serde_json::json;
use std::env;

use route_lib::Route;

#[derive(Deserialize)]
struct SetRouteRequest {
    access_token: String,
    id: String,
    name: String,
    message: String,
    is_private: String,
}

struct StravaRouteData {
    distance: f64,
    elevation_gain: f64,
    map_url: String,
}

#[derive(Deserialize)]
struct EmailRecipient {
    email: String,
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(set_route)).await
}

async fn set_route(event: Request) -> Result<Response<Body>, Error> {
    let body = read_event_body(event)?;
    let access_token = body.access_token;
    let route_id = body.id;
    let name = body.name;
    let message = body.message;
    let is_private = body.is_private;

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ddb_client = ddb::Client::new(&aws_config);
    let ses_client = ses::Client::new(&aws_config);
    let ssm_client = ssm::Client::new(&aws_config);
    let reqwest_client = ReqwestClient::new();

    if !check_if_admin(&ssm_client, &reqwest_client, &access_token).await? {
        return Ok(Response::builder()
            .status(401)
            .header("content-type", "text/html")
            .body(json!({ "message": "Unauthorized" }).to_string().into())
            .map_err(Box::new)?);
    }

    let route_data = get_route_data(&reqwest_client, &access_token, &route_id).await?;
    let elevation_gain = format!("{:.0}m", route_data.elevation_gain);
    let distance = format!("{:.0}km", route_data.distance / 1000.0);

    let route = Route {
        status: "ready".to_string(),
        id: route_id,
        name,
        message,
        distance,
        elevation_gain,
        map_url: route_data.map_url,
        is_private,
    };

    update_route_data(&ssm_client, &route).await?;

    send_email_notifications(&ddb_client, &ses_client, &route).await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(
            json!({ "message": "Route set successfully" })
                .to_string()
                .into(),
        )
        .map_err(Box::new)?)
}

fn read_event_body(event: Request) -> Result<SetRouteRequest, Error> {
    let body = match event.body() {
        Body::Text(text) => serde_json::from_str(text).expect("Unable to parse body"),
        Body::Binary(input) => {
            let text = String::from_utf8(input.to_vec()).expect("Unable to parse binary body");
            serde_json::from_str(&text).expect("Unable to parse body")
        }
        _ => panic!("No event body was provided"),
    };
    Ok(body)
}

async fn check_if_admin(
    ssm_client: &ssm::Client,
    reqwest_client: &ReqwestClient,
    access_token: &String,
) -> Result<bool, Error> {
    let user_id = get_user_id(&reqwest_client, &access_token).await?;
    let admin_list = get_admin_list(&ssm_client).await?;
    Ok(admin_list.contains(&user_id))
}

async fn get_user_id(
    reqwest_client: &ReqwestClient,
    access_token: &String,
) -> Result<String, Error> {
    #[derive(Deserialize)]
    struct StravaAthlete {
        id: i32,
    }

    let user_resp = reqwest_client
        .get("https://www.strava.com/api/v3/athlete")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let user_json: StravaAthlete = user_resp.json().await?;
    let id_str = user_json.id.to_string();

    Ok(id_str)
}

async fn get_admin_list(ssm_client: &ssm::Client) -> Result<Vec<String>, Error> {
    let admin_list_ssm_id = env::var("ADMIN_IDS_SSM").expect("ADMIN_IDS_SSM not set");

    let ssm_resp = ssm_client
        .get_parameter()
        .name(admin_list_ssm_id)
        .with_decryption(true)
        .send()
        .await?;

    let ssm_value = ssm_resp
        .parameter
        .expect("No parameter found")
        .value
        .expect("No value found");

    let admin_list: Vec<String> = ssm_value.split(",").map(|s| s.to_string()).collect();

    Ok(admin_list)
}

async fn get_route_data(
    reqwest_client: &ReqwestClient,
    access_token: &String,
    route_id: &String,
) -> Result<StravaRouteData, Error> {
    #[derive(Deserialize)]
    struct StravaMapUrls {
        retina_url: String,
    }
    #[derive(Deserialize)]
    struct StravaRoute {
        distance: f64,
        elevation_gain: f64,
        map_urls: StravaMapUrls,
    }

    let route_resp = reqwest_client
        .get(&format!(
            "https://www.strava.com/api/v3/routes/{}",
            route_id
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let route_json: StravaRoute = route_resp.json().await?;

    Ok(StravaRouteData {
        distance: route_json.distance,
        elevation_gain: route_json.elevation_gain,
        map_url: route_json.map_urls.retina_url,
    })
}

async fn update_route_data(ssm_client: &ssm::Client, route: &Route) -> Result<(), Error> {
    let route_data_ssm_id = env::var("ROUTE_DATA_SSM").expect("ROUTE_DATA_SSM not set");
    let route_json = serde_json::to_string(route).expect("Unable to serialize route data");

    ssm_client
        .put_parameter()
        .name(route_data_ssm_id)
        .value(route_json)
        .overwrite(true)
        .send()
        .await?;
    Ok(())
}

async fn send_email_notifications(
    ddb_client: &ddb::Client,
    ses_client: &ses::Client,
    route: &Route,
) -> Result<(), Error> {
    let mailing_list = get_mailing_list(&ddb_client).await?;
    for recipient in mailing_list {
        match send_email(&ses_client, &recipient, &route).await {
            Ok(_) => println!("[INFO] Email sent to {}", recipient.email),
            Err(err) => println!("[ERROR] {:?}", err),
        }
    }
    Ok(())
}

async fn get_mailing_list(ddb_client: &ddb::Client) -> Result<Vec<EmailRecipient>, Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    let ddb_items = ddb_client
        .scan()
        .table_name(mailing_list_ddb_id)
        .filter_expression("#verified = :v")
        .expression_attribute_names("#verified", "verified")
        .expression_attribute_values(":v", ddb::types::AttributeValue::Bool(true))
        .send()
        .await?
        .items
        .expect("No items found");

    let mut mailing_list: Vec<EmailRecipient> = Vec::new();

    for item in ddb_items {
        let email = match item.get("email") {
            Some(value) => match value.as_s() {
                Ok(value_str) => value_str.to_string(),
                Err(err) => {
                    println!("[ERROR] {:?}", err);
                    continue;
                }
            },

            None => {
                println!("[ERROR] No email found");
                continue;
            }
        };

        let id = match item.get("id") {
            Some(value) => match value.as_s() {
                Ok(value_str) => value_str.to_string(),
                Err(err) => {
                    println!("[ERROR] {:?}", err);
                    continue;
                }
            },
            None => {
                println!("[ERROR] No id found");
                continue;
            }
        };

        let verified = match item.get("verified") {
            Some(value) => match value.as_bool() {
                Ok(value_bool) => value_bool,
                Err(err) => {
                    println!("[ERROR] {:?}", err);
                    continue;
                }
            },
            None => {
                println!("[ERROR] No verification status found");
                continue;
            }
        };

        if *verified {
            mailing_list.push(EmailRecipient { email, id });
        }
    }

    Ok(mailing_list)
}

async fn send_email(
    ses_client: &ses::Client,
    recipient: &EmailRecipient,
    route: &Route,
) -> Result<(), Error> {
    let mut destination: ses::types::Destination = ses::types::Destination::builder().build();
    destination.to_addresses = Some(vec![recipient.email.clone()]);

    let subject_content = ses::types::Content::builder()
        .data("This week's route")
        .charset("UTF-8")
        .build()
        .expect("Unable to build subject content");

    let body_content = ses::types::Content::builder()
        .data(build_email_body(&route, &recipient))
        .charset("UTF-8")
        .build()
        .expect("Unable to build body content");

    let body = ses::types::Body::builder().html(body_content).build();

    let message = ses::types::Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = ses::types::EmailContent::builder().simple(message).build();

    ses_client
        .send_email()
        // TODO: Replace with production domain
        .from_email_address("Exeter Cycling Club <ecc@oliver-bilbie.co.uk>")
        .destination(destination)
        .content(email_content)
        .send()
        .await?;

    Ok(())
}

fn build_email_body(route: &Route, recipient: &EmailRecipient) -> String {
    if route.is_private == "true" {
        let template_body = include_str!("../../templates/update-private.html");

        template_body
            .replace("%RECIPIENT_ID%", &recipient.id)
            .replace("$NEWLINE", "\n")
    } else {
        let template_body = include_str!("../../templates/update.html");

        template_body
            .replace("%ROUTE_NAME%", &route.name)
            .replace("%DESCRIPTION%", &route.message)
            .replace("%DISTANCE%", &route.distance)
            .replace("%ELEVATION_GAIN%", &route.elevation_gain)
            .replace("%MAP_URL%", &route.map_url)
            .replace("%RECIPIENT_ID%", &recipient.id)
            .replace("$NEWLINE", "\n")
    }
}
