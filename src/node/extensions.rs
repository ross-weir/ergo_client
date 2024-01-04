use super::{endpoints::NodeEndpoint, NodeError};
use ergo_lib::{
    chain::transaction::{unsigned::UnsignedTransaction, Transaction},
    ergotree_ir::{
        chain::{address::NetworkAddress, ergo_box::ErgoBox},
        ergo_tree::ErgoTree,
    },
};

#[derive(Debug)]
pub struct NodeExtension<'a> {
    endpoints: &'a NodeEndpoint,
}

impl<'a> NodeExtension<'a> {
    pub fn new(endpoints: &'a NodeEndpoint) -> Self {
        Self { endpoints }
    }

    async fn get_utxos(&self) -> Result<Vec<ErgoBox>, NodeError> {
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
    ) -> Result<Vec<ErgoBox>, NodeError> {
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
    ) -> Result<Vec<ErgoBox>, NodeError> {
        self.take_until_amount(nano_erg_amount, self.get_utxos().await?)
    }

    /// Signs and submits the supplied transaction.
    /// Returns the signed transaction that was submitted.
    pub async fn sign_and_submit(
        &self,
        unsigned_tx: UnsignedTransaction,
    ) -> Result<Transaction, NodeError> {
        let signed_tx = self
            .endpoints
            .wallet()?
            .transaction()?
            .sign(unsigned_tx, None, None)
            .await?;
        self.endpoints.transactions()?.submit(&signed_tx).await?;
        Ok(signed_tx)
    }

    /// Compiles the provided Ergo Script source code into a ErgoTree instance
    pub async fn compile_contract(&self, source: &str) -> Result<ErgoTree, NodeError> {
        let addr = self.endpoints.script()?.p2s_address(source).await?;
        Ok(NetworkAddress::try_from(addr)
            .unwrap()
            .address()
            .script()
            .unwrap())
    }
}
