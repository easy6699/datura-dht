use crate::{
    client,
    identity::NodeId,
    records::HSRecord,
    records::NatRecord,
    records::Record,
    rpc::Message,
    routing::Peer,
};

pub async fn find_closest(
    bootstrap: Peer,
    target: NodeId,
) -> Vec<Peer>
{
    match client::rpc(
        &bootstrap.addr.to_string(),
        Message::FindNode {
            target,
        },
    )
    .await
    {
        Some(
            Message::Nodes {
                peers,
            },
        ) => peers,

        _ => vec![],
    }
}

pub async fn publish_nat_record(
    bootstrap: Peer,
    record: NatRecord,
)
{
    let peers =
        find_closest(
            bootstrap,
            record.owner,
        )
        .await;

    for peer in peers {

        let _ =
            client::rpc(
                &peer.addr.to_string(),
                Message::Store {
                    key: record.owner,
                    record: Record::Nat(
                        record.clone(),
                    ),
                },
            )
            .await;
    }
}

pub async fn resolve_nat_record(
    bootstrap: Peer,
    node_id: NodeId,
) -> Option<NatRecord>
{
    let peers =
        find_closest(
            bootstrap,
            node_id,
        )
        .await;

    for peer in peers {

        if let Some(
            Message::Value {
                record:
                    Some(
                        Record::Nat(r),
                    ),
            },
        ) =
            client::rpc(
                &peer.addr.to_string(),
                Message::FindValue {
                    key: node_id,
                },
            )
            .await
        {
            return Some(r);
        }
    }

    None
}

pub async fn publish_hs_descriptor(
    bootstrap: Peer,
    record: HSRecord,
)
{
    let peers =
        find_closest(
            bootstrap,
            record.hs_hash,
        )
        .await;

    for peer in peers {

        let _ =
            client::rpc(
                &peer.addr.to_string(),
                Message::Store {
                    key: record.hs_hash,
                    record:
                        Record::HiddenService(
                            record.clone(),
                        ),
                },
            )
            .await;
    }
}

pub async fn resolve_hs_descriptor(
    bootstrap: Peer,
    hs_hash: NodeId,
) -> Option<HSRecord>
{
    let peers =
        find_closest(
            bootstrap,
            hs_hash,
        )
        .await;

    for peer in peers {

        if let Some(
            Message::Value {
                record:
                    Some(
                        Record::HiddenService(
                            desc,
                        ),
                    ),
            },
        ) =
            client::rpc(
                &peer.addr.to_string(),
                Message::FindValue {
                    key: hs_hash,
                },
            )
            .await
        {
            return Some(desc);
        }
    }

    None
}

