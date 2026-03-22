extern crate std;

use quasar_svm::{Account, Instruction, Pubkey, QuasarSvm};
use solana_address::Address;

use quasar_counter_client::{IncrementInstruction, InitializeInstruction};

use crate::Counter;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_counter.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

#[test]
fn test_flow() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();
    let (counter, counter_bump) = Pubkey::find_program_address(&[b"counter"], &crate::ID);

    // Initialize Instruction
    let instruction: Instruction = InitializeInstruction {
        payer: Address::from(payer.to_bytes()),
        counter: Address::from(counter.to_bytes()),
        system_program: Address::from(quasar_svm::system_program::ID.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &instruction,
        &[
            Account {
                address: payer,
                lamports: 10_000_000_000,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
            Account {
                address: counter,
                lamports: 0,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
        ],
    );

    let counter_account = svm.get_account(&counter).unwrap();

    let data = &counter_account.data;
    let count = u64::from_le_bytes(data[1..9].try_into().unwrap());
    let bump = data[9];
    assert_eq!(count, 0);
    assert_eq!(bump, counter_bump);

    result.assert_success();

    // Increament Instruction
    let incrment_instruction: Instruction = IncrementInstruction {
        payer: Address::from(payer.to_bytes()),
        counter: Address::from(counter.to_bytes()),
        system_program: Address::from(quasar_svm::system_program::ID.to_bytes()),
    }
    .into();

    let result = svm.process_instruction(
        &incrment_instruction,
        &[Account {
            address: payer,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    let counter_account = svm.get_account(&counter).unwrap();

    let data = &counter_account.data;
    let count = u64::from_le_bytes(data[1..9].try_into().unwrap());
    let bump = data[9];
    assert_eq!(count, 1);
    assert_eq!(bump, counter_bump);

    result.assert_success();
}
