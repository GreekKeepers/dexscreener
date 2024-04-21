use std::sync::Arc;

use chrono::Utc;
use reqwest::Client;

pub mod errors;
pub mod models;

#[derive(Clone)]
pub struct DexScreener {
    client: Arc<Client>,
    last_refresh: i64,
    pairs: Arc<models::Pairs>,
}

impl DexScreener {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::new()),
            last_refresh: Default::default(),
            pairs: Arc::new(models::Pairs {
                pairs: Vec::with_capacity(0),
            }),
        }
    }

    pub async fn fetch_pairs(
        &mut self,
        pairs: &[&str],
    ) -> Result<Vec<models::Pair>, errors::Error> {
        let now = Utc::now().timestamp_millis();
        if now > self.last_refresh {
            let response = self
                .client
                .get(format!(
                    "https://api.dexscreener.com/latest/dex/tokens/{}",
                    pairs.join(",")
                ))
                .send()
                .await
                .map_err(errors::Error::RequestError)?
                .text()
                .await
                .map_err(errors::Error::RequestError)?;

            let pairs = serde_json::from_str(&response)
                .map_err(|e| errors::Error::SerdeError(e, response))?;
            self.pairs = Arc::new(pairs)
        }

        return Ok(self.pairs.pairs.clone());
    }
    pub async fn fetch_pairs_raw(
        &mut self,
        pairs: &str,
    ) -> Result<Vec<models::Pair>, errors::Error> {
        let now = Utc::now().timestamp_millis();
        if now > self.last_refresh {
            let response = self
                .client
                .get(format!(
                    "https://api.dexscreener.com/latest/dex/tokens/{}",
                    pairs
                ))
                .send()
                .await
                .map_err(errors::Error::RequestError)?
                .text()
                .await
                .map_err(errors::Error::RequestError)?;

            let pairs = serde_json::from_str(&response)
                .map_err(|e| errors::Error::SerdeError(e, response))?;
            self.pairs = Arc::new(pairs)
        }

        return Ok(self.pairs.pairs.clone());
    }
}


#[cfg(test)]
mod test{
    use super::*;
    #[tokio::test]
    async fn test(){
        let mut screener = DexScreener::new();


        let pairs = screener.fetch_pairs(&["0x7f7F49B6128F7CB89BAaB704F6EA1662A270455b"]).await.unwrap();
        println!("{:?}",pairs);
    }
}
