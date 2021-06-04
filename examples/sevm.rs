use evm::backend::{MemoryAccount, MemoryBackend, MemoryVicinity};
use evm::executor::{MemoryStackState, StackExecutor, StackSubstateMetadata};
use evm::Config;
use primitive_types::{H160, U256};
use std::{collections::BTreeMap, str::FromStr};
use vm_test::SolDef;

fn main() {
    let config = Config::istanbul();

    let vicinity = MemoryVicinity {
        gas_price: U256::zero(),
        origin: H160::default(),
        block_hashes: Vec::new(),
        block_number: Default::default(),
        block_coinbase: Default::default(),
        block_timestamp: Default::default(),
        block_difficulty: Default::default(),
        block_gas_limit: Default::default(),
        chain_id: U256::one(),
    };

    let sd = SolDef::from_path("contracts/Box.json");

    let code = &sd.deployed_bytecode[2..];

    let mut state = BTreeMap::new();
    state.insert(
        H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: hex::decode(code).unwrap(),
        },
    );
    state.insert(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: Vec::new(),
        },
    );

    let backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(u64::max_value(), &config);
    let state = MemoryStackState::new(metadata, &backend);
    let mut executor = StackExecutor::new(state, &config);

    let _reason = executor.transact_call(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        U256::zero(),
        hex::decode("6057361d0000000000000000000000000000000000000000000000000000000000000064")
            .unwrap(),
        u64::max_value(),
    );

    println!("{:?}", _reason);

    let _reason = executor.transact_call(
        H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        U256::zero(),
        hex::decode("2e64cec1").unwrap(),
        u64::max_value(),
    );

    println!("{:?}", _reason);
}
