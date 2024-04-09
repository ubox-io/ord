use std::str::FromStr;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use bitcoin::{BlockHash};
use tokio::task;
use crate::subcommand::server::error::{ServerError, ServerResult};
use crate::{Index, ubox};
use crate::templates::ServerConfig;


// ubox event
pub(crate) struct UboxServer {}

impl UboxServer {
  pub(crate) async fn rune_block_events(
    Extension(server_config): Extension<Arc<ServerConfig>>,
    Extension(index): Extension<Arc<Index>>,
    Path(block_hash): Path<String>,
  ) -> ServerResult<Json<ubox::runes::rune_server::RuneBlockEvents>> {
    task::block_in_place(|| {
      if !index.has_sat_index() {
        return Err(ServerError::NotFound(
          "this server has no sat index".to_string(),
        ));
      }
      let blockhash = BlockHash::from_str(&block_hash).unwrap();

      let mut events = vec![];
      if let Ok(block_opt) = index.get_block_info_by_hash(blockhash) {
        if let Some(block) = block_opt {
          for txid in block.tx.iter() {
            if let Ok(mut rune_event) = index.get_rune_event_by_txid(txid) {
              for x in &mut rune_event.outputs {
                let address = server_config.chain
                  .address_from_script(&x.script_pubkey)
                  .ok();
                if let Some(address) = address{
                  x.address = Some(address.unwrap().to_string());
                }else {
                  x.address = Some("noStandard".parse().unwrap());
                }

              }
              for x in &mut rune_event.inputs {
                let address = server_config.chain
                  .address_from_script(&x.script_pubkey)
                  .ok();
                x.address = Some(address.unwrap().to_string());
              }
              events.push(rune_event);
            }
          }
        }
      }

      Ok(Json(ubox::runes::rune_server::RuneBlockEvents { events, blockhash: block_hash }))
    })
  }
}

