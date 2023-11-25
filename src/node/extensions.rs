use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;

use super::{endpoints::NodeEndpoint, NodeError};

#[derive(Debug)]
pub struct NodeExtension {
    endpoints: NodeEndpoint,
}

impl NodeExtension {
    pub fn new(endpoints: NodeEndpoint) -> Self {
        Self { endpoints }
    }

    async fn get_utxos(&self) -> Result<Vec<ErgoBox>, crate::Error> {
        Ok(self
            .endpoints
            .wallet()
            .boxes()
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
    ) -> Result<Vec<ErgoBox>, crate::Error> {
        let mut running_total = 0;
        let utxos = boxes
            .into_iter()
            .take_while(|b| {
                running_total += b.value.as_u64();
                running_total < nano_erg_amount
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

    pub async fn get_utxos_totaling_amount(
        &self,
        nano_erg_amount: u64,
    ) -> Result<Vec<ErgoBox>, crate::Error> {
        self.take_until_amount(nano_erg_amount, self.get_utxos().await?)
    }
}
