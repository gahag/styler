use scraper::Html;

use crate::error::Error;


pub async fn fetch_html<U, Q>(
	client: &reqwest::Client,
	url: U,
	query: &Q
) -> Result<Html, Error>
where
	U: reqwest::IntoUrl,
	Q: serde::Serialize + ?Sized,
{
	let response = client
		.get(url)
		.query(query)
		.send()
		.await?
		.text()
		.await?;

	Ok(
		Html::parse_document(&response)
	)
}
