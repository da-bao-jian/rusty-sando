// // credit to 0xKitsune's cfmms-rs: https://github.com/0xKitsune/cfmms-rs/tree/main/src/pool

// use ethers::prelude::*;

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Pool {
//     pub address: Address,
//     pub token_0: Address,
//     pub token_1: Address,
//     pub swap_fee: U256,
//     pub pool_variant: PoolVariant,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum PoolVariant {
//     UniswapV2,
//     UniswapV3,
// }

// impl Pool {
//     // Creates a new pool instance
//     pub fn new(
//         address: Address,
//         token_a: Address,
//         token_b: Address,
//         swap_fee: U256,
//         pool_variant: PoolVariant,
//     ) -> Pool {
//         let (token_0, token_1) = if token_a < token_b {
//             (token_a, token_b)
//         } else {
//             (token_b, token_a)
//         };

//         Pool {
//             address,
//             token_0,
//             token_1,
//             swap_fee,
//             pool_variant,
//         }
//     }
// }

// impl PoolVariant {
//     pub fn pool_created_event_signature(&self) -> H256 {
//         match self {
//             PoolVariant::UniswapV2 => {
//                 H256::from_str("0x0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9")
//                     .unwrap()
//             }
//             PoolVariant::UniswapV3 => {
//                 H256::from_str("0x783cca1c0412dd0d695e784568c96da2e9c22ff989357a2e8b1d9b2b4e6b7118")
//                     .unwrap()
//             }
//         }
//     }
// }


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
}

impl Hash for Pool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}