use aws_sdk_ssm as ssm;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn get_parameter(
    ssm_client: &ssm::Client,
    name: impl Into<String>,
) -> Result<String, Error> {
    let resp = ssm_client
        .get_parameter()
        .name(name)
        .with_decryption(true)
        .send()
        .await?;

    resp.parameter
        .and_then(|p| p.value)
        .ok_or_else(|| Error::from("No parameter found"))
}

pub async fn put_parameter(
    ssm_client: &ssm::Client,
    name: impl Into<String>,
    value: impl Into<String>,
) -> Result<(), Error> {
    ssm_client
        .put_parameter()
        .name(name)
        .value(value)
        .overwrite(true)
        .send()
        .await?;
    Ok(())
}

pub async fn get_csv_list(
    ssm_client: &ssm::Client,
    name: impl Into<String>,
) -> Result<Vec<String>, Error> {
    let value = get_parameter(ssm_client, name).await?;
    Ok(value.split(',').map(str::to_string).collect())
}
