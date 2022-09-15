use clap::Parser;
use libmoss::prelude::*;

mod args;
use args::MossCliArgs;

fn main() {
    let args = MossCliArgs::parse_from(wild::args());

    let moss_config: MossConfig<(&str, u16)> = match args.try_into() {
        Ok(config) => config,
        Err(err) => {
            println!("{err}");
            std::process::exit(1)
        }
    };

    let moss_client: MossClient<(&str, u16)> = match moss_config.try_into() {
        Ok(client) => client,
        Err(err) => {
            println!("{err}");
            std::process::exit(1)
        }
    };

    match moss_client.send() {
        Ok(response) => println!("{}", response),
        Err(err) => {
            println!("{err}");
            std::process::exit(1)
        }
    };
}
