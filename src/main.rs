use std::{time::Duration, thread};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct CliArg{
    #[clap(short = 'm', long = "mem-increment", default_value_t = 1000)]
    memory_increment: usize,
    #[clap(short = 'l', long = "mem-limit", default_value_t = 1000 * 1000 * 1000 )]
    memory_limit: usize
}

fn main() {
    let args = CliArg::parse();

    env_logger::Builder::new()
    .target(env_logger::Target::Stdout)
    .filter_level(log::LevelFilter::Info)
    .init();

    log::info!("Starting Ramono");

    let mut memory_hog = ramono::MemoryHog::new(args.memory_increment, args.memory_limit);

    let one_second = Duration::from_secs(1);

    loop{
        thread::sleep(one_second);
        memory_hog.tick();
    }
}
