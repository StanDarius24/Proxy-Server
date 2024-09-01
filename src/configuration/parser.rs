use std::fs;
use crate::model::proxy_server::ProxyServer;

pub struct Parser {}

impl Parser {
    pub fn read_configuration() -> ProxyServer {
        let yaml_content = fs::read_to_string("config.yml");
        let proxy_dto: ProxyServer = serde_yaml::from_str(Box::leak(yaml_content.unwrap().into_boxed_str())).expect("REASON");
        proxy_dto
    }
}
