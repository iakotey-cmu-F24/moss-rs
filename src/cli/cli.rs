
use clap::Parser;
use libmoss::prelude::*;

mod args;
use args::MossCliArgs;

fn main() {
    let args = MossCliArgs::parse_from(wild::args());

    let moss_config: MossConfig<(&str, u16)> = args.try_into().unwrap();

    let moss_client: MossClient<(&str, u16)> = moss_config.try_into().unwrap();

    let moss_response = moss_client.send().unwrap();

    println!("{}", moss_response);
}
