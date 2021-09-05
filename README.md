# Tatoonie_Discord_Script
This is a rust script to push Tatooine faucet servers status and balance updates to the discord channel. This script uses discords webhooks to send messages. Tatooine is faucet based on bitcoindevkit library.

## Building the Script
- cd `tatooine_script`
- execute `cargo build`
The will output the execuatble in /target/debug folder

## Using a custom settings file
By default this script will be using the configurations set in the settings.toml file. To update webhook and faucet url head to the file and update it according to your needs.

## Running the Script
On linux/mac
simply run:
- `cargo run server` for server status update
- `cargo run balance` for balance update

Using build execuatble:
- Put settings and tatooine_script in same folder and run
- `./tatooine_script server` for server status update
- `./tatooine_script balance` for balance update

All outputs/logs will be written to tatooine_discord.log file in the scripts working directory