pub mod modules;
pub mod structs;
use anyhow::Result;

use crate::modules::{
    api::Api, docs_cache::DocsCache, downloader::Downloader, http::Http, logger::Logger,
};
use crate::structs::repo::Repo;

pub async fn repo(name: impl Into<String>) -> Result<()> {
    Logger::new();
    Logger::log("Get Flutter Flame documentation");
    let repo = Repo::new(&name.into());
    let http = Http::new();
    let docs_cache = DocsCache::new().clean()?;
    let copy_request =
        Api::fetch(http, &repo.api, &repo.api, &repo.headers, &docs_cache.name).await?;
    Downloader::new(&repo)
        .add_git()?
        .get_files_from_repository(&copy_request)?
        .copy_files_to_local(&copy_request)?
        .remove_git()?;

    Ok(())
}
