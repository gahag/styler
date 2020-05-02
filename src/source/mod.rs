pub mod qaz;
pub mod netscience;

use async_trait::async_trait;

use crate::error::Error;


#[async_trait]
pub trait Source {
	async fn fetch(&self, text: &str) -> Result<Box<[String]>, Error>;
}


pub type Sources = Box<[Box<dyn Source>]>;
