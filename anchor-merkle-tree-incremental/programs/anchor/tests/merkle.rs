use anchor::hash::{hash_pair, ZERO_HASHES};
use anchor::state::MerkleTree;
use mollusk_svm::{program, result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

const IX_INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
const IX_INSERT: [u8; 8] = [205, 174, 245, 70, 145, 250, 39, 168];
const IX_VERIFY: [u8; 8] = [133, 161, 141, 48, 120, 198, 88, 150];

const ERR_ZERO_LEAF: ProgramError = ProgramError::Custom(6000);
const ERR_INVALID_PROOF: ProgramError = ProgramError::Custom(6002);
const ERR_CONSTRAINT_SEEDS: ProgramError = ProgramError::Custom(2006);

fn setup() -> (Mollusk, Pubkey) {
    let program_id = Pubkey::new_from_array(anchor::ID.to_bytes());
    let mollusk = Mollusk::new(&program_id, "../../target/deploy/anchor");
    (mollusk, program_id)
}

fn merkle_pda(authority: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"merkle", authority.as_ref()], program_id)
}

fn ix_initialize(program_id: &Pubkey, authority: &Pubkey, pda: &Pubkey) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*pda, false),
            AccountMeta::new_readonly(Pubkey::default(), false),
        ],
        data: IX_INITIALIZE.to_vec(),
    }
}

fn ix_insert(program_id: &Pubkey, authority: &Pubkey, pda: &Pubkey, leaf: [u8; 32]) -> Instruction {
    let mut data = IX_INSERT.to_vec();
    data.extend_from_slice(&leaf);
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new(*pda, false),
        ],
        data,
    }
}

fn ix_verify(
    program_id: &Pubkey,
    pda: &Pubkey,
    leaf: [u8; 32],
    index: u32,
    proof: &[[u8; 32]; 20],
) -> Instruction {
    let mut data = IX_VERIFY.to_vec();
    data.extend_from_slice(&leaf);
    data.extend_from_slice(&index.to_le_bytes());
    for p in proof {
        data.extend_from_slice(p);
    }
    Instruction {
        program_id: *program_id,
        accounts: vec![AccountMeta::new_readonly(*pda, false)],
        data,
    }
}

fn expected_root_after_insert(leaf: [u8; 32]) -> [u8; 32] {
    let mut h = leaf;
    for i in 0..20usize {
        h = hash_pair(&h, &ZERO_HASHES[i]);
    }
    h
}

fn parse_tree(data: &[u8]) -> (Pubkey, u8, u32, [u8; 32]) {
    let authority = Pubkey::new_from_array(data[8..40].try_into().unwrap());
    let depth = data[40];
    let next_index = u32::from_le_bytes(data[41..45].try_into().unwrap());
    let root: [u8; 32] = data[45..77].try_into().unwrap();
    (authority, depth, next_index, root)
}

fn do_initialize(
    mollusk: &Mollusk,
    program_id: &Pubkey,
    authority: &Pubkey,
    pda: &Pubkey,
) -> Vec<(Pubkey, Account)> {
    let (sp, sa) = program::keyed_account_for_system_program();
    let result = mollusk.process_and_validate_instruction(
        &ix_initialize(program_id, authority, pda),
        &vec![
            (*authority, Account::new(10_000_000_000, 0, &sp)),
            (*pda, Account::new(0, 0, &sp)),
            (sp, sa),
        ],
        &[Check::success()],
    );
    result.resulting_accounts
}

#[test]
fn test_initialize() {
    let (mollusk, program_id) = setup();
    let (sp, sa) = program::keyed_account_for_system_program();
    let authority = Pubkey::new_unique();
    let (pda, _) = merkle_pda(&authority, &program_id);

    let result = mollusk.process_and_validate_instruction(
        &ix_initialize(&program_id, &authority, &pda),
        &vec![
            (authority, Account::new(10_000_000_000, 0, &sp)),
            (pda, Account::new(0, 0, &sp)),
            (sp, sa),
        ],
        &[Check::success()],
    );
    println!("initialize CU: {}", result.compute_units_consumed);
    let accounts = result.resulting_accounts;
    let data = &accounts[1].1.data;

    assert_eq!(data.len(), MerkleTree::SPACE, "wrong account size");
    assert_eq!(accounts[1].1.owner, program_id, "wrong owner");

    let (stored_auth, depth, next_index, root) = parse_tree(data);
    assert_eq!(stored_auth, authority, "authority mismatch");
    assert_eq!(depth, 20, "depth must be 20");
    assert_eq!(next_index, 0, "next_index must start at 0");
    assert_eq!(root, ZERO_HASHES[20], "empty tree root mismatch");
}

#[test]
fn test_insert_updates_state() {
    let (mollusk, program_id) = setup();
    let (sp, _) = program::keyed_account_for_system_program();

    let authority = Pubkey::new_unique();
    let (pda, _) = merkle_pda(&authority, &program_id);

    let after_init = do_initialize(&mollusk, &program_id, &authority, &pda);
    let authority_acc = after_init[0].1.clone();
    let pda_acc = after_init[1].1.clone();

    let leaf = [1u8; 32];
    let result = mollusk.process_and_validate_instruction(
        &ix_insert(&program_id, &authority, &pda, leaf),
        &vec![(authority, authority_acc), (pda, pda_acc)],
        &[Check::success()],
    );
    println!("insert CU: {}", result.compute_units_consumed);

    let data = &result.resulting_accounts[1].1.data;
    let (_, _, next_index, root) = parse_tree(data);

    assert_eq!(next_index, 1, "next_index must be 1 after insert");
    assert_ne!(root, ZERO_HASHES[20], "root must change after insert");
    assert_eq!(
        root,
        expected_root_after_insert(leaf),
        "root doesn't match reference"
    );
    let _ = sp;
}

#[test]
fn test_insert_and_verify() {
    let (mollusk, program_id) = setup();
    let (sp, _) = program::keyed_account_for_system_program();

    let authority = Pubkey::new_unique();
    let (pda, _) = merkle_pda(&authority, &program_id);

    let after_init = do_initialize(&mollusk, &program_id, &authority, &pda);
    let authority_acc = after_init[0].1.clone();
    let pda_acc = after_init[1].1.clone();

    let leaf = [0xABu8; 32];
    let after_insert = mollusk.process_and_validate_instruction(
        &ix_insert(&program_id, &authority, &pda, leaf),
        &vec![(authority, authority_acc), (pda, pda_acc)],
        &[Check::success()],
    );
    let pda_acc = after_insert.resulting_accounts[1].1.clone();

    let proof: [[u8; 32]; 20] = std::array::from_fn(|i| ZERO_HASHES[i]);
    let result = mollusk.process_and_validate_instruction(
        &ix_verify(&program_id, &pda, leaf, 0, &proof),
        &vec![(pda, pda_acc.clone())],
        &[Check::success()],
    );
    println!("verify CU: {}", result.compute_units_consumed);

    let bad_proof: [[u8; 32]; 20] = std::array::from_fn(|_| [0xFFu8; 32]);
    mollusk.process_and_validate_instruction(
        &ix_verify(&program_id, &pda, leaf, 0, &bad_proof),
        &vec![(pda, pda_acc.clone())],
        &[Check::err(ERR_INVALID_PROOF)],
    );

    mollusk.process_and_validate_instruction(
        &ix_verify(&program_id, &pda, [0x01u8; 32], 0, &proof),
        &vec![(pda, pda_acc.clone())],
        &[Check::err(ERR_INVALID_PROOF)],
    );

    mollusk.process_and_validate_instruction(
        &ix_verify(&program_id, &pda, leaf, 1, &proof),
        &vec![(pda, pda_acc)],
        &[Check::err(ERR_INVALID_PROOF)],
    );
    let _ = sp;
}

#[test]
fn test_zero_leaf_rejected() {
    let (mollusk, program_id) = setup();
    let authority = Pubkey::new_unique();
    let (pda, _) = merkle_pda(&authority, &program_id);

    let after_init = do_initialize(&mollusk, &program_id, &authority, &pda);
    let authority_acc = after_init[0].1.clone();
    let pda_acc = after_init[1].1.clone();

    mollusk.process_and_validate_instruction(
        &ix_insert(&program_id, &authority, &pda, [0u8; 32]),
        &vec![(authority, authority_acc), (pda, pda_acc)],
        &[Check::err(ERR_ZERO_LEAF)],
    );
}

#[test]
fn test_wrong_authority_rejected() {
    let (mollusk, program_id) = setup();
    let authority = Pubkey::new_unique();
    let attacker = Pubkey::new_unique();
    let (pda, _) = merkle_pda(&authority, &program_id);
    let (sp, sa) = program::keyed_account_for_system_program();

    let after_init = do_initialize(&mollusk, &program_id, &authority, &pda);
    let pda_acc = after_init[1].1.clone();

    mollusk.process_and_validate_instruction(
        &ix_insert(&program_id, &attacker, &pda, [1u8; 32]),
        &vec![
            (attacker, Account::new(10_000_000_000, 0, &sp)),
            (pda, pda_acc),
        ],
        &[Check::err(ERR_CONSTRAINT_SEEDS)],
    );
    let _ = sa;
}
