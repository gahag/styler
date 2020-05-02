use async_trait::async_trait;

use crate::error::Error;


pub struct Module {
	client: reqwest::Client,
}


impl Module {
	pub fn new(client: reqwest::Client) -> Self {
		Module { client }
	}
}


#[async_trait]
impl super::Source for Module {
	async fn fetch(&self, text: &str) -> Result<Box<[String]>, Error> {
		let item_selector = scraper::Selector
			::parse("table pre")
			.expect("invalid css selector");

		let page = crate::net
			::fetch_html(
				&self.client,
				"http://www.network-science.de/ascii/ascii.php",
				&[
					("TEXT", text),
					("FONT", "univers"),
					("RICH", "no"),
					("FORM", "left"),
					("STRE", "no"),
					("WIDT", "48")
				]
			)
			.await?;

		Ok(
			page
				.select(&item_selector)
				.filter_map(
					|element| element
						.text()
						.next()
				)
				.take(1)
				.map(
					|str| format!("```{}```", str)
				)
				.collect()
		)
	}
}
