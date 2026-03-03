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
    #[serde(default)]
    pub unpacked: Option<Value>,
    pub packet: Option<String>,
    pub expected_error: Option<String>,
}
