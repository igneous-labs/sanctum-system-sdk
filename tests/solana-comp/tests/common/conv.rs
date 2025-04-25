use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

pub fn to_sol_ix<const A: usize>(
    program: &Pubkey,
    accounts: &[Pubkey; A],
    is_signer: &[bool; A],
    is_writable: &[bool; A],
    data: &[u8],
) -> Instruction {
    Instruction {
        program_id: *program,
        accounts: accounts
            .iter()
            .copied()
            .zip(is_signer.iter().copied())
            .zip(is_writable.iter().copied())
            .map(|((pubkey, is_signer), is_writable)| AccountMeta {
                pubkey,
                is_signer,
                is_writable,
            })
            .collect(),
        data: Vec::from(data),
    }
}
