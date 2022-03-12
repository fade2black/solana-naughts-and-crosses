//use crate::utils;
use crate::{Error, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use program::game::{Game, Mark, Move};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};

use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signer,
    signer::keypair::Keypair, transaction::Transaction,
};

pub struct Client {
    client: RpcClient,
    player_1: Pubkey,
    player_2: Pubkey,
    organizer: Keypair,
    program_keypair: Keypair,
}
impl Client {
    pub fn new(
        rpc_url: &str,
        player_1: Pubkey,
        player_2: Pubkey,
        organizer: Keypair,
        program_keypair: Keypair,
    ) -> Self {
        let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        Self {
            client,
            player_1,
            player_2,
            organizer,
            program_keypair,
        }
    }

    pub fn create_game(&self) -> Result<Pubkey> {
        let pubkey = self.derive_pubkey()?;

        if let Ok(acc) = self.client.get_account(&pubkey) {
            println!("Account already exists: {:?}", acc);
        } else {
            let size = self.get_schema_data_size()?;
            let program_id = self.program_keypair.pubkey();
            let organizer_id = self.organizer.pubkey();

            let minimum_balance = self.client.get_minimum_balance_for_rent_exemption(size)?;
            let blockhash = self.client.get_latest_blockhash()?;

            let instruction = solana_sdk::system_instruction::create_account_with_seed(
                &organizer_id, // payer
                &pubkey,       // derived key
                &organizer_id, // base key
                self.get_seed().as_str(),
                minimum_balance * 10,
                size as u64,
                &program_id, // owner
            );

            let tx = Transaction::new_signed_with_payer(
                &[instruction],
                Some(&organizer_id),
                &[&self.organizer],
                blockhash,
            );

            self.client.send_and_confirm_transaction(&tx)?;
        }

        Ok(pubkey)
    }

    pub fn play(&self, player: &Keypair, row: usize, col: usize, mark: Mark) -> Result<()> {
        let pubkey = self.derive_pubkey()?;
        let program_id = self.program_keypair.pubkey();
        let organizer_id = self.organizer.pubkey();
        let mv = Move { row, col, mark };

        let instruction = Instruction::new_with_borsh(
            program_id,
            &mv,
            vec![
                AccountMeta::new(pubkey, false),
                AccountMeta::new(player.pubkey(), false),
            ],
        );
        let blockhash = self.client.get_latest_blockhash()?;

        let trx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&organizer_id), // organizer (payer)
            &[&self.organizer],  // player
            blockhash,
        );
        self.client.send_and_confirm_transaction(&trx)?;

        Ok(())
    }

    pub fn is_over(&self) -> Result<bool> {
        let pubkey = self.derive_pubkey()?;

        let acc = self.client.get_account(&pubkey)?;
        let game = Game::try_from_slice(&acc.data)?;

        Ok(game.is_over())
    }

    pub fn get_winner(&self) -> Result<Option<Pubkey>> {
        let pubkey = self.derive_pubkey()?;

        let acc = self.client.get_account(&pubkey)?;
        let game = Game::try_from_slice(&acc.data)?;

        Ok(game.get_winner())
    }

    pub fn get_grid(&self) -> Result<Vec<Vec<Mark>>> {
        let pubkey = self.derive_pubkey()?;

        let acc = self.client.get_account(&pubkey)?;
        let game = Game::try_from_slice(&acc.data)?;

        Ok(game.get_grid())
    }

    fn derive_pubkey(&self) -> Result<Pubkey> {
        Ok(Pubkey::create_with_seed(
            &self.organizer.pubkey(),
            self.get_seed().as_str(),
            &self.program_keypair.pubkey(),
        )?)
    }

    fn get_seed(&self) -> String {
        let pk_1 = self.organizer.pubkey().to_string();
        let pk_2 = self.player_1.to_string();
        let pk_3 = self.player_1.to_string();
        format!(
            "{}-{}-{}",
            &pk_1.as_str()[0..10],
            &pk_2.as_str()[0..10],
            &pk_3.as_str()[0..10]
        )
    }

    pub fn get_schema_data_size(&self) -> Result<usize> {
        let encoded = Game::new(self.player_1, self.player_2)
            .try_to_vec()
            .map_err(|e| Error::SerializationError(e))?;
        Ok(encoded.len())
    }

    pub fn get_game_balance(&self) -> Result<u64> {
        Ok(self.client.get_balance(&self.derive_pubkey()?)?)
    }
}
