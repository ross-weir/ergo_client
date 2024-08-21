use super::{
    endpoints::{
        scan::{ScanBox, ScanQuery},
        NodeEndpoint,
    },
    NodeError,
};
use ergo_lib::{
    chain::transaction::{unsigned::UnsignedTransaction, Transaction},
    ergo_chain_types::EcPoint,
    ergotree_interpreter::sigma_protocol::private_input::DlogProverInput,
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

    pub async fn get_utxos(&self) -> Result<Vec<ErgoBox>, NodeError> {
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
    /// Get private key for EcPoint if it is in wallet database
    pub async fn get_private_key(&self, public_key: EcPoint) -> Result<DlogProverInput, NodeError> {
        let address = self.endpoints.utils()?.raw_to_address(public_key).await?;
        self.endpoints.wallet()?.get_private_key(&address).await
    }

    /// Get all unspent boxes. Maximum amount of boxes that can be retrieved in one API call to /scan/unspentBoxes is 2500, this will keep calling the endpoint until all boxes are retrieved
    pub async fn get_all_unspent_boxes(
        &self,
        scan_id: u32,
        include_unconfirmed: bool,
    ) -> Result<Vec<ScanBox>, NodeError> {
        let mut scan_query = ScanQuery {
            min_confirmations: if include_unconfirmed { -1 } else { 0 },
            max_confirmations: -1,
            min_inclusion_height: 0,
            max_inclusion_height: -1,
            limit: 2500,
            offset: 0,
        };
        let mut boxes = vec![];
        let scan_endpoint = self.endpoints.scan()?;
        loop {
            let new_boxes = scan_endpoint.unspent_boxes(scan_id, &scan_query).await?;
            boxes.extend_from_slice(&new_boxes);
            if new_boxes.is_empty() {
                break;
            }
            scan_query.offset += new_boxes.len() as u32;
        }
        Ok(boxes)
    }
}
