// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use tw_coin_entry::derivation::Derivation;

/// Extend this enum.
/// TODO generate like `CoinType`.
#[derive(Default, strum_macros::FromRepr)]
#[repr(u32)]
pub enum TWDerivation {
    // Do not touch and use it!
    Custom = 1,
    BitcoinSegwit = 2,
    BitcoinLegacy = 3,
    BitcoinTestnet = 4,
    LitecoinLegacy = 5,
    SolanaSolana = 6,
    StratisSegwit = 7,
    BitcoinTaproot = 8,
    PactusMainnet = 9,
    PactusTestnet = 10,
    // end_of_derivation_enum - USED TO GENERATE CODE
    #[default]
    Default = 0,
}

impl From<TWDerivation> for Derivation {
    fn from(derivation: TWDerivation) -> Self {
        match derivation {
            TWDerivation::Default | TWDerivation::Custom => Derivation::Default,
            TWDerivation::BitcoinSegwit | TWDerivation::StratisSegwit => Derivation::Segwit,
            TWDerivation::BitcoinLegacy | TWDerivation::LitecoinLegacy => Derivation::Legacy,
            TWDerivation::BitcoinTestnet => Derivation::Testnet,
            TWDerivation::SolanaSolana => Derivation::Default,
            TWDerivation::BitcoinTaproot => Derivation::Taproot,
            TWDerivation::PactusMainnet => Derivation::Default,
            TWDerivation::PactusTestnet => Derivation::Testnet,
        }
    }
}
