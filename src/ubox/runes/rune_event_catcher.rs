use std::collections::HashMap;
use bitcoin::{OutPoint, ScriptBuf, Transaction, Txid, TxOut};
use bitcoin::consensus::Decodable;
use redb::{ReadableTable, Table};
use crate::{Index, RuneId};
use crate::index::entry::Entry;
use crate::{Result};
use crate::ubox::runes::rune_event::{Etch, RuneBalance, RuneEvent, RuneEventOutput};

pub(crate) struct RuneEventCatcher<'a, 'tx> {
  pub(crate) transaction_id_to_rune_event: &'a mut Table<'tx, &'static crate::index::entry::TxidValue, &'static [u8]>,
  pub(crate) outpoint_to_balances: &'a mut Table<'tx, &'static crate::index::entry::OutPointValue, &'static [u8]>,
  pub(crate) transaction_id_to_transaction: &'a mut Table<'tx, &'static crate::index::entry::TxidValue, &'static [u8]>,

}

impl RuneEventCatcher<'_, '_> {
  pub(crate) fn get_input_runes(&self, tx: &Transaction) -> Vec<RuneEventOutput> {
    let mut inputs: Vec<RuneEventOutput> = vec![];
    let mut pre_txs: Vec<Txid> = vec![];
    for input in &tx.input {
      if !pre_txs.contains(&input.previous_output.txid) {
        pre_txs.push(input.previous_output.txid);
      }
    }
    let mut tx_map: HashMap<Txid, Transaction> = HashMap::new();
    for txid in pre_txs {
      let result = self.transaction_id_to_transaction.get(&txid.store());
      if result.is_ok() {
        let option = result.unwrap();
        if let Some(previous_tx) = option {
          if let Ok(tx) = Transaction::consensus_decode(&mut previous_tx.value()) {
            tx_map.insert(txid, tx);
          }
        }
      }
    }

    for input in &tx.input {
      let buffer_result = self.outpoint_to_balances.get(&input.previous_output.store());
      let mut i = 0;
      let mut runes_balance: Vec<RuneBalance> = vec![];
      if buffer_result.is_ok() {
        if let Some(buffer) = buffer_result.unwrap() {
          let buffer = buffer.value();
          while i < buffer.len() {
            let ((id, balance), len) = Index::decode_rune_balance(&buffer[i..]).unwrap();
            i += len;
            let balance = RuneBalance {
              id,
              balance,
            };
            runes_balance.push(balance);
          }
        }
      }

      if let Some(previous_tx) = tx_map.get(&input.previous_output.txid) {
        let tx_out: &TxOut = &previous_tx.output[input.previous_output.vout as usize];
        let rune_event_input = RuneEventOutput {
          output: input.previous_output,
          value: tx_out.value,
          script_pubkey: tx_out.script_pubkey.clone(),
          runes: runes_balance,
        };
        inputs.push(rune_event_input);
      }
    }
    inputs
  }

  pub(crate) fn catch_event(&mut self, txid: Txid, tx: &Transaction, etch: Option<Etch>, burned: HashMap<RuneId, u128>, allocated: Vec<HashMap<RuneId, u128>>, inputs: Vec<RuneEventOutput>) -> Result<()> {
    let mut outputs: Vec<RuneEventOutput> = vec![];
    let mut burns: Vec<RuneBalance> = vec![];
    for (id, amount) in burned {
      burns.push(RuneBalance {
        id,
        balance: amount,
      })
    }
    for (vout, balances) in allocated.into_iter().enumerate() {
      if balances.is_empty() {
        continue;
      }

      let tx_output = &tx.output[vout];
      if !&tx_output.script_pubkey.is_op_return() {
        let mut runes_balance: Vec<RuneBalance> = vec![];
        for (id, balance) in &balances {
          runes_balance.push(RuneBalance { id: *id, balance: *balance })
        }
        outputs.push(RuneEventOutput {
          output: OutPoint { txid, vout: vout as u32 },
          value: *&tx_output.value,
          script_pubkey: ScriptBuf::from_hex(&tx_output.script_pubkey.to_hex_string()).unwrap(),
          runes: runes_balance,
        })
      }
    }
    let event = RuneEvent { txid, inputs, outputs, etch, burns };
    self.transaction_id_to_rune_event.insert(&txid.store(), rmp_serde::to_vec(&event).unwrap().as_slice())?;
    return Ok(());
  }
}
