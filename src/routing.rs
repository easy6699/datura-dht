use crate::identity::NodeId;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Peer {
    pub id: NodeId,
    pub addr: SocketAddr,
}

pub struct RoutingTable {
    peers: Vec<Peer>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self { peers: vec![] }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        if !self.peers.iter().any(|p| p.id == peer.id) {
            self.peers.push(peer);
        }
    }

    pub fn all(&self) -> Vec<Peer> {
        self.peers.clone()
    }

    pub fn closest(
        &self,
        target: NodeId,
        count: usize,
    ) -> Vec<Peer> {

        let mut peers = self.peers.clone();

        peers.sort_by_key(|p| xor_distance(&p.id, &target));

        peers.into_iter()
            .take(count)
            .collect()
    }
}

fn xor_distance(
    a: &NodeId,
    b: &NodeId,
) -> [u8; 32] {

    let mut out = [0u8; 32];

    for i in 0..32 {
        out[i] = a[i] ^ b[i];
    }

    out
}