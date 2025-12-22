use crate::model::proxy_server::EntitiesConfig;
use std::fs;

pub struct Parser {}

impl Parser {
    pub fn read_configuration() -> EntitiesConfig {
        let yaml_content = fs::read_to_string("config.yml");
        let proxy_dto: EntitiesConfig =
            serde_yaml::from_str(Box::leak(yaml_content.unwrap().into_boxed_str()))
                .expect("REASON");
        proxy_dto
    }
}
