use anyhow::{anyhow, Result};
use dirs::data_local_dir;
use futures::StreamExt;
use libp2p::{
    identity::Keypair, kad::{
        store::MemoryStore, Behaviour as KadBehaviour, Config as KadConfig, Quorum, Record,
        RecordKey,
    }, mdns, multiaddr::Protocol, request_response::{json::Behaviour as ResqBehaviour, ProtocolSupport}, swarm::{NetworkBehaviour, SwarmEvent}, tls::Config as TlsConfig, yamux::Config as YamuxConfig, Multiaddr, StreamProtocol, Swarm, SwarmBuilder
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{error::Error, time::Duration};
use tokio::fs;

#[derive(Serialize, Deserialize, Debug)]
pub enum PearReq {}
#[derive(Serialize, Deserialize, Debug)]
pub enum PearRes {}

#[derive(NetworkBehaviour)]
pub struct PearBehaviour {
    pub resq: ResqBehaviour<PearReq, PearRes>,
    // FIXME: `MemoryStore` just for now, should be replaced with a custom store
    pub kad: KadBehaviour<MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub keypair: Vec<u8>,
    pub uid: String,
}

pub struct PearService {
    pub swarm: Swarm<PearBehaviour>,
    pub config: Config,
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
                kad_config.set_query_timeout(Duration::from_secs(5 * 60));

                let kad_store = MemoryStore::new(key.public().to_peer_id());
                let kad =
                    KadBehaviour::with_config(key.public().to_peer_id(), kad_store, kad_config);

                let resq = ResqBehaviour::new(
                    [(StreamProtocol::new("/pear"), ProtocolSupport::Full)],
                    Default::default(),
                );

                let mdns =
                    mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

                Ok(PearBehaviour { resq, kad, mdns })
            })?
            .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(5)))
            .build();

    Ok(PearService { swarm, config })
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
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        loop {
            let event = self.swarm.select_next_some().await;
            self.handle_event(event).await;
        }
    }

    async fn handle_event(&mut self, event: SwarmEvent<PearBehaviourEvent>) {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {address:?}");
                if !is_private_network(&address) {
                    info!("putting {address:?} to dht");
                    let addr_record =
                        Record::new(RecordKey::new(&self.config.uid), address.to_vec());
                    let res = self
                        .swarm
                        .behaviour_mut().kad
                        .put_record(addr_record, Quorum::One);
                    if let Err(e) = res {
                        error!("error putting {address:?} to dht: {e}")
                    }
                }
            }
            SwarmEvent::Behaviour(PearBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    println!("mDNS discovered a new peer: {peer_id}");
                    self.swarm.behaviour_mut().kad.add_address(&peer_id, multiaddr);
                }
            },
            SwarmEvent::Behaviour(event) => info!("event: {event:?}"),
            _ => {}
        }
    }
}
