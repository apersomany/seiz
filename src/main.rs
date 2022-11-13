use std::{
    net::{IpAddr, TcpStream},
    time::Instant,
};

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct Node {
    #[serde(
        rename = "commercial_node",
        deserialize_with = "deserialize_commercial_node"
    )]
    commercial: bool,
    hostname: String,
    location: String,
    ip: IpAddr,
    #[serde(rename = "sid")]
    id: u64,
}

fn deserialize_commercial_node<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    Ok(u8::deserialize(deserializer)? == 1)
}

#[derive(Debug, Deserialize)]
struct StaticNodes {
    #[serde(rename = "staticnodes")]
    nodes: Vec<Node>,
}

fn main() {
    let StaticNodes { nodes } = ureq::get("https://mudfish.net/api/staticnodes")
        .call()
        .unwrap()
        .into_json::<StaticNodes>()
        .unwrap();
    let nodes = nodes
        .into_iter()
        .filter(|node| node.location.contains("Korea"))
        .collect::<Vec<_>>();
    for node in nodes {
        println!("Pinging node {} ({} / {})", node.id, node.ip, node.location);
        for _ in 0..4 {
            let now = Instant::now();
            TcpStream::connect((node.ip, 18081)).unwrap();
            println!("{:?}", now.elapsed());
        }
    }
}
