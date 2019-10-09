extern crate base64;
extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate hex;
extern crate protobuf;
extern crate serde;
extern crate serde_json;
extern crate slog;
extern crate slog_async;
extern crate slog_json;

use slog::Drain;

use self::serde::{Deserialize, Serialize};

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{
    common::file_utils,
    mock::{
        grpc::SingleResponse,
        proto::{node::*, node_grpc::*},
    },
};
use bytes::Bytes;
use chain_impl_mockchain::key::Hash;
use grpc::{Metadata, MetadataKey, Server};
use std::{fmt, iter};

pub fn start(
    port: u16,
    genesis_hash: Hash,
    tip: Hash,
    version: ProtocolVersion,
    log_path: PathBuf,
) -> Server {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port);
    server.add_service(NodeServer::new_service_def(JormungandrServerImpl::new(
        genesis_hash,
        tip,
        version,
        log_path,
    )));

    let server = server.build().expect("server");
    println!("mock node started on port {}", port);
    server
}

#[derive(Debug)]
pub struct MockLogger {
    pub log_file_path: PathBuf,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum MethodType {
    Handshake,
    PullBlocksToTip,
    Tip,
}

impl fmt::Display for MethodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum Level {
    WARN,
    INFO,
    ERRO,
}

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub msg: String,
    pub level: Level,
    pub ts: String,
    pub method: MethodType,
}

impl MockLogger {
    pub fn new(log_file_path: PathBuf) -> Self {
        MockLogger { log_file_path }
    }

    pub fn get_log_content(&self) -> String {
        file_utils::read_file(&self.log_file_path)
    }

    fn parse_line_as_entry(&self, line: &String) -> LogEntry {
        self.try_parse_line_as_entry(line).unwrap_or_else(|error| panic!(
            "Cannot parse log line into json '{}': {}. Please ensure json logger is used for node. Full log content: {}",
            &line,
            error,
            self.get_log_content()
        ))
    }

    fn try_parse_line_as_entry(&self, line: &String) -> Result<LogEntry, impl std::error::Error> {
        serde_json::from_str(&line)
    }

    pub fn get_log_entries(&self) -> impl Iterator<Item = LogEntry> + '_ {
        self.get_lines_from_log()
            .map(move |x| self.parse_line_as_entry(&x))
    }

    pub fn executed_at_least_once(&self, method: MethodType) -> bool {
        self.get_log_entries().any(|entry| entry.method == method)
    }

    fn get_lines_from_log(&self) -> impl Iterator<Item = String> {
        let file = File::open(self.log_file_path.clone())
            .expect(&format!("cannot find log file: {:?}", &self.log_file_path));
        let reader = BufReader::new(file);
        reader.lines().map(|line| line.unwrap())
    }
}

#[derive(Clone, Debug)]
pub enum ProtocolVersion {
    Bft = 0,
    GenesisPraos = 1,
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct JormungandrServerImpl {
    genesis_hash: Hash,
    tip: Hash,
    protocol: ProtocolVersion,
    log: slog::Logger,
}

impl JormungandrServerImpl {
    fn init_logger(log_path: PathBuf) -> (slog::Logger) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
            .unwrap();

        let drain = slog_json::Json::new(file).add_default_keys().build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let log = slog::Logger::root(drain, o!());
        log
    }

    pub fn new(
        genesis_hash: Hash,
        tip: Hash,
        protocol: ProtocolVersion,
        log_path: PathBuf,
    ) -> Self {
        let log = JormungandrServerImpl::init_logger(log_path);
        JormungandrServerImpl {
            genesis_hash,
            tip,
            protocol,
            log,
        }
    }
}

impl Node for JormungandrServerImpl {
    fn handshake(
        &self,
        _o: ::grpc::RequestOptions,
        _p: HandshakeRequest,
    ) -> ::grpc::SingleResponse<HandshakeResponse> {
        info!(self.log,"Handshake method recieved";"method" => MethodType::Handshake.to_string());
        let mut handshake = HandshakeResponse::new();

        handshake.set_version(self.protocol.clone() as u32);
        handshake.set_block0(self.genesis_hash.as_ref().to_vec());

        SingleResponse::completed_with_metadata_and_trailing_metadata(
            get_metadata(),
            handshake,
            get_metadata(),
        )
    }

    fn tip(
        &self,
        _o: ::grpc::RequestOptions,
        _p: TipRequest,
    ) -> ::grpc::SingleResponse<TipResponse> {
        info!(self.log,"Tip request recieved";"method" => MethodType::Tip.to_string());
        let mut tip_response = TipResponse::new();
        tip_response.set_block_header(self.tip.as_ref().to_vec());
        ::grpc::SingleResponse::completed(tip_response)
    }

    fn get_blocks(
        &self,
        _o: ::grpc::RequestOptions,
        _p: BlockIds,
    ) -> ::grpc::StreamingResponse<Block> {
        println!("get_blocks");
        ::grpc::StreamingResponse::empty()
    }

    fn get_headers(
        &self,
        _o: ::grpc::RequestOptions,
        _p: BlockIds,
    ) -> ::grpc::StreamingResponse<Header> {
        println!("get_headers");
        ::grpc::StreamingResponse::empty()
    }

    fn get_fragments(
        &self,
        _o: ::grpc::RequestOptions,
        _p: FragmentIds,
    ) -> ::grpc::StreamingResponse<Fragment> {
        println!("get_fragments");
        ::grpc::StreamingResponse::empty()
    }

    fn pull_headers(
        &self,
        _o: ::grpc::RequestOptions,
        _p: PullHeadersRequest,
    ) -> ::grpc::StreamingResponse<Header> {
        println!("pull_headers");
        ::grpc::StreamingResponse::empty()
    }

    fn pull_blocks_to_tip(
        &self,
        _o: ::grpc::RequestOptions,
        _p: PullBlocksToTipRequest,
    ) -> ::grpc::StreamingResponse<Block> {
        info!(self.log,"PullBlocksToTip request recieved";"method" => MethodType::PullBlocksToTip.to_string());
        ::grpc::StreamingResponse::completed_with_metadata_and_trailing_metadata(
            get_metadata(),
            iter::from_fn(|| None).collect(),
            get_metadata(),
        )
    }

    fn push_headers(
        &self,
        _o: ::grpc::RequestOptions,
        _p: ::grpc::StreamingRequest<Header>,
    ) -> ::grpc::SingleResponse<PushHeadersResponse> {
        println!("push_headers");
        let header_response = PushHeadersResponse::new();
        ::grpc::SingleResponse::completed(header_response)
    }

    fn upload_blocks(
        &self,
        _o: ::grpc::RequestOptions,
        _p: ::grpc::StreamingRequest<Block>,
    ) -> ::grpc::SingleResponse<UploadBlocksResponse> {
        println!("upload_blocks");
        let block_response = UploadBlocksResponse::new();
        ::grpc::SingleResponse::completed(block_response)
    }

    fn block_subscription(
        &self,
        _o: ::grpc::RequestOptions,
        _p: ::grpc::StreamingRequest<Header>,
    ) -> ::grpc::StreamingResponse<BlockEvent> {
        println!("block_subscription");
        ::grpc::StreamingResponse::completed_with_metadata_and_trailing_metadata(
            get_metadata(),
            iter::from_fn(|| None).collect(),
            get_metadata(),
        )
    }

    fn content_subscription(
        &self,
        _o: ::grpc::RequestOptions,
        _p: ::grpc::StreamingRequest<Fragment>,
    ) -> ::grpc::StreamingResponse<Fragment> {
        println!("content_subscription");
        ::grpc::StreamingResponse::empty()
    }

    fn gossip_subscription(
        &self,
        _o: ::grpc::RequestOptions,
        _p: ::grpc::StreamingRequest<Gossip>,
    ) -> ::grpc::StreamingResponse<Gossip> {
        println!("gossip_subscription");
        ::grpc::StreamingResponse::completed_with_metadata_and_trailing_metadata(
            get_metadata(),
            iter::from_fn(|| None).collect(),
            get_metadata(),
        )
    }
}

fn get_metadata() -> Metadata {
    let mut metadata = Metadata::new();
    metadata.add(
        MetadataKey::from("node-id-bin"),
        Bytes::from(&b"6266663338323161373465336631353966333466643463383865633233653664"[..]),
    );
    metadata
}
