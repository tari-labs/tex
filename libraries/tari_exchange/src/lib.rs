use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tari_template_lib::prelude::ResourceAddress;

pub const ADMIN: &str = "GreatOotle";
pub const LIQUIDITY_PROVIDER: &str = "HumbleLiquidityProvider";
pub const TRADER: &str = "CuriousTrader";
pub const TEX_TEMPLATE_HEX: &str =
    "6fb668f01cd9c74afbbd23f4151b252ffc825085971b8d65cd4fb49598db2e28";
pub const COIN_TEMPLATE_HEX: &str =
    "9edd4c3b831885bb3c017bc0acd7f74fe33f0b4a33ea73a9ad8dae1add0480b5";
pub const TEX_COMPONENT_INSTANCE_ADDRESS_STR: &str =
    "component_39806234aba484d9806ab8f2372f44d84ad090c4431b196d4dc82db1cb213697";
pub const COIN_COMPONENT_INSTANCE_ADDRESS_STR: &str =
    "component_69e41614dcc9444854a9b541bc81094dee8bf42f383cbf58f5817725b40230d7";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TariCoin {
    pub id: ResourceAddress,
    pub name: String,
    pub balance: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TariTransaction {
    pub id: String,
    pub json: String,
    pub result: Option<String>,
    pub status: String,
    pub date_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub a: i64,
    pub b: i64,
    pub lp_resource: ResourceAddress,
    pub fees_collected: HashMap<String, f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiquidityPools {
    pub inner: HashMap<Pair, LiquidityPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exchange {
    pub liquidity_pools: LiquidityPools,
    pub fee: i64,
}

pub type Pair = String;

impl Exchange {
    pub fn pools(&self) -> Vec<LiquidityPool> {
        self.liquidity_pools.inner.values().cloned().collect()
    }
}

impl PartialEq for LiquidityPool {
    fn eq(&self, other: &Self) -> bool {
        self.lp_resource == other.lp_resource
    }
}

impl Eq for LiquidityPool {}
