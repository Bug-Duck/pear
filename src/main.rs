#![allow(unused_variables)]

use log::{info, warn};
use env_logger::Env;
use anyhow::Result;
use pear_service::{PearService, match_event};
use libp2p::{core::transport::ListenerId, swarm::SwarmEvent, Multiaddr, mdns};
use pear_service::*;
use macros::handler;

pub struct Started {
  pub listener_id: ListenerId,
  pub address: Multiaddr,
}

pub struct MdnsEvent(pub mdns::Event);

match_event! {
  { Started, SwarmEvent::NewListenAddr { listener_id, address } => Started { listener_id, address } },
  { MdnsEvent, SwarmEvent::Behaviour(PearBehaviourEvent::Mdns(target)) => MdnsEvent(target) },
}

#[handler]
async fn handle_started(event: Started, state: PearContext) {
  info!("Listening on : {}", event.address);
}

#[handler]
async fn handle_mdns(event: MdnsEvent, state: PearContext) {
  match event.0 {
    mdns::Event::Discovered(peers) => {
      for (peer, addr) in &peers {
        info!("Found peer : {} on {}", peer, addr)
      }
    }
    mdns::Event::Expired(peers) => {
      for (peer, addr) in &peers {
        warn!("Peer expired : {} on {}", peer, addr)
      }
    },
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  PearService::init_service()
    .await?
    .with_handler(handle_started_service)
    .with_handler(handle_mdns_service)
    .run()
    .await
}
