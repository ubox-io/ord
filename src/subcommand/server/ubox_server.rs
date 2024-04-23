use std::str::FromStr;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use bitcoin::{BlockHash};
use tokio::task;
use crate::{Index};
use crate::templates::ServerConfig;
use crate::ubox::runes::rune_server::RuneBlockEvents;


// ubox event
pub(crate) struct UboxServer {}

impl UboxServer {
  pub(crate) async fn rune_block_events(
    Extension(server_config): Extension<Arc<ServerConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(block_hash): Path<String>,
  ) -> Result<Json<RuneBlockEvents>, &'static str> {
    task::block_in_place(|| {
      let blockhash = BlockHash::from_str(&block_hash).unwrap();

      let mut events = vec![];
      if let Ok(block_opt) = index.get_block_info_by_hash(blockhash) {
        if let Some(block) = block_opt {
          // get blockhash from redb.
          let blockhash = index
            .block_hash(Some(u32::try_from(block.height).unwrap()));

          if let Ok(blockhash) = blockhash {
            if let Some(blockhash) = blockhash {
              if block.hash != blockhash {
                return Err("Block not found");
              }
            } else {
              return Err("Not sync to this block");
            }
          } else {
            return Err("Not sync to this block");
          }

          for txid in block.tx.iter() {
            if let Ok(mut rune_event) = index.get_rune_event_by_txid(txid) {
              for x in &mut rune_event.outputs {
                let address = server_config.chain
                  .address_from_script(&x.script_pubkey)
                  .ok();
                if let Some(address) = address {
                  x.address = Some(address.to_string());
                } else {
                  x.address = Some("noStandard".to_string());
                }
              }
              for x in &mut rune_event.inputs {
                let address = server_config.chain
                  .address_from_script(&x.script_pubkey)
                  .ok();
                if let Some(address) = address {
                  x.address = Some(address.to_string());
                } else {
                  x.address = Some("noStandard".to_string());
                }
              }
              if let Some(mint) = &mut rune_event.mint {
                let address = server_config.chain
                  .address_from_script(&mint.script_pubkey)
                  .ok();
                if let Some(address) = address {
                  mint.address = Some(address.to_string());
                } else {
                  mint.address = Some("noStandard".to_string());
                }
              }
              events.push(rune_event);
            }
          }
        } else {
          return Err("Block not found");
        }
      } else {
        return Err("Block not found");
      }

      Ok(Json(RuneBlockEvents { events, blockhash: block_hash }))
    })
  }
}

