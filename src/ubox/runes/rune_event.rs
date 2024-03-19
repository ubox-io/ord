use bitcoin::Txid;
use crate::{Rune};
use serde::{Deserialize, Serialize};
use crate::runes::Mint;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum EventType {
  Etching,
  Claim,
  Edict,
  Burn,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Event {
  Etching(EtchingEvent),
  Edict(EdictEvent),
  Claim(EdictEvent),
  Burn(EdictEvent),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EtchingEvent {
  pub id: u128,
  pub divisibility: u8,
  pub mint: Option<Mint>,
  pub rune: Rune,
  pub spacers: u32,
  pub symbol: Option<char>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EdictEvent {
  pub id: u128,
  pub amount: u128,
  pub output: usize,
  pub allocated: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RuneEvent {
  pub event_type: EventType,
  pub txid: Txid,
  pub event: Event,
}

