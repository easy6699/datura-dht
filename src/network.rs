use std::{
    sync::{Arc, Mutex},
};

use tokio::net::UdpSocket;

use crate::{
    identity::NodeId,
    rpc::Message,
    routing::RoutingTable,
    storage::Storage,
    routing::Peer,
};


pub async fn run_server(
    bind_addr: &str,
    my_id: NodeId,
    routing: Arc<Mutex<RoutingTable>>,
    storage: Arc<Mutex<Storage>>,
) -> Result<(), Box<dyn std::error::Error>> {

    let socket = UdpSocket::bind(bind_addr).await?;

    println!("Listening on {}", bind_addr);

    let mut buffer = [0u8; 4096];


    loop {

        let (size, sender) =
            socket.recv_from(&mut buffer).await?;

        let msg: Message =
            serde_json::from_slice(&buffer[..size])?;


        println!("Received {:?} from {}", msg, sender);


        let reply = match msg {

            Message::Ping => {

                Some(
                    Message::Pong {
                        id: my_id
                    }
                )
            }


            Message::FindNode { target } => {

                let peers =
                    routing
                    .lock()
                    .unwrap()
                    .closest(target, 8);

                Some(
                    Message::Nodes {
                        peers
                    }
                )
            }


            Message::Store {
                key,
                record,
            } => {

                storage
                    .lock()
                    .unwrap()
                    .put(key, record);

                None
            }


            Message::FindValue { key } => {

                let value =
                    storage
                    .lock()
                    .unwrap()
                    .get(&key)
                    .cloned();

                Some(
                    Message::Value {
                        record: value
                    }
                )
            }

            Message::Hello { peer } => {

                let my_peer = Peer {
                    id: my_id,
                    addr: socket.local_addr()?,
                };

                routing
                    .lock()
                    .unwrap()
                    .add_peer(peer.clone());

                Some(
                    Message::HelloAck {
                        peer: my_peer.clone(),
                    }
                )
            }


            _ => None,
        };


        if let Some(reply) = reply {

            let bytes =
                serde_json::to_vec(&reply)?;

            socket
                .send_to(
                    &bytes,
                    sender,
                )
                .await?;
        }
    }
}