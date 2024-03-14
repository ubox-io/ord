use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Claim {
    #[clap(long, help = "Etch with fee rate of <FEE_RATE> sats/vB.")]
    fee_rate: FeeRate,
    #[clap(long, help = "Etch rune <RUNE>. May contain `.` or `•`as spacers.")]
    rune: SpacedRune,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub rune: SpacedRune,
    pub transaction: Txid,
}

impl Claim {
    pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
        ensure!(
            wallet.has_rune_index(),
            "`ord wallet etch` requires index created with `--index-runes` flag",
        );

        let SpacedRune { rune, spacers: _ } = self.rune;

        let bitcoin_client = wallet.bitcoin_client();

        let (id, _, _) = wallet
            .get_rune(rune)?
            .with_context(|| format!("rune `{}` has not been etched", rune))?;

        let destination = wallet.get_change_address()?;

        let runestone = Runestone {
            etching: None,
            edicts: vec![],
            default_output: None,
            burn: false,
            claim: Some(id.into()),
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
            input: Vec::new(),
            output: vec![
                TxOut {
                    script_pubkey,
                    value: 0,
                },
                TxOut {
                    script_pubkey: destination.script_pubkey(),
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
