use super::helpers::{buffer_to_hex, hex_to_buffer, option_buffer_to_hex, option_hex_to_buffer};
use super::value::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct UnknownAction {
  pub code: u8,
  #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
  pub data: Vec<u8>,
}

// Action code 0x88
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ConstantPool {
  pub constant_pool: Vec<String>,
}

// Action code 0x9b
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct DefineFunction {
  // Empty string if anonymous
  pub name: String,
  pub parameters: Vec<String>,
  #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
  pub body: Vec<u8>,
}

pub mod define_function2 {
  #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
  #[serde(rename_all = "snake_case")]
  pub struct Parameter {
    pub register: u8,
    pub name: String,
  }
}

// Action code 0x8e
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct DefineFunction2 {
  // Empty string if anonymous
  pub name: String,
  pub preload_parent: bool,
  pub preload_root: bool,
  pub suppress_super: bool,
  pub preload_super: bool,
  pub suppress_arguments: bool,
  pub preload_arguments: bool,
  pub suppress_this: bool,
  pub preload_this: bool,
  pub preload_global: bool,
  pub register_count: usize,
  pub parameters: Vec<define_function2::Parameter>,
  #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
  pub body: Vec<u8>,
}

// Action code 0x83
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct GetUrl {
  pub url: String,
  pub target: String,
}

pub mod get_url2 {
  #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
  #[serde(rename_all = "kebab-case")]
  pub enum SendVarsMethod {
    None,
    Get,
    Post,
  }
}

// Action code 0x9a
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct GetUrl2 {
  pub send_vars_method: get_url2::SendVarsMethod,
  pub load_target: bool,
  pub load_variables: bool,
}

// Action code 0x81
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct GotoFrame {
  pub frame: usize,
}

// Action code 0x9f
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct GotoFrame2 {
  pub play: bool,
  pub scene_bias: usize,
}

// Action code 0x8c
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct GoToLabel {
  pub label: String,
}

// Action code 0x9d
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct If {
  pub offset: i16,
}

// Action code 0x99
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Jump {
  pub offset: i16,
}

// Action code 0x96
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Push {
  pub values: Vec<Value>,
}

// Action code 0x8b
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct SetTarget {
  pub target_name: String,
}

// Action code 0x87
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct StoreRegister {
  pub register_number: u8,
}

pub mod r#try {
  #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
  #[serde(tag = "type", content = "target", rename_all = "kebab-case")]
  pub enum CatchTarget {
    Register(u8),
    Variable(String),
  }
}

// Action code 0x8f
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Try {
  #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
  pub r#try: Vec<u8>,
  pub catch_target: r#try::CatchTarget,
  #[serde(skip_serializing_if = "Option::is_none", serialize_with = "option_buffer_to_hex", deserialize_with = "option_hex_to_buffer")]
  pub catch: Option<Vec<u8>>,
  #[serde(skip_serializing_if = "Option::is_none", serialize_with = "option_buffer_to_hex", deserialize_with = "option_hex_to_buffer")]
  pub finally: Option<Vec<u8>>,
}

// Action code 0x8a
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct WaitForFrame {
  pub frame: usize,
  pub skip_count: usize,
  // TODO: body: Vec<Action> ?
}

// Action code 0x8d
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct WaitForFrame2 {
  pub skip_count: usize,
  // TODO: body: Vec<Action> ?
}

// Action code 0x94
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct With {
  #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
  pub with: Vec<u8>,
}
