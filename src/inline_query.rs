use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;

use telegram_bot::{
	Api,
	CanAnswerInlineQuery,
	InlineQuery,
	InlineQueryResult,
	InlineQueryResultArticle,
	InputTextMessageContent,
	ParseMode,
};

use crate::source::Sources;
use crate::error::Error;


pub async fn handle(
	api: &Api,
	sources: &Sources,
	query: InlineQuery
) -> Result<(), Error> {
	let items: FuturesUnordered<_> = sources
		.iter()
		.map(
			|src| src.fetch(&query.query)
		)
		.collect();

	let result = items
		.filter_map(
			|result| async move {
				match result {
					Ok(items) => Some(items),
					Err(err) => {
						log::warn!("source error: {}", err);
						None
					}
				}
			}
		)
		.map(
			|items| futures::stream::iter(
				items
					.to_vec()
					.into_iter()
			)
		)
		.flatten()
		.enumerate()
		.map(
			|(id, text)| build_result(id, text)
		)
		.collect()
		.await;

	api
		.send(
			query
				.id
				.answer(result)
		)
		.await?;

	Ok(())
}


fn build_result(id: usize, text: String) -> InlineQueryResult {
	InlineQueryResult::from(
		InlineQueryResultArticle::new(
			format!("{:x}", id),
			text.clone(),
			InputTextMessageContent {
				message_text: text,
				parse_mode: Some(ParseMode::Markdown),
				disable_web_page_preview: true,
			}
		)
	)
}
