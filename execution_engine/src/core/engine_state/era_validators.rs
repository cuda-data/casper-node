use thiserror::Error;

use datasize::DataSize;

use casper_types::ProtocolVersion;

use crate::{core::engine_state::error::Error, shared::newtypes::Blake2bHash};

#[derive(Debug, Error, DataSize)]
pub enum GetEraValidatorsError {
    /// Invalid state hash was used to make this request
    #[error("Invalid state hash")]
    RootNotFound,
    /// Engine state error
    #[error(transparent)]
    Other(#[from] Error),
    /// EraValidators missing
    #[error("Era validators missing")]
    EraValidatorsMissing,
}

impl GetEraValidatorsError {
    pub fn is_era_validators_missing(&self) -> bool {
        matches!(self, GetEraValidatorsError::EraValidatorsMissing)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetEraValidatorsRequest {
    state_hash: Blake2bHash,
    protocol_version: ProtocolVersion,
}

impl GetEraValidatorsRequest {
    pub fn new(state_hash: Blake2bHash, protocol_version: ProtocolVersion) -> Self {
        GetEraValidatorsRequest {
            state_hash,
            protocol_version,
        }
    }

    pub fn state_hash(&self) -> Blake2bHash {
        self.state_hash
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }
}
