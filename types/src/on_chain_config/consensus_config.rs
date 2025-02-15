// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::on_chain_config::OnChainConfig;
use anyhow::{format_err, Result};
use serde::{Deserialize, Serialize};

/// The on-chain consensus config, in order to be able to add fields, we use enum to wrap the actual struct.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum OnChainConsensusConfig {
    V1(ConsensusConfigV1),
}

/// The public interface that exposes all values with safe fallback.
impl OnChainConsensusConfig {
    /// The number of recent rounds that don't count into reputations.
    pub fn leader_reputation_exclude_round(&self) -> u64 {
        match &self {
            OnChainConsensusConfig::V1(config) => config.exclude_round,
        }
    }

    /// Decouple execution from consensus or not.
    pub fn decoupled_execution(&self) -> bool {
        match &self {
            OnChainConsensusConfig::V1(config) => config.decoupled_execution,
        }
    }

    /// Backpressure controls
    /// 1. how much gaps can be between ordered and committed blocks in decoupled execution setup.
    /// 2. how much gaps can be between the root and the remote sync info ledger.
    pub fn back_pressure_limit(&self) -> u64 {
        if !self.decoupled_execution() {
            return 10;
        }
        match &self {
            OnChainConsensusConfig::V1(config) => config.back_pressure_limit,
        }
    }
}

/// This is used when on-chain config is not initialized.
impl Default for OnChainConsensusConfig {
    fn default() -> Self {
        OnChainConsensusConfig::V1(ConsensusConfigV1::default())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConsensusConfigV1 {
    pub decoupled_execution: bool,
    pub back_pressure_limit: u64,
    pub exclude_round: u64,
}

impl Default for ConsensusConfigV1 {
    fn default() -> Self {
        Self {
            decoupled_execution: true,
            back_pressure_limit: 10,
            exclude_round: 20,
        }
    }
}

impl OnChainConfig for OnChainConsensusConfig {
    const IDENTIFIER: &'static str = "ConsensusConfig";

    /// The Move resource is
    /// ```ignore
    /// struct AptosConsensusConfig has copy, drop, store {
    ///    config: vector<u8>,
    /// }
    /// ```
    /// so we need two rounds of bcs deserilization to turn it back to OnChainConsensusConfig
    fn deserialize_into_config(bytes: &[u8]) -> Result<Self> {
        let raw_bytes: Vec<u8> = bcs::from_bytes(bytes)?;
        bcs::from_bytes(&raw_bytes)
            .map_err(|e| format_err!("[on-chain config] Failed to deserialize into config: {}", e))
    }
}
