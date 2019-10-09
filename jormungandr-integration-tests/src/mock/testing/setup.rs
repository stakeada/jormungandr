use crate::common::{
    configuration::jormungandr_config::JormungandrConfig, jormungandr::JormungandrProcess,
    startup::*,
};
use crate::mock::client::JormungandrClient;
use std::{thread, time::Duration};

const LOCALHOST: &str = "127.0.0.1";

pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn attach_to_local_node(port: u16) -> Self {
        Self {
            host: String::from(LOCALHOST),
            port: port,
        }
    }

    pub fn client(&self) -> JormungandrClient {
        JormungandrClient::new(&self.host, self.port)
    }
}

pub fn bootstrap_node() -> (JormungandrProcess, JormungandrConfig) {
    let mut config = ConfigurationBuilder::new().with_slot_duration(4).build();
    let server = start_jormungandr_node_as_leader(&mut config);
    thread::sleep(Duration::from_secs(4));
    (server, config)
}

pub fn build_configuration(mock_port: u16) -> JormungandrConfig {
    ConfigurationBuilder::new()
        .with_slot_duration(4)
        .with_trusted_peers(vec![format!("/ip4/{}/tcp/{}", LOCALHOST, mock_port)])
        .build()
}

pub fn bootstrap_node_with_peer(mock_port: u16) -> (JormungandrProcess, JormungandrConfig) {
    let mut config = build_configuration(mock_port);
    let server = start_jormungandr_node_as_leader(&mut config);
    thread::sleep(Duration::from_secs(4));
    (server, config)
}
