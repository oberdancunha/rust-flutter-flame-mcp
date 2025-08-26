use crate::modules::documentation_files::DocumentationFiles;
use regex::Regex;

pub struct HandleResources {}

impl HandleResources {
    pub fn execute(uri: &str) -> String {
        let content = DocumentationFiles::get_content(&uri).unwrap();
        let content = content
            .replace("\r\n", "\n")
            .replace("\r", "\n")
            .replace("\t", "    ");
        let re = Regex::new(r"[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]").unwrap();
        let content = re.replace_all(&content, "").to_string();

        content
    }
}
