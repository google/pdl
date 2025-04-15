use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Packet {
    #[serde(rename = "packet")]
    pub name: String,
    pub tests: Vec<TestVector>,
}

#[derive(Debug, Deserialize)]
pub struct TestVector {
    pub packed: String,
    pub unpacked: Value,
    pub packet: Option<String>,
}
