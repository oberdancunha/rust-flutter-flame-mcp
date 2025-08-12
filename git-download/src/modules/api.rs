use crate::modules::{http::Http, logger::Logger};
use crate::structs::copy_request::CopyRequest;
use anyhow::Result;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::{future::Future, pin::Pin};

#[derive(Debug, Deserialize)]
pub struct Api {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub ftype: String,
}

impl Api {
    pub async fn fetch(
        http: Http,
        api: &str,
        original_api: &str,
        headers: &HeaderMap,
        docs_cache_path: &String,
    ) -> Result<Vec<CopyRequest>> {
        let copy_request =
            (Self::fetch_inner(http, api, original_api, headers, docs_cache_path).await).unwrap();
        Ok(copy_request)
    }

    fn fetch_inner(
        http: Http,
        api: &str,
        original_api: &str,
        headers: &HeaderMap,
        docs_cache_path: &String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<CopyRequest>>> + Send>> {
        let http = http.clone();
        let api = api.to_string();
        let original_api = original_api.to_string();
        let headers = headers.clone();
        let docs_cache_path = docs_cache_path.clone();
        let mut copy_request: Vec<CopyRequest> = vec![];
        Box::pin(async move {
            let response = http.clone().make_request::<Vec<Self>>(&api, &headers).await;
            match response {
                Ok(response_data) => {
                    for data in response_data.iter() {
                        if data.ftype == "dir" {
                            let api = format!("{}/{}", api, data.name);
                            copy_request.extend(
                                Self::fetch(
                                    http.clone(),
                                    &api,
                                    &original_api,
                                    &headers,
                                    &docs_cache_path,
                                )
                                .await?,
                            );
                        } else {
                            if data.name.ends_with(".md") {
                                let path = &data.path;
                                let dst = path.replace("doc", "docs_cache");
                                Logger::log(format!("File to be get {}", path).as_str());
                                copy_request.push(CopyRequest::add_file(path, dst));
                            }
                        }
                    }
                }
                Err(error) => Logger::log_error(format!("Error: {}", error).as_str()),
            };

            Ok(copy_request)
        })
    }
}
