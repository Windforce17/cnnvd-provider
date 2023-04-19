use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::Instrument;
#[derive(Serialize, Deserialize)]
pub struct Welcome2 {
    #[serde(rename = "cnnvd")]
    cnnvd: Cnnvd,
}

#[derive(Serialize, Deserialize)]
pub struct Cnnvd {
    #[serde(rename = "entry")]
    entry: Vec<Entry>,

    #[serde(rename = "_xmlns:xsi")]
    xmlns_xsi: String,

    #[serde(rename = "_cnnvd_xml_version")]
    cnnvd_xml_version: String,

    #[serde(rename = "_pub_date")]
    pub_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "vuln-id")]
    vuln_id: String,

    #[serde(rename = "published")]
    published: String,

    #[serde(rename = "modified")]
    modified: String,

    #[serde(rename = "source")]
    source: String,

    #[serde(rename = "severity")]
    severity: String,

    #[serde(rename = "vuln-type")]
    vuln_type: String,

    #[serde(rename = "vuln-descript")]
    vuln_descript: String,

    #[serde(rename = "other-id")]
    other_id: OtherId,

    #[serde(rename = "vuln-solution")]
    vuln_solution: String,
}

#[derive(Serialize, Deserialize)]
pub struct OtherId {
    #[serde(rename = "cve-id")]
    cve_id: String,

    #[serde(rename = "bugtraq-id")]
    bugtraq_id: String,
}

#[tokio::main]
async fn main() {
    let mut c = tokio::process::Command::new("./slow.bash")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = c.stdout.take().unwrap();
    let buf = BufReader::new(stdout);
    let result: Vec<_> = buf
        .buffer()
        .lines()
        .get_ref()
        .iter()
        .inspect(|s| println!("> {:?}", s))
        .collect();

    println!("All the lines: {:?}", result);
    println!("Hello, world!");
}
