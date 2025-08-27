use regex::Regex;

use crate::data_source::docs_cache_data_source::DocsCacheDataSource;

pub struct HandleResource {}

impl HandleResource {
    pub fn execute(uri: &str) -> String {
        let content = DocsCacheDataSource::get_content(&uri).unwrap();
        let content = content
            .replace("\r\n", "\n")
            .replace("\r", "\n")
            .replace("\t", "    ");
        let re = Regex::new(r"[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]").unwrap();
        let content = re.replace_all(&content, "").to_string();

        content
    }
}
