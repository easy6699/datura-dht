mod identity;
mod routing;
mod records;
mod storage;
mod rpc;
mod network;
mod bootstrap;
mod client;
mod dht;

use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};

use identity::Identity;
use routing::RoutingTable;
use storage::Storage;
use routing::Peer;
use records::NatRecord;
use records::HSRecord;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let node = Identity::new();

    println!(
        "Node ID: {:x?}",
        node.node_id
    );


    let routing =
        Arc::new(
            Mutex::new(
                RoutingTable::new()
            )
        );


    let storage =
        Arc::new(
            Mutex::new(
                Storage::new()
            )
        );

    let args: Vec<String> =
        std::env::args().collect();

    let mode =
        args.get(1)
            .map(String::as_str)
            .unwrap_or("node");

    let bind =
        args
            .get(2)
            .cloned()
            .unwrap_or(
                "127.0.0.1:9000".into()
            );

    let bootstrap =
        args.get(3).cloned();

    match mode {

        "node" => {
            // existing server code
        }

        "publish-nat" => {

            let bootstrap =
                args[2].clone();

            let identity = Identity::new();
            let owner = identity.node_id;

            let mut gateway = [0u8; 32];
            gateway[0] = 99;

            let record = NatRecord {
                owner,
                gateway,
                external_address:
                    "203.0.113.10:5555"
                        .to_string(),
                timestamp: 123456789,
            };

            let bootstrap_peer = Peer {
                id: [0u8;32],
                addr: bootstrap.parse()?,
            };

            dht::publish_nat_record(
                bootstrap_peer,
                record,
            )
            .await;

            println!("published NAT record");
        }

        "resolve-nat" => {

            let bootstrap =
                args[2].clone();

            let mut owner = [0u8; 32];
            owner[0] = 42;

            let bootstrap_peer = Peer {
                id: [0u8;32],
                addr: bootstrap.parse()?,
            };

            let result =
                dht::resolve_nat_record(
                    bootstrap_peer,
                    owner,
                )
                .await;

            println!("{:#?}", result);
        }

        "publish-hs" => {

            let bootstrap =
                args[2].clone();

            let hs_addr = "my-hidden-service.dn";
            let hs_hash = Sha256::digest(hs_addr.as_bytes()).into();

            let mut rendezvous = [0u8;32];
            rendezvous[0] = 88;

            let descriptor = HSRecord {
                hs_hash,
                rendezvous,
                expires: 9999999999,
            };

            let bootstrap_peer = Peer {
                id: [0u8;32],
                addr: bootstrap.parse()?,
            };

            dht::publish_hs_descriptor(
                bootstrap_peer,
                descriptor,
            )
            .await;

            println!("published HS descriptor");
        }

        "resolve-hs" => {

            let bootstrap =
                args[2].clone();

            let mut hs_hash = [0u8;32];
            hs_hash[0] = 7;

            let bootstrap_peer = Peer {
                id: [0u8;32],
                addr: bootstrap.parse()?,
            };

            let result =
                dht::resolve_hs_descriptor(
                    bootstrap_peer,
                    hs_hash,
                )
                .await;

            println!("{:#?}", result);
        }

        _ => {}
    }

    let me = Peer {
        id: node.node_id,
        addr: bind.parse()?,
    };

    if let Some(addr) = bootstrap {

        bootstrap::join_network(
            addr,
            me,
            routing.clone(),
        )
        .await;
    }

    network::run_server(
        &bind,
        node.node_id,
        routing.clone(),
        storage.clone(),
    )
    .await?;

    Ok(())
}