use crate::domain::resource::resource::Resource;
use crate::modules::documentation_files::DocumentationFiles;

pub struct ListResources;

impl ListResources {
    pub fn execute() -> Vec<Resource> {
        let files: Vec<DocumentationFiles> = DocumentationFiles::build_index().unwrap();
        let mut resources: Vec<Resource> = vec![];
        for file in files {
            let file_name = file.uri.replace("flame://", "").replace("/", " > ");
            resources.push(Resource {
                uri: file.uri,
                name: format!("Flame: {}", file_name),
                description: format!("Flame engine documentation: {}", file_name),
                mime_type: "text/markdown".into(),
            });
        }

        resources
    }
}
