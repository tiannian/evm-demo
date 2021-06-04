use ethereum_types::{Address, U256};
use std::cell::RefCell;
use std::sync::Arc;
use vm_test::SolDef;

fn main() {
    env_logger::init();
    let db = Arc::new(cita_vm::state::MemoryDB::new(false));
    let mut state = cita_vm::state::State::new(db).unwrap();

    let sd = SolDef::from_path("contracts/Box.json");

    let code = &sd.deployed_bytecode[2..];
    println!("{}", code);

    state.new_contract(
        &Address::from("0xBd770416a3345F91E4B34576cb804a576fa48EB1"),
        U256::from(10),
        U256::from(1),
        hex::decode(code).unwrap(),
    );

    state.new_contract(
        &Address::from("0x1000000000000000000000000000000000000000"),
        U256::from(1_000_000_000_000_000u64),
        U256::from(1),
        vec![],
    );

    let block_data_provider: Arc<dyn cita_vm::BlockDataProvider> =
        Arc::new(cita_vm::BlockDataProviderMock::default());
    let state_data_provider = Arc::new(RefCell::new(state));
    let context = cita_vm::evm::Context::default();
    let config = cita_vm::Config::default();

    let tx = cita_vm::Transaction {
        from: Address::from("0x1000000000000000000000000000000000000000"),
        to: Some(Address::from("0xBd770416a3345F91E4B34576cb804a576fa48EB1")),
        value: U256::from(0),
        nonce: U256::from(1),
        gas_limit: 80000,
        gas_price: U256::from(1),
        input: hex::decode(
            "60fe47b1000000000000000000000000000000000000000000000000000000000000002a",
        )
        .unwrap(),
    };
    let r = cita_vm::exec(
        block_data_provider.clone(),
        state_data_provider.clone(),
        context.clone(),
        config.clone(),
        tx,
    )
    .unwrap();
    println!("return={:?}", r);

    let tx = cita_vm::Transaction {
        from: Address::from("0x1000000000000000000000000000000000000000"),
        to: Some(Address::from("0xBd770416a3345F91E4B34576cb804a576fa48EB1")),
        value: U256::from(0),
        nonce: U256::from(2),
        gas_limit: 80000,
        gas_price: U256::from(1),
        input: hex::decode("6d4ce63c").unwrap(),
    };
    let r = cita_vm::exec(
        block_data_provider,
        state_data_provider,
        context,
        config,
        tx,
    )
    .unwrap();
    println!("return={:?}", r);
}
