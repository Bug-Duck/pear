use std::time::Duration;

use anyhow::Ok;
use dirs;
use futures::StreamExt;
use libp2p::kad::{Behaviour, store::MemoryStore};
use libp2p::multiaddr::Protocol;
use libp2p::Multiaddr;
use tokio::fs;
use anyhow::{anyhow, Result};
use libp2p::swarm::SwarmEvent;
use libp2p::{identity, kad, noise, tcp, yamux, Swarm};

pub struct DHT {
    swarm: Swarm<Behaviour<MemoryStore>>,
    config: Config,
}

#[derive(Clone)]
struct Config {
    keypair: identity::Keypair,
    uid: String,
}

async fn get_config(uid: String) -> Result<Config> {
    let mut data_dir = dirs::data_dir().ok_or(anyhow!("failed to get home dir"))?;
    data_dir.push("pear");
    let key_path = data_dir.join("pravite.key");
    let uid_path = data_dir.join("uid");
    // FIXME: blocking io?
    if data_dir.exists() && key_path.exists() {
        info!("reading keypair in {}", key_path.to_string_lossy());
        let key_bytes = fs::read(key_path).await?;
        let uid_bytes = fs::read(uid_path).await?;
        let config = Config{
            keypair: identity::Keypair::from_protobuf_encoding(&key_bytes)?,
            uid: String::from_utf8(uid_bytes)?,
        };
        return Ok(config)
    }

    // Create a random key for ourselves.
    info!("creating keypair in {}", key_path.to_string_lossy());
    fs::create_dir_all(data_dir).await?;
    let keypair = identity::Keypair::generate_ed25519();
    let key_bytes = keypair.to_protobuf_encoding()?;
    fs::write(key_path, key_bytes).await?;
    fs::write(uid_path, uid.as_bytes()).await?;

    Ok(Config{
        keypair,
        uid,
    })
}

pub async fn init_dht(uid: String) -> Result<DHT> {
    let config = get_config(uid).await?;
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(config.clone().keypair)
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

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    Ok(DHT {
        swarm: swarm,
        config: config,
    })
}

fn is_pravite(address: &Multiaddr) -> bool {
    if let Some(Protocol::Ip4(ip)) = address.iter().next() {
        if ip.is_private() {
            return true
        }
        // filter out nat addrs, ref: https://en.wikipedia.org/wiki/Private_network
        return match ip.octets() {
            [100, b, ..] if b >= 64 && b <= 127 => true,
            [127, 0, 0, 1] => true,
            _ => false,
        }
    }

    false
}

impl DHT {
    pub async fn main_loop(&mut self) {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address:?}");
                    if !is_pravite(&address) {
                        info!("putting {address:?} to dht");
                        let addr_record = kad::Record::new(kad::RecordKey::new(&self.config.uid), address.to_vec());
                        let res = self.swarm
                            .behaviour_mut()
                            .put_record(addr_record, kad::Quorum::One);
                        if let Err(e) = res {
                            error!("error putting {address:?} to dht: {e}")
                        }
                    }
                },
                SwarmEvent::Behaviour(event) => println!("event: {event:?}"),
                SwarmEvent::ExternalAddrConfirmed { address } => println!("address: {address:?}"),
                _ => {}
            }
        }
    }
}
