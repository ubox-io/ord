use serde::{Deserialize, Serialize};
use crate::ubox::runes::rune_event::RuneEvent;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RuneBlockEvents {
  pub events: Vec<RuneEvent>,
  pub blockhash: String,
}


