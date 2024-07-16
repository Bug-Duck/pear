use std::time::Duration;

use anyhow::Ok;
use dirs;
use tokio::fs;
use anyhow::{anyhow, Result};
use libp2p::swarm::{NetworkBehaviour, StreamProtocol, SwarmEvent};
use libp2p::{bytes::BufMut, identity, kad, noise, tcp, yamux, PeerId, Swarm};

pub struct dht {
    swarm: Swarm<dyn NetworkBehaviour>,
}

pub async fn get_keypair() -> Result<identity::Keypair> {
    let mut data_dir = dirs::data_dir().ok_or(anyhow!("failed to get home dir"))?;
    data_dir.push("pear");
    let key_path = data_dir.join("pravite.key");
    // FIXME: blocking io?
    if data_dir.exists() && key_path.exists() {
        info!("reading keypair in {}", key_path.to_string_lossy());
        let key_bytes = fs::read(key_path).await?;
        return Ok(identity::Keypair::from_protobuf_encoding(&key_bytes)?)
    }

    // Create a random key for ourselves.
    info!("creating keypair in {}", key_path.to_string_lossy());
    fs::create_dir_all(data_dir).await?;
    let key_pair = identity::Keypair::generate_ed25519();
    let key_bytes = key_pair.to_protobuf_encoding()?;
    fs::write(key_path, key_bytes).await?;

    Ok(key_pair)
}

async fn init_dht() -> Result<dht> {
    let local_key = get_keypair().await?;
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_dns()?
        .with_behaviour(|key| {
            // Create a Kademlia behaviour.
            // FIXME: the new function is not released
            // let mut cfg = kad::Config::new(StreamProtocol::new("/pear/kad/1.0"));
            let mut cfg = kad::Config::default();
            cfg.set_query_timeout(Duration::from_secs(5 * 60));
            cfg.set_record_ttl(Some(Duration::from_secs(60 * 60 * 24 * 365))); // 1 year
            let store = kad::store::MemoryStore::new(key.public().to_peer_id());
            kad::Behaviour::with_config(key.public().to_peer_id(), store, cfg)
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(5)))
        .build();

        Ok(dht {
            swarm: swarm,
        })
}
