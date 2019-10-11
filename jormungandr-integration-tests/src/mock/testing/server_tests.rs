use crate::{
    common::{
        configuration, file_utils, jormungandr::logger::Level,
        jormungandr::starter::start_jormungandr_node_as_leader,
    },
    mock::{
        server::{self, MethodType, MockLogger, ProtocolVersion},
        testing::{setup::bootstrap_node_with_peer, setup::build_configuration},
    },
};
use chain_core::property::FromStr;
use chain_impl_mockchain::key::Hash;
use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub fn start_mock<F: 'static>(
    mock_port: u16,
    genesis_hash: Hash,
    tip_hash: Hash,
    protocol_version: ProtocolVersion,
    stop_func: F,
) -> JoinHandle<()>
where
    F: Fn(&MockLogger) -> bool,
    F: std::marker::Send,
{
    let log_file = file_utils::get_path_in_temp("mock.log");
    let logger = MockLogger::new(log_file.clone());

    thread::spawn(move || {
        let _server = server::start(
            mock_port,
            genesis_hash,
            tip_hash,
            protocol_version,
            log_file.clone(),
        );

        let start = Instant::now();
        let timeout = Duration::from_secs(120);

        loop {
            if start.elapsed() > timeout {
                return;
            }
            if stop_func(&logger) {
                return;
            }
            thread::sleep(Duration::from_secs(1));
        }
    })
}

const FAKE_HASH: &str = "efe2d4e5c4ad84b8e67e7b5676fff41cad5902a60b8cb6f072f42d7c7d26c944";

pub fn fake_hash() -> Hash {
    Hash::from_str(FAKE_HASH).unwrap()
}

pub fn peer_addr(port: u16) -> Option<String> {
    Some(format!("127.0.0.1:{}", port))
}

// L1005 Handshake version discrepancy
#[test]
pub fn wrong_protocol() {
    let mock_port = configuration::get_available_port();
    let mut config = build_configuration(mock_port);

    let mock_thread = start_mock(
        mock_port,
        Hash::from_str(&config.genesis_block_hash).unwrap(),
        fake_hash(),
        ProtocolVersion::Bft,
        |logger: &MockLogger| logger.executed_at_least_once(MethodType::Handshake),
    );

    let server = start_jormungandr_node_as_leader(&mut config);
    mock_thread.join().expect("mock thread error");

    assert!(server.logger.get_log_entries().any(|x| {
        x.msg == "protocol handshake failed: UnsupportedVersion(\"0\")"
            && x.peer_addr == peer_addr(mock_port)
            && x.level == Level::WARN
    }));
}

// L1004 Handshake hash discrepancy
#[test]
pub fn wrong_genesis_hash() {
    let mock_port = configuration::get_available_port();
    let mock_thread = start_mock(
        mock_port,
        fake_hash(),
        fake_hash(),
        ProtocolVersion::GenesisPraos,
        |logger: &MockLogger| logger.executed_at_least_once(MethodType::Handshake),
    );

    let (server, _) = bootstrap_node_with_peer(mock_port);
    mock_thread.join().expect("mock thread error");

    assert!(server.logger.get_log_entries().any(|x| {
        x.msg.contains("block 0 hash")
            && x.peer_addr == peer_addr(mock_port)
            && x.level == Level::WARN
    }));
}

// L1002 Handshake compatible
#[test]
pub fn handshake_ok() {
    let mock_port = configuration::get_available_port();
    let mut config = build_configuration(mock_port);

    let mock_thread = start_mock(
        mock_port,
        Hash::from_str(&config.genesis_block_hash).unwrap(),
        fake_hash(),
        ProtocolVersion::GenesisPraos,
        |logger: &MockLogger| logger.executed_at_least_once(MethodType::Handshake),
    );

    let server = start_jormungandr_node_as_leader(&mut config);
    mock_thread.join().expect("mock thread error");

    assert!(!server
        .logger
        .get_log_entries()
        .any(|x| { x.peer_addr == peer_addr(mock_port) && x.level == Level::WARN }));
}

//L1008 Tip request hash discrepancy
#[test]
pub fn tip_request_malformed_discrepancy() {
    let mock_port = configuration::get_available_port();
    let mut config = build_configuration(mock_port);

    let mock_thread = start_mock(
        mock_port,
        Hash::from_str(&config.genesis_block_hash).unwrap(),
        fake_hash(),
        ProtocolVersion::GenesisPraos,
        |logger: &MockLogger| logger.executed_at_least_once(MethodType::Tip),
    );

    let server = start_jormungandr_node_as_leader(&mut config);
    mock_thread.join().expect("mock thread error");

    println!("{}", server.logger.get_log_content());
}
