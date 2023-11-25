use ergo_lib::{
    chain::transaction::unsigned::UnsignedTransaction, ergotree_ir::chain::ergo_box::ErgoBox,
};

use crate::Error;

use super::{endpoints::NodeEndpoint, NodeError};

#[derive(Debug)]
pub struct NodeExtension<'a> {
    endpoints: &'a NodeEndpoint,
}

impl<'a> NodeExtension<'a> {
    pub fn new(endpoints: &'a NodeEndpoint) -> Self {
        Self { endpoints }
    }

    async fn get_utxos(&self) -> Result<Vec<ErgoBox>, Error> {
        Ok(self
            .endpoints
            .wallet()?
            .boxes()?
            .unspent(None)
            .await?
            .into_iter()
            .map(|b| b.ergo_box)
            .collect::<Vec<_>>())
    }

    fn take_until_amount(
        &self,
        nano_erg_amount: u64,
        boxes: Vec<ErgoBox>,
    ) -> Result<Vec<ErgoBox>, Error> {
        let mut running_total = 0;
        let utxos = boxes
            .into_iter()
            .take_while(|b| {
                let keep_taking = running_total < nano_erg_amount;
                running_total += b.value.as_u64();
                keep_taking
            })
            .collect::<Vec<_>>();
        if running_total >= nano_erg_amount {
            Ok(utxos)
        } else {
            Err(NodeError::InsufficientFunds {
                requested: nano_erg_amount,
                found: running_total,
            })?
        }
    }

    pub async fn get_utxos_summing_amount(
        &self,
        nano_erg_amount: u64,
    ) -> Result<Vec<ErgoBox>, Error> {
        self.take_until_amount(nano_erg_amount, self.get_utxos().await?)
    }

    pub async fn sign_and_submit(&self, unsigned_tx: UnsignedTransaction) -> Result<String, Error> {
        let signed_tx = self
            .endpoints
            .wallet()?
            .transaction()?
            .sign(unsigned_tx, None, None)
            .await?;
        self.endpoints.transactions()?.submit(signed_tx).await
    }
}
