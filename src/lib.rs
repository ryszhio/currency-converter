pub async fn convert_currency(from: &str, to: &str, amount: f64) -> Result<f64, Box<dyn std::error::Error>> {
    let from_lower = from.to_lowercase();
    let to_lower = to.to_lowercase();

    let url = format!(
        "https://latest.currency-api.pages.dev/v1/currencies/{from_lower}.json"
    );

    let response = reqwest::get(&url).await?.text().await?;
    let parsed: serde_json::Value = serde_json::from_str(&response)?;

    let rate = parsed
        .get(&from_lower)
        .and_then(|v| v.get(&to_lower))
        .and_then(|v| v.as_f64());

    match rate {
        Some(r) => Ok(amount * r),
        None => Err(format!("Currency pair {} -> {} not found.", from_lower, to_lower).into())
    }
}