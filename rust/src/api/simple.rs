use std::str::FromStr;
use bip39::Mnemonic;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::AccountMeta, message::Message, pubkey::Pubkey, signer::Signer, system_program,
    transaction::Transaction,
};
use solana_sdk::signature::{Keypair, SeedDerivable, Signature};


const INITIALIZE_DISCRIMINANT: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserInfoParams {
    pub name: String,
    pub age: u8,
}

const PROGRAM_ID: &str = "79dcqDDUa1v3q4eR5xWpXqWVrtTCy53f8jaugNyjhbrC";

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}



#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn transact() -> String {
    let user = UserInfoParams {
        name: "Alice".to_string(),
        age: 30,
    };

    // The mnemonic phrase
    let mnemonic_phrase = "smooth salmon hundred sick ordinary dog hair cash transfer vibrant jungle badge";

    // Create a mnemonic object
    let mnemonic = Mnemonic::from_str(mnemonic_phrase).expect("Invalid mnemonic phrase");

    // Create a seed from the mnemonic
    let seed = mnemonic.to_seed("");

    // Create keypair from seed and derivation path
    let keypair = Keypair::from_seed_and_derivation_path(&seed, None)
        .expect("Failed to create keypair");

    let connection = RpcClient::new("https://api.devnet.solana.com");

    let signer_pubkey = keypair.pubkey();

    match Pubkey::from_str(PROGRAM_ID) {
        Ok(program_id) => {
            let seeds: &[&[u8]; 2] = &[b"hello", &signer_pubkey.to_bytes()];

            let (pda_account, _) = Pubkey::find_program_address(seeds, &program_id);

            let ix = solana_sdk::instruction::Instruction::new_with_borsh(
                program_id,
                &(INITIALIZE_DISCRIMINANT, user),
                vec![
                    AccountMeta::new(pda_account, false),
                    AccountMeta::new_readonly(signer_pubkey, true),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
            );
            let message = Message::new(&[ix], Some(&signer_pubkey));

            let mut tx = Transaction::new_unsigned(message);

            match connection.get_latest_blockhash() {
                Ok(latest_block_hash) => {
                    tx.sign(&[&keypair], latest_block_hash);

                    // let tx_id = connection
                    //     .send_and_confirm_transaction_with_spinner(&tx)
                    //     .map_err(|err| {
                    //         // println!("{:?}", err);
                    //     }).unwrap();
                    // format!("Program uploaded successfully. Transaction ID: {}", tx_id)

                    match connection
                        .send_and_confirm_transaction_with_spinner(&tx)
                        .map_err(|err| {
                            // println!("{:?}", err);
                        }) {
                        Ok(tx_id) => {
                            format!("Program uploaded successfully. Transaction ID: {}", tx_id)
                        }
                        Err(e) => {
                            format!("Failed to send and confirm transaction {:?}", e)
                        }
                    }

                },
                Err(_) => {
                    "Failed to get latest block hash".to_string()
                }
            }

        },
        Err(_) => {
            "Failed to create program Id!".to_string()
        }
    }


}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
