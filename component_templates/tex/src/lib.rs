//   Copyright 2025. The Tari Project
//
//   Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//   following conditions are met:
//
//   1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//   disclaimer.
//
//   2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//   following disclaimer in the documentation and/or other materials provided with the distribution.
//
//   3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//   products derived from this software without specific prior written permission.
//
//   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//   INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//   DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//   SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//   WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//   USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use bounded_integer::BoundedU8;
use tari_template_abi::rust::collections::HashMap;
use tari_template_lib::{
    prelude::*,
    template_dependencies::serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LiquidityPool {
    a: Vault,
    b: Vault,
    lp_resource: ResourceAddress,
    fees_collected: HashMap<String, f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiquidityPools {
    inner: HashMap<Pair, LiquidityPool>,
}

type Fee = BoundedU8<0, 100>;
//TODO:
// [] check a != b
// make it side independent (no matter if it's a<>b or b<>a)
type Pair = (ResourceAddress, ResourceAddress);

impl LiquidityPool {
    fn new((a_address, b_address): Pair) -> Self {
        //TODO: [STAGE2] customize LP to hold information about range
        //TODO: token should depend on Pool's identity
        let lp_resource = ResourceBuilder::fungible().with_token_symbol("LP").build();
        LiquidityPool {
            a: Vault::new_empty(a_address),
            b: Vault::new_empty(b_address),
            lp_resource,
            fees_collected: HashMap::new(),
        }
    }

    fn swap(&mut self, input: Bucket, output_address: ResourceAddress, fee: Fee) -> Bucket {
        let input_resource = input.resource_address();
        //TODO we are assuming that input a for now, make it both ways
        let input_pool_balance = self.a.balance();
        let output_pool_balance = self.b.balance();
        assert!(
            !input_pool_balance.is_zero(),
            "The pool for resource '{}' is empty",
            input_resource
        );
        assert!(
            !output_pool_balance.is_zero(),
            "The pool for resource '{}' is empty",
            output_address
        );
        let input_bucket_balance = input.amount().value();
        //TODO: move to some method of FEE
        use num_traits::ToPrimitive;
        let effective_input_balance = input_bucket_balance
            - (input_bucket_balance * fee.to_i64().expect("fee is not a valid i64")) / 100_i64;
        let effective_input_balance = Amount::new(effective_input_balance);
        //TODO: do not use fee automatically, put it into `fees_collected`
        let k = input_pool_balance * output_pool_balance;
        let new_input_pool_balance = input_pool_balance + effective_input_balance;
        let new_output_pool_balance = k / new_input_pool_balance;
        let output_bucket_amount = output_pool_balance - new_output_pool_balance;
        self.a.deposit(input);
        self.b.withdraw(output_bucket_amount)
    }

    //TODO: replace (Bucket, Bucket) with Pair analogy.
    // Pair should represent Pair of addresses while this type should represent values.
    fn add(&mut self, (a, b): (Bucket, Bucket)) -> Bucket {
        let a_amount = a.amount();
        let b_amount = b.amount();
        self.a.deposit(a);
        self.b.deposit(b);
        let a_ratio = if self.a.balance().is_zero() {
            Amount::new(1)
        } else {
            a_amount / self.a.balance()
        };
        let b_ratio = if self.b.balance().is_zero() {
            Amount::new(1)
        } else {
            b_amount / self.b.balance()
        };
        let new_lp_amount = a_ratio * a_amount + b_ratio * b_amount;
        ResourceManager::get(self.lp_resource).mint_fungible(new_lp_amount)
    }

    pub fn remove(&mut self, lp_bucket: Bucket) -> (Bucket, Bucket) {
        assert!(
            lp_bucket.resource_address() == self.lp_resource,
            "invalid lp resource {}, {} expected",
            lp_bucket.resource_address(),
            self.lp_resource
        );
        let a_balance = self.a.balance().value() as f64;
        let b_balance = self.b.balance().value() as f64;
        let lp_ratio = lp_bucket.amount().value() as f64
            / ResourceManager::get(self.lp_resource)
                .total_supply()
                .value() as f64;
        let a_amount = Amount::new((lp_ratio * a_balance).ceil() as i64);
        let b_amount = Amount::new((lp_ratio * b_balance).ceil() as i64);
        lp_bucket.burn();
        let a_bucket = self.a.withdraw(a_amount);
        let b_bucket = self.b.withdraw(b_amount);
        (a_bucket, b_bucket)
    }

    // pub fn collect_fees(&mut self, token: &Token, fee: f64) {
    //     let token_key = token.symbol.clone();
    //     *self.fees_collected.entry(token_key).or_insert(0.0) += fee;
    // }
}

impl LiquidityPools {
    fn liquidity_pool_mut(&mut self, pair: &Pair) -> Option<&mut LiquidityPool> {
        self.inner.get_mut(pair)
    }

    //TODO: merge with ^^^ and make it liquidity pool key - it should be a pair or liquidity provider token
    fn liquidity_pool_mut_by_(
        &mut self,
        liqudity_provider_token: &Bucket,
    ) -> Option<&mut LiquidityPool> {
        self.inner
            .values_mut()
            .find(|pool| pool.lp_resource == liqudity_provider_token.resource_address())
    }

    fn liquidity_pool_mut_or_insert(&mut self, pair: Pair) -> &mut LiquidityPool {
        self.inner
            .entry(pair)
            .or_insert_with(|| LiquidityPool::new(pair))
    }
}

#[template]
mod tex {
    use super::*;

    /// Tari Exchange. Decentralized exchange for Tari based network assets.
    /// Current version includes basic Automated Market Maker functionality with plain fees.
    pub struct Tex {
        liquidity_pools: LiquidityPools,
        fee: Fee,
    }

    impl Tex {
        /// Initialises a new exchange component.
        /// `fee` represents a percentage, so it must be between 0 and 100.
        pub fn new(fee: Fee) -> Component<Self> {
            Component::new(Self {
                fee,
                liquidity_pools: LiquidityPools::default(),
            })
            //TODO: [STAGE2] to protect liquidity (in a DEFI2.0 way), require to own an additional token (resource) to be able to interact with Tex
            // that way Tex will be an owner of own liquidity and prevent whales or other parties from manipulations on the markets
            .with_access_rules(AccessRules::allow_all())
            .create()
        }

        /// Trade provided asset to an asset of requested type.
        /// Execution may fail if we do not have enough liquidity of requested type on the market.
        pub fn swap(&mut self, input: Bucket, output_address: ResourceAddress) -> Bucket {
            let input_address = input.resource_address();
            self.liquidity_pools
                .liquidity_pool_mut(&(input_address, output_address))
                .unwrap_or_else(|| {
                    panic!(
                        "liquidity pool not available for ({input_address}{output_address}) pair"
                    )
                })
                .swap(input, output_address, self.fee)
        }

        pub fn add_liquidity(&mut self, a: Bucket, b: Bucket) -> Bucket {
            self.liquidity_pools
                .liquidity_pool_mut_or_insert((a.resource_address(), b.resource_address()))
                .add((a, b))
        }

        pub fn remove_liquidity(&mut self, lp_bucket: Bucket) -> (Bucket, Bucket) {
            self.liquidity_pools
                .liquidity_pool_mut_by_(&lp_bucket)
                .unwrap_or_else(|| {
                    panic!(
                        "liquidity pool not available for {}",
                        lp_bucket.resource_address()
                    )
                })
                .remove(lp_bucket)
        }

        pub fn pools(&self) -> LiquidityPools {
            self.liquidity_pools.clone()
        }
    }
}
