use std::collections::HashMap;
use bitcoin::{OutPoint, ScriptBuf, Transaction, Txid};
use redb::{Table};
use crate::{RuneId};
use crate::index::entry::{Entry};
use crate::{Result};
use crate::index::{lot::Lot};
use crate::ubox::runes::rune_event::{Etch, RuneBalance, RuneEvent, RuneEventOutput};
use std::time::{Instant};

pub(crate) struct RuneEventCatcher<'a, 'tx> {
  pub(crate) transaction_id_to_rune_event: &'a mut Table<'tx, &'static crate::index::entry::TxidValue, &'static [u8]>,
}

impl RuneEventCatcher<'_, '_> {
  pub(crate) fn catch_event(&mut self, txid: Txid, tx: &Transaction, etch: Option<Etch>, burned: HashMap<RuneId, Lot>, allocated: Vec<HashMap<RuneId, Lot>>, inputs: Vec<RuneEventOutput>) -> Result<()> {
    let start_time = Instant::now();
    let mut outputs: Vec<RuneEventOutput> = vec![];
    let mut burns: Vec<RuneBalance> = vec![];
    for (id, amount) in burned {
      burns.push(RuneBalance {
        id,
        balance: amount.0,
      })
    }
    let burns_duration = start_time.elapsed();
    println!("rune_event_catcher.burns_duration use time ：{:?}", burns_duration);
    for (vout, balances) in allocated.into_iter().enumerate() {
      if balances.is_empty() {
        continue;
      }

      let tx_output = &tx.output[vout];
      if !&tx_output.script_pubkey.is_op_return() {
        let mut runes_balance: Vec<RuneBalance> = vec![];
        for (id, balance) in &balances {
          runes_balance.push(RuneBalance { id: *id, balance: balance.0 })
        }
        outputs.push(RuneEventOutput {
          output: OutPoint { txid, vout: vout as u32 },
          value: *&tx_output.value,
          script_pubkey: ScriptBuf::from_hex(&tx_output.script_pubkey.to_hex_string()).unwrap(),
          address: None,
          runes: runes_balance,
        })
      }
    }
    let allocated_duration = start_time.elapsed();
    println!("rune_event_catcher.allocated_duration use time ：{:?}", allocated_duration);
    if !inputs.is_empty() || !outputs.is_empty() || !burns.is_empty() || etch.is_some() {
      let event = RuneEvent { txid, inputs, outputs, etch, burns };
      self.transaction_id_to_rune_event.insert(&txid.store(), rmp_serde::to_vec(&event).unwrap().as_slice())?;
    }
    let save_duration = start_time.elapsed();
    println!("rune_event_catcher.save_duration use time ：{:?}", save_duration);
    return Ok(());
  }
}
