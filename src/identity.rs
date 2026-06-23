use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};

pub type NodeId = [u8; 32];

pub struct Identity {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub node_id: NodeId,
}

impl Identity {
    pub fn new() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);

        let signing_key = SigningKey::from_bytes(&bytes);
        let verifying_key: VerifyingKey = signing_key.verifying_key();

        let mut hasher = Sha256::new();
        hasher.update(verifying_key.as_bytes());

        let hash = hasher.finalize();

        let mut node_id = [0u8; 32];
        node_id.copy_from_slice(&hash);

        Self {
            signing_key,
            verifying_key,
            node_id,
        }
    }
}