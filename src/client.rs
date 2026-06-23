use tokio::net::UdpSocket;

use crate::rpc::Message;

pub async fn rpc(
    destination: &str,
    msg: Message,
) -> Option<Message> {

    let socket =
        UdpSocket::bind("0.0.0.0:0")
            .await
            .ok()?;

    let bytes =
        serde_json::to_vec(&msg).ok()?;

    socket
        .send_to(&bytes, destination)
        .await
        .ok()?;

    let mut buffer = [0u8; 4096];

    let (size, _) =
        socket.recv_from(&mut buffer)
            .await
            .ok()?;

    serde_json::from_slice(
        &buffer[..size]
    )
    .ok()
}