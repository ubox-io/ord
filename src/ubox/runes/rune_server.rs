use bitcoin::Txid;
use serde::{Deserialize, Serialize};
use crate::ubox::runes::rune_event::Event;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RuneBlockEvents {
  pub events: Vec<TxidEvent>,
  pub blockhash: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub(crate) struct TxidEvent {
  pub txid: Txid,
  pub event: Event,
}

