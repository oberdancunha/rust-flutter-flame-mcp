use crate::{
    data_source::docs_cache_data_source::DocsCacheDataSource, domain::resource::resource::Resource,
};

pub struct ListResources;

impl ListResources {
    pub fn execute() -> Vec<Resource> {
        let docs_cache_data_source = DocsCacheDataSource::new();
        let mut resources: Vec<Resource> = vec![];
        for uri in docs_cache_data_source.files_uri.iter() {
            let file_name = &uri.replace("flame://", "").replace("/", " > ");
            resources.push(Resource {
                uri: uri.clone(),
                name: format!("Flame: {}", file_name),
                description: format!("Flame engine documentation: {}", file_name),
                mime_type: "text/markdown".into(),
            });
        }

        resources
    }
}
