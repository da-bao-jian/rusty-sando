use cfmms::{dex, pool};
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    hash::{Hash, Hasher},
};

pub type PoolVariant = dex::DexVariant;
pub type PoolType = pool::Pool;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pool {
    pub address: Address,
    pub token_0: Address,
    pub token_1: Address,
    pub swap_fee: U256,
    pub pool_variant: PoolVariant,
    pub pool_type: PoolType,
}

impl Pool {
    // Creates a new pool instance
    pub async fn new(
        provider: Arc<Provider<Ws>>,
        address: Address,
        token_a: Address,
        token_b: Address,
        swap_fee: U256,
        pool_variant: PoolVariant,
    ) -> Option<Pool> {
        let (token_0, token_1) = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };
        match pool_variant {
            PoolVariant::UniswapV2 => {
                // TODO: function to query pool info
                if let Ok(_pool_type) =
                    pool::UniswapV2Pool::new_from_address(address, provider).await
                {
                    println!("Getting Uni V2 Pool: {:?}", _pool_type);

                    Some(Pool {
                        address,
                        token_0,
                        token_1,
                        swap_fee,
                        pool_variant,
                        pool_type: PoolType::UniswapV2(_pool_type),
                    })
                } else {
                    None
                }
            }
            PoolVariant::UniswapV3 => {
                if let Ok(_pool_type) =
                    pool::UniswapV3Pool::new_from_address(address, provider).await
                {
                    println!("Getting Uni V3 Pool: {:?}", _pool_type);
                    Some(Pool {
                        address,
                        token_0,
                        token_1,
                        swap_fee,
                        pool_variant,
                        pool_type: PoolType::UniswapV3(_pool_type),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn default() -> Pool {
        Pool {
            address: Address::zero(),
            token_0: Address::zero(),
            token_1: Address::zero(),
            swap_fee: U256::zero(),
            pool_variant: PoolVariant::UniswapV2,
            pool_type: PoolType::UniswapV2(pool::UniswapV2Pool::default()),
        }
    }


    pub fn from(address: &Address, token_0: &Address, token_1: &Address, swap_fee: &U256, pool_variant: &PoolVariant, pool_type: &PoolType) -> Pool {
        Pool {
            address: *address,
            token_0: *token_0,
            token_1: *token_1,
            swap_fee: *swap_fee,
            pool_variant: *pool_variant,
            pool_type: *pool_type,
        }
    }
}

impl Hash for Pool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl Default for Pool {
    fn default() -> Self {
        Pool {
            address: Address::zero(),
            token_0: Address::zero(),
            token_1: Address::zero(),
            swap_fee: U256::zero(),
            pool_variant: PoolVariant::UniswapV2,
            pool_type: PoolType::UniswapV2(pool::UniswapV2Pool::default()),
        }
    }
}