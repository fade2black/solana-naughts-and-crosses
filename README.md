# Solana noughts-and-crosses
A Solana program (smart contract) that allows to play noughts-and-crosses.
Some parts of the project is adopted from this [repository](https://github.com/ezekiiel/simple-solana-program/tree/079054d86643cd56d9b62bfbda99d30bc5dcf68e), for example `run.sh`.

## Getting started
Assuming that Rust and Solana are installed, configure Solana to run locally.
```
solana config set --url localhost
solana-keygen new
```
These commands create Solana config files in `~/.config/solana/`
inluding a keypair in `json` format.

Run a local Solana validator
```
solana-test-validator
```

Generate three wallets (key pairs), one wallet corresponds to the game orginiser (intermediary)
and two wallets belong to competing players and put them into `~/.config/solana/cli/config.yml`.
```
...
json_rpc_url: "http://localhost:8899"
keypair_path: /Users/bayramkuliyev/.config/solana/id.json
keypair_1_path: /Users/mymac/.config/solana/player_1_id.json
keypair_2_path: /Users/mymac/.config/solana/player_2_id.json
...
```
## Building and deploying the Solana program
To build and deploy the Solana program in this repository to the Solana cluster

```
./run.sh deploy
```
If you get `permission denied: ./run.sh` error, fix it by `chmod +x ./run.sh`.

## Running the client program
`./run.sh client`

The client immitates playing

```rust
...
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
...
```
