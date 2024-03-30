use bitcoin::{OutPoint, ScriptBuf, Txid};
use crate::{RuneId};
use serde::{Deserialize, Serialize};
use ordinals::{Rune, Terms};



#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RuneEvent {
  pub txid: Txid,
  pub inputs: Vec<RuneEventOutput>,
  pub outputs: Vec<RuneEventOutput>,
  pub etch: Option<Etch>,
  pub burns: Vec<RuneBalance>,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
pub struct Etch {
  pub divisibility: Option<u8>,
  pub premine: Option<u128>,
  pub rune: Option<Rune>,
  pub spacers: Option<u32>,
  pub symbol: Option<char>,
  pub terms: Option<Terms>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RuneEventOutput {
  pub output: OutPoint,
  pub value: u64,
  pub script_pubkey: ScriptBuf,
  pub runes: Vec<RuneBalance>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RuneBalance {
  pub id: RuneId,
  pub balance: u128,
}


