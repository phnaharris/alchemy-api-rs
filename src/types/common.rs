use derive_builder::Builder;
use ethers_core::types::{Address, U256};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

/// Cursor object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    after: Option<String>,
}

/// Information for pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationObject {
    cursors: Cursor,
    total_count: u64,
}

/// NFT information for webhook filtering.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Builder, Debug, Clone, Serialize, Deserialize)]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
pub struct NftFilter {
    /// Contract address for an NFT. If this field and the `token_id` aren't set all metadata
    /// updates will be sent.
    contract_address: Address,
    /// Token ID for an NFT. Can be decimal or "0x" prefixed hex integer string. If this field and
    /// the `token_id` aren't set all metadata updates will be sent. This field can't be set if the
    /// `contract_address` field isn't set.
    #[builder(default)]
    token_id: Option<U256>,
}

impl NftFilter {
    /// Create a builder for the endpoint.
    pub fn builder() -> NftFilterBuilder {
        NftFilterBuilder::default()
    }
}

impl NftFilterBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.token_id.is_some() && self.contract_address.is_none() {
            return Err("cannot set token_id if the contract_address is not set".to_string());
        }
        Ok(())
    }
}
