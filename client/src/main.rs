use noughts_and_crosses::Result;
use noughts_and_crosses::{client::Client, utils};
use program::game::Mark;
use solana_sdk::signature::Signer;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!(
            "usage: {} <path to solana noughts-and-crosses program keypair>",
            args[0]
        );
        std::process::exit(-1);
    }
    let program_keypair_path = &args[1];

    let rpc_url = utils::get_rpc_url()?;
    let organizer = utils::get_user_keypair("keypair_path")?;
    let player_1 = utils::get_user_keypair("keypair_1_path")?;
    let player_2 = utils::get_user_keypair("keypair_2_path")?;
    let program_keypair = utils::get_program_keypair(program_keypair_path)?;

    let client = Client::new(
        rpc_url.as_str(),
        player_1.pubkey(),
        player_2.pubkey(),
        organizer,
        program_keypair,
    );

    println!("{} plays X", player_1.pubkey());
    println!("{} plays O", player_2.pubkey());
    client.create_game()?;

    client.play(&player_1, 0, 0, Mark::X)?;
    print_grid(&client.get_grid()?);
    client.play(&player_1, 0, 1, Mark::O)?;
    print_grid(&client.get_grid()?);
    client.play(&player_1, 2, 0, Mark::X)?;
    print_grid(&client.get_grid()?);
    client.play(&player_1, 1, 1, Mark::O)?;
    print_grid(&client.get_grid()?);
    client.play(&player_1, 1, 0, Mark::X)?;
    print_grid(&client.get_grid()?);

    if client.is_over()? {
        println!("Game is over");
    }

    if let Some(winner) = client.get_winner()? {
        println!("Winner: {}", winner);
    } else {
        println!("No winner");
    }

    Ok(())
}

fn print_grid(grid: &Vec<Vec<Mark>>) {
    for row in grid {
        for mark in row {
            print!("{}", mark);
        }
        println!();
    }
    println!("**********");
}
