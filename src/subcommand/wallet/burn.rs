use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Burn {
    #[clap(long, help = "Burn with fee rate of <FEE_RATE> sats/vB.")]
    fee_rate: FeeRate,
    #[clap(long, help = "Burn rune <RUNE>. May contain `.` or `•`as spacers.")]
    rune: SpacedRune,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub rune: SpacedRune,
    pub transaction: Txid,
}

impl Burn {
    pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
        ensure!(
          wallet.has_rune_index(),
          "burn runes requires index created with `--index-runes` flag",
        );

        let bitcoin_client = wallet.bitcoin_client();

        let SpacedRune { rune, spacers: _ } = self.rune;

        let (_id, entry, _parent) = wallet
            .get_rune(rune)?
            .with_context(|| format!("rune `{}` has not been etched", self.rune))?;

        let mut input_runes = 0;
        let mut input = Vec::new();

        let runic_outputs = wallet.get_runic_outputs()?;
        let inscriptions = wallet.inscriptions();
        let inscribed_outputs = inscriptions
            .keys()
            .map(|satpoint| satpoint.outpoint)
            .collect::<HashSet<OutPoint>>();

        for output in runic_outputs {
            if inscribed_outputs.contains(&output) {
                println!("{:?} also contains inscriptions, skip burn", output);
                continue;
            }

            let balance = wallet.get_rune_balance_in_output(&output, entry.rune)?;

            if balance > 0 {
                input_runes += balance;
                input.push(output);
            }
        }
        println!("find total rune {} of {}", input_runes, self.rune);

        ensure!(
              input_runes > 0,
              "not find any runes of `{}` in wallet",
              self.rune
            );

        let runestone = Runestone {
            etching: None,
            edicts: vec![],
            default_output: None,
            burn: true,
            claim: None,
        };

        let script_pubkey = runestone.encipher();

        ensure!(
          script_pubkey.len() <= 82,
          "runestone greater than maximum OP_RETURN size: {} > 82",
          script_pubkey.len()
        );

        let unfunded_transaction = Transaction {
            version: 2,
            lock_time: LockTime::ZERO,
            input: input
                .into_iter()
                .map(|previous_output| TxIn {
                    previous_output,
                    script_sig: ScriptBuf::new(),
                    sequence: Sequence::MAX,
                    witness: Witness::new(),
                })
                .collect(),
            output: vec![
                TxOut {
                    script_pubkey: runestone.encipher(),
                    value: 0,
                },
                TxOut {
                    script_pubkey: wallet.get_change_address()?.script_pubkey(),
                    value: TARGET_POSTAGE.to_sat(),
                },
            ],
        };

        let inscriptions = wallet
            .inscriptions()
            .keys()
            .map(|satpoint| satpoint.outpoint)
            .collect::<Vec<OutPoint>>();

        if !bitcoin_client.lock_unspent(&inscriptions)? {
            bail!("failed to lock UTXOs");
        }

        let unsigned_transaction =
            fund_raw_transaction(bitcoin_client, self.fee_rate, &unfunded_transaction)?;

        let signed_transaction = bitcoin_client
            .sign_raw_transaction_with_wallet(&unsigned_transaction, None, None)?
            .hex;

        let transaction = bitcoin_client.send_raw_transaction(&signed_transaction)?;

        Ok(Some(Box::new(Output {
            rune: self.rune,
            transaction,
        })))
    }
}
