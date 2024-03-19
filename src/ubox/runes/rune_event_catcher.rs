use bitcoin::{Txid};
use redb::{Table};
use crate::{MintEntry, Result, Rune};
use crate::index::entry::Entry;
use crate::runes::Mint;
use crate::ubox::runes::rune_event::{EdictEvent, EtchingEvent, Event};

pub(crate) struct RuneEventCatcher<'a, 'db, 'tx> {
  pub(crate) transaction_id_to_rune_event: &'a mut Table<'db, 'tx, &'static crate::index::entry::TxidValue, &'static [u8]>,
}

impl RuneEventCatcher<'_, '_, '_> {
  pub(crate) fn catch_etching_event(&self, balance: u128, divisibility: u8, id: u128, mint: Option<MintEntry>, rune: Rune, spacers: u32, symbol: Option<char>) -> Result<Event> {
    println!("catch_etching_event balance={}, divisibility={}, id={}, mint={:?}, rune={}, spacers={}, symbol={:?}", balance, divisibility, id, mint, rune, spacers, symbol);
    let mut _mint = None;
    if let Some(mint_entry) = mint {
      _mint = Some(Mint {
        deadline: mint_entry.deadline,
        limit: mint_entry.limit,
        term: mint_entry.end,
      });
    };
    let event = Event::Etching(EtchingEvent {
      id,
      divisibility,
      mint: _mint,
      rune,
      spacers,
      symbol,
    });
    Ok(event)
  }


  pub(crate) fn catch_edict_event(&self, output: usize, rune_id: u128, amount: u128, claim: bool, allocated: bool) -> Result<Event> {
    let event;
    if claim {
      event = Event::Claim(EdictEvent {
        id: rune_id,
        amount,
        output,
        allocated,
      });
    } else {
      event = Event::Edict(EdictEvent {
        id: rune_id,
        amount,
        output,
        allocated,
      });
    }

    Ok(event)
  }
  pub(crate) fn catch_burn_event(&self, rune_id: u128, amount: u128) -> Result<Event> {
    let event = Event::Burn(EdictEvent {
      id: rune_id,
      amount,
      output: 0,
      allocated: false,
    });
    Ok(event)
  }

  pub(crate) fn save_events(&mut self, txid: Txid, events: &[Event]) -> Result<()> {
    if !events.is_empty() {
      println!("save_events txid={}, events={:?}", txid, events);
      self.transaction_id_to_rune_event.insert(
        &txid.store(),
        rmp_serde::to_vec(events).unwrap().as_slice(),
      )?;
    }
    Ok(())
  }
}
