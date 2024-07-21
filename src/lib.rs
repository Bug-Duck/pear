#![allow(unused_variables)]

use std::{
  io::BufReader,
  ops::{Deref, DerefMut},
  sync::Arc,
};

use log::{info, warn};
use anyhow::Result;
use dirs::data_local_dir;
use futures::{future::BoxFuture, StreamExt};
use libp2p::*;
use libp2p::{swarm::NetworkBehaviour, Swarm};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use swarm::SwarmEvent;
use tokio::{
  fs::{read_to_string, write},
  select,
  sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    RwLock,
  },
};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct PearConfig {
  pub uid : String,
  pub keypair : Vec<u8>,
}

impl PearConfig {
  pub async fn load_config() -> Result<Self> {
    // TODO: provide a available data directory
    let data_dir = data_local_dir().unwrap_or_default();
    let config_file = data_dir.join(".pear.json");

    info!("Reading data file from : {}", config_file.to_string_lossy());

    let result = read_to_string(config_file.clone()).await;

    if let Ok(content) = result {
      let rdr = BufReader::new(content.as_bytes());
      let config : Self = serde_json::from_reader(rdr)?;

      Ok(config)
    } else {
      write(config_file.clone(), to_string_pretty(&PearConfig::default()).unwrap()).await.unwrap();
      warn!("Found no data file, writing in : {}", config_file.to_string_lossy());

      Ok(Default::default()) }
  }
}

#[derive(NetworkBehaviour)]
pub struct PearBehaviour {
  pub mdns : mdns::tokio::Behaviour,
  pub kad : kad::Behaviour<kad::store::MemoryStore>,
}

pub struct PearState {
  pub config : PearConfig,
  pub spawner : UnboundedSender<PearCallback>,
}

unsafe impl Send for PearState {}
unsafe impl Sync for PearState {}

pub type InnerType = Arc<RwLock<PearState>>;

#[derive(Clone)]
pub struct PearContext {
  inner : InnerType,
}

impl PearContext {
  pub fn new(inner : PearState) -> Self {
    Self {
      inner : Arc::new(RwLock::new(inner)),
    }
  }
}

impl Deref for PearContext {
  type Target = InnerType;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl DerefMut for PearContext {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

pub trait MatchEvent: Sized {
  fn try_match(event : &SwarmEvent<PearBehaviourEvent>) -> bool;
  fn extract_from(event : SwarmEvent<PearBehaviourEvent>) -> Option<Self>;
}

#[macro_export]
macro_rules! match_event {
  { $({ $name:ident, $cond:pat => $custom:expr },)+ } => {
    $(
      impl MatchEvent for $name {
        fn try_match(event: &SwarmEvent<PearBehaviourEvent>) -> bool {
          match event {
            $cond => true,
            _ => false
          }
        }
        fn extract_from(event: SwarmEvent<PearBehaviourEvent>) -> Option<Self> {
          match event {
            $cond => Some($custom),
            _ => None,
          }
        }
      }
    )+
  }
}

pub trait Handler {
  fn match_event(&self, event : &SwarmEvent<PearBehaviourEvent>) -> bool;
  fn handle(&self, event : SwarmEvent<PearBehaviourEvent>, state : PearContext) -> BoxFuture<()>;
}

type PearCallback = Box<dyn FnOnce(&mut PearService) + 'static>;
pub struct PearService {
  pub handlers : Vec<Box<dyn Handler + 'static>>,
  pub context : PearContext,
  pub swarm : Swarm<PearBehaviour>,
  pub spawner_r : UnboundedReceiver<PearCallback>,
}

impl PearService {
  pub async fn init_service() -> Result<PearService> {
    // FIXME: with_new_identty() should be replaced
    let swarm = SwarmBuilder::with_new_identity()
      .with_tokio()
      .with_tcp(
        tcp::Config::default(),
        noise::Config::new,
        yamux::Config::default,
      )?
      .with_behaviour(|key| {
        let mdns = mdns::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
        let kad = kad::Behaviour::new(
          key.public().to_peer_id(),
          kad::store::MemoryStore::new(key.public().to_peer_id()),
        );

        Ok(PearBehaviour { mdns, kad })
      })?
      .build();
    let (s, r) = unbounded_channel();
    let state = PearState {
      config : PearConfig::load_config().await?,
      spawner : s,
    };
    let context = PearContext::new(state);
    let service = PearService {
      handlers : vec![],
      context,
      swarm,
      spawner_r : r,
    };

    Ok(service)
  }

  pub fn with_handler(mut self, handler : impl Handler + 'static) -> Self {
    self.handlers.push(Box::new(handler));

    self
  }

  pub async fn run(mut self) -> Result<()> {

    self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?).unwrap();

    loop {
      select! {
        event = self.swarm.select_next_some() => {
          let handler = self.handlers.iter().find(|h| h.match_event(&event));
          if let Some(handler) = handler {
            handler.handle(event, self.context.clone()).await;
          }
        },
        Some(task) = self.spawner_r.recv() => {
          task(&mut self);
        }
      }
    }
  }
}
