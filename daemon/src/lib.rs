mod command;
pub mod connect;

use anyhow::{anyhow, Result};
use command::*;
use dirs::data_local_dir;
use futures::StreamExt;
use libp2p::{
    identity::Keypair,
    kad::{
        self, store::MemoryStore, Behaviour as KadBehaviour, Config as KadConfig, GetRecordOk,
        Quorum, Record, RecordKey,
    },
    mdns::{self, tokio::Tokio, Behaviour as MdnsBehaviour, Event as MdnsEvent},
    multiaddr::Protocol,
    request_response::{json::Behaviour as ResqBehaviour, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    tls::Config as TlsConfig,
    yamux::Config as YamuxConfig,
    Multiaddr, PeerId, StreamProtocol, Swarm, SwarmBuilder,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    str,
    collections::{hash_map, HashMap},
    error::Error,
    ops::{Deref, DerefMut},
    sync::Arc,
    time::Duration,
};
use tokio::{
    fs,
    sync::{mpsc, oneshot, RwLock},
};

#[derive(Serialize, Deserialize, Debug)]
pub enum PearReq {
    /// Try connecting to a peer
    Connect(PeerId, Multiaddr),
}

#[derive(Serialize, Deserialize)]
pub enum Commands {
    GetUser,
    Login,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PearRes {
    /// Received connecting request, true for accept, false for refuse
    Connect(bool),
}

#[derive(NetworkBehaviour)]
pub struct PearBehaviour {
    pub resq: ResqBehaviour<PearReq, PearRes>,
    // FIXME: `MemoryStore` just for now, should be replaced with a custom store
    pub kad: KadBehaviour<MemoryStore>,
    pub mdns: MdnsBehaviour<Tokio>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub keypair: Vec<u8>,
    pub uid: String,
}

pub struct PearServiceInner {
    pub swarm: Swarm<PearBehaviour>,
    pub config: Config,
    pub command_channel_tx: mpsc::Sender<Command>,
    command_channel_rx: mpsc::Receiver<Command>,
    command_response_channel_tx: mpsc::Sender<CommandResp>,
    pub command_response_channel_rx: mpsc::Receiver<CommandResp>,

    pending_dht: HashMap<String, oneshot::Sender<Result<Multiaddr, Box<dyn Error + Send>>>>,
    pending_dial: HashMap<Multiaddr, oneshot::Sender<Result<(), Box<dyn Error + Send>>>>,
}

#[derive(Clone)]
pub struct PearService(pub Arc<RwLock<PearServiceInner>>);

impl Deref for PearService {
    type Target = Arc<RwLock<PearServiceInner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PearService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub async fn init_service() -> Result<PearService, Box<dyn Error>> {
    // INFO: the "" here would simply be replaced by a user name passed from the frontend user
    let config = PearService::get_config("".to_string()).await?;

    // FIXME: the `with_new_identity` must be replaced for security and business logic
    let swarm =
        SwarmBuilder::with_existing_identity(Keypair::from_protobuf_encoding(&config.keypair)?)
            .with_tokio()
            .with_tcp(Default::default(), TlsConfig::new, YamuxConfig::default)?
            .with_behaviour(|key| {
                let mut kad_config = KadConfig::default();
                kad_config.set_record_ttl(None);
                kad_config.set_query_timeout(Duration::from_secs(5));

                let kad_store = MemoryStore::new(key.public().to_peer_id());
                let kad =
                    KadBehaviour::with_config(key.public().to_peer_id(), kad_store, kad_config);

                let resq = ResqBehaviour::new(
                    [(StreamProtocol::new("/pear"), ProtocolSupport::Full)],
                    Default::default(),
                );

                let mdns = MdnsBehaviour::new(Default::default(), key.public().to_peer_id())?;

                Ok(PearBehaviour { resq, kad, mdns })
            })?
            .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(5)))
            .build();

    let (tx, mut rx) = mpsc::channel(32);
    let (res_tx, mut res_rx) = mpsc::channel(32);

    let inner = Arc::new(RwLock::new(PearServiceInner {
        swarm,
        config,
        command_channel_rx: rx,
        command_channel_tx: tx,
        command_response_channel_rx: res_rx,
        command_response_channel_tx: res_tx,

        pending_dht: Default::default(),
        pending_dial: Default::default(),
    }));

    Ok(PearService(inner))
}

pub fn is_private_network(addr: &Multiaddr) -> bool {
    if let Some(Protocol::Ip4(ip)) = addr.iter().next() {
        if ip.is_private() {
            return true;
        }

        return match ip.octets() {
            [100, b, ..] if b >= 64 && b <= 127 => true,
            [127, 0, 0, 1] => true,
            _ => false,
        };
    }

    false
}

impl PearService {
    pub async fn get_config(uid: String) -> Result<Config> {
        let data_dir = data_local_dir()
            .ok_or_else(|| anyhow!("failed to read the specified data directory!"))?;

        let key_path = data_dir.join("private.key");
        let uid_path = data_dir.join("uid");

        if data_dir.exists() && key_path.exists() {
            let key_bytes = fs::read(key_path).await?;
            let uid_bytes = fs::read(uid_path).await?;
            let config = Config {
                keypair: key_bytes,
                uid: String::from_utf8(uid_bytes)?,
            };

            Ok(config)
        } else {
            fs::create_dir_all(data_dir).await?;
            let keypair = Keypair::generate_ed25519();
            let key_bytes = keypair.to_protobuf_encoding()?;

            fs::write(key_path, key_bytes.clone()).await?;
            fs::write(uid_path, uid.as_bytes()).await?;

            Ok(Config {
                keypair: key_bytes,
                uid,
            })
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.write()
            .await
            .swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        loop {
            let event = self.write().await.swarm.select_next_some().await;
            self.handle_event(event).await;
        }
    }

    async fn handle_event(&mut self, event: SwarmEvent<PearBehaviourEvent>) {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {address:?}");
                if !is_private_network(&address) {
                    info!("putting {address:?} to dht");
                    let addr_record = Record::new(
                        RecordKey::new(&self.read().await.config.uid),
                        address.to_vec(),
                    );
                    let res = self
                        .write()
                        .await
                        .swarm
                        .behaviour_mut()
                        .kad
                        .put_record(addr_record, Quorum::One);
                    if let Err(e) = res {
                        error!("error putting {address:?} to dht: {e}")
                    }
                }
            }
            // mdns events
            SwarmEvent::Behaviour(PearBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    println!("mDNS discovered a new peer: {peer_id}");
                    self.write()
                        .await
                        .swarm
                        .behaviour_mut()
                        .kad
                        .add_address(&peer_id, multiaddr);
                }
            }
            // dht events
            SwarmEvent::Behaviour(PearBehaviourEvent::Kad(
                kad::Event::OutboundQueryProgressed {
                    result: kad::QueryResult::GetRecord(Ok(GetRecordOk::FoundRecord(record))),
                    ..
                },
            )) => {
                info!("get record from dht: {:?}", record);
                let uid = String::from_utf8(record.record.key.to_vec()).unwrap();
                let multiaddr = Multiaddr::try_from(record.record.value).unwrap();
                if let Some(sender) = self.write().await.pending_dht.remove(&uid) {
                    sender.send(Ok(multiaddr)).unwrap();
                }
            }
            SwarmEvent::Behaviour(PearBehaviourEvent::Kad(
                kad::Event::OutboundQueryProgressed {
                    result: kad::QueryResult::GetRecord(Err(err)),
                    ..
                },
            )) => {
                error!("Failed to GetRecord: {}", err);
                if let Some(sender) = self.write().await.pending_dht.remove(str::from_utf8(&err.key().to_vec()).unwrap()) {
                    sender.send(Err(Box::new(err))).unwrap();
                }
            }
            // connection events
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                if endpoint.is_dialer() {
                    if let Some(sender) = self
                        .write()
                        .await
                        .pending_dial
                        .remove(endpoint.get_remote_address())
                    {
                        let _ = sender.send(Ok(()));
                    }
                }
            }
            SwarmEvent::Behaviour(event) => info!("event: {event:?}"),
            _ => {}
        }
    }

    async fn handle_command(&mut self, command: Command) {
        match command {
            Command::GetUser { uid } => {
                self.write()
                    .await
                    .swarm
                    .behaviour_mut()
                    .kad
                    .get_record(kad::RecordKey::new(&uid));
                let (sender, receiver) = oneshot::channel();
                if let hash_map::Entry::Vacant(e) =
                    self.write().await.pending_dht.entry(uid.clone())
                {
                    e.insert(sender);
                } else {
                    // duplicated command
                    return;
                }
                let (sender_dial, receiver_dial) = oneshot::channel();
                match receiver.await {
                    Ok(addr_res) => {
                        if let Ok(addr) = addr_res {
                            if let Err(err) = self.write().await.swarm.dial(addr.clone()) {
                                error!("{}", err);
                            }
                            if let hash_map::Entry::Vacant(e) =
                                self.write().await.pending_dial.entry(addr)
                            {
                                e.insert(sender_dial);
                            }
                        } else {
                            return;
                        }
                    }
                    Err(err) => {
                        error!("{}", err);
                        self.read().await.command_response_channel_tx.send(
                            CommandResp::GetUserResp {
                                uid: uid,
                                exists: false,
                                connected: false,
                            },
                        ).await.unwrap();
                        return;
                    }
                }

                match receiver_dial.await {
                    Ok(_) => {
                        self.read().await.command_response_channel_tx.send(
                            CommandResp::GetUserResp {
                                uid: uid,
                                exists: true,
                                connected: true,
                            },
                        ).await.unwrap();
                    }
                    Err(err) => {
                        error!("{}", err);
                        return;
                    }
                }
            }
        }
    }
}
