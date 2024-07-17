use anyhow::{anyhow, Result};
use dirs::data_local_dir;
use libp2p::{
    identity::Keypair,
    kad::{store::MemoryStore, Behaviour as KadBehaviour, Config as KadConfig},
    request_response::{json::Behaviour as ResqBehaviour, ProtocolSupport},
    swarm::NetworkBehaviour,
    tls::Config as TlsConfig,
    yamux::Config as YamuxConfig,
    StreamProtocol, Swarm, SwarmBuilder,
};
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

                Ok(PearBehaviour { resq, kad })
            })?
            .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(5)))
            .build();

    Ok(PearService { swarm, config })
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
}
