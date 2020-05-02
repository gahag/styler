mod error;
mod inline_query;
mod net;
mod source;

use futures::StreamExt;

use telegram_bot::{
	UpdateKind,
	Api,
};


const BOT_KEY_VAR: &str = "BOT_KEY";


#[tokio::main]
async fn main() {
	let result = simple_logger::init_with_level(log::Level::Info);
	if let Err(err) = result {
		log::error!("failed to init log: {}", err);
		log::error!("failed to start!");
		std::process::exit(-1);
	};

	let api_key = match std::env::var(BOT_KEY_VAR) {
		Ok(var) => var,
		Err(err) => {
			log::error!("{}: {}", err, BOT_KEY_VAR);
			log::error!("failed to start!");
			std::process::exit(1);
		},
	};

	let reqwest_client = reqwest::Client::new();

	let modules: source::Sources = Box::new(
		[
			Box::new(source::qaz::Module::new(reqwest_client.clone())),
			Box::new(source::netscience::Module::new(reqwest_client))
		]
	);

	let api = Api::new(api_key);

	log::info!("online!");

	let mut stream = api.stream();

	while let Some(update) = stream.next().await {
		let update = match update {
			Ok(update) => update,
			Err(err) => {
				log::error!("update error: {}", err);

				continue
			}
		};

		match update.kind {
			UpdateKind::InlineQuery(query) => {
				log::info!(
					"query from {} ({}): {}",
					query.from.first_name,
					query.from.username
						.as_deref()
						.unwrap_or("?"),
					query.query
				);

				let result = inline_query::handle(&api, &modules, query).await;

				if let Err(err) = result {
					log::error!("failed to answer query: {}", err);
				}
			},
			other => log::warn!("unsupported update kind: {:?}", other),
		};
	};
}
