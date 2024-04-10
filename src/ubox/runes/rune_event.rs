use bitcoin::{OutPoint, ScriptBuf, Txid};
use crate::{RuneId};
use serde::{Deserialize, Serialize};
use ordinals::{Rune, SpacedRune};


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RuneEvent {
  pub txid: Txid,
  pub inputs: Vec<RuneEventOutput>,
  pub outputs: Vec<RuneEventOutput>,
  pub etch: Option<Etch>,
  pub mint: Option<RuneMint>,
  pub burns: Vec<RuneBalance>,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
pub struct Terms {
  pub amount: Option<u128>,
  pub cap: Option<u128>,
  pub start_height: Option<u64>,
  pub end_height: Option<u64>,
  pub start_offset: Option<u64>,
  pub end_offset: Option<u64>,
  pub abs_start_height: Option<u64>,
  pub abs_end_height: Option<u64>,
}


#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
pub struct Etch {
  pub rune_id: Option<RuneId>,
  pub rune: Option<Rune>,
  pub spacer_rune: Option<SpacedRune>,
  pub divisibility: Option<u8>,
  pub premine: Option<u128>,
  pub spacers: Option<u32>,
  pub symbol: Option<char>,
  pub terms: Option<Terms>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RuneEventOutput {
  pub output: OutPoint,
  pub value: u64,
  pub script_pubkey: ScriptBuf,
  pub address: Option<String>,
  pub runes: Vec<RuneBalance>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RuneBalance {
  pub id: RuneId,
  pub balance: u128,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RuneMint {
  pub id: RuneId,
  pub amount: u128,
  pub script_pubkey: ScriptBuf,
  pub address: Option<String>,
}



