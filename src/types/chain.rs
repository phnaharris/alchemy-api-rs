#![allow(missing_docs)]

use ethers_core::types::Chain;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlchemyChain {
    #[default]
    EthMainnet,
    EthGoerli,
    EthSepolia,
    OptMainnet,
    OptGoerli,
    ArbMainnet,
    ArbGoerli,
    MaticMainnet,
    MaticMumbai,
    AstarMainnet,
    PolygonzkevmMainnet,
    PolygonzkevmTestnet,
    BaseMainnet,
    BaseGoerli,
}

impl From<AlchemyChain> for Chain {
    fn from(alchemy_chain: AlchemyChain) -> Self {
        match alchemy_chain {
            AlchemyChain::EthMainnet => Chain::Mainnet,
            AlchemyChain::EthGoerli => Chain::Goerli,
            AlchemyChain::EthSepolia => Chain::Sepolia,
            AlchemyChain::OptMainnet => Chain::Optimism,
            AlchemyChain::OptGoerli => Chain::OptimismGoerli,
            AlchemyChain::ArbMainnet => Chain::Arbitrum,
            AlchemyChain::ArbGoerli => Chain::ArbitrumGoerli,
            AlchemyChain::MaticMainnet => Chain::Polygon,
            AlchemyChain::MaticMumbai => Chain::PolygonMumbai,
            AlchemyChain::AstarMainnet => unimplemented!("chain is not supported by ether-rs"),
            AlchemyChain::PolygonzkevmMainnet => Chain::PolygonZkEvm,
            AlchemyChain::PolygonzkevmTestnet => Chain::PolygonZkEvmTestnet,
            AlchemyChain::BaseMainnet => Chain::Base,
            AlchemyChain::BaseGoerli => Chain::BaseGoerli,
        }
    }
}
