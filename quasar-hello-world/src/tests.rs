extern crate std;

use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_hello_world_client::InitializeInstruction;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_hello_world.so");
    QuasarSvm::new()
        .with_program(&Pubkey::from(crate::ID), elf)
}

#[test]
fn test_initialize() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();

    let instruction: Instruction = InitializeInstruction {
        payer: Address::from(payer.to_bytes()),
        system_program: Address::from(quasar_svm::system_program::ID.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[Account {
            address: payer,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();
}
