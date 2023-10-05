use std::{time::Duration, thread};

use clap::Parser;

#[derive(Debug, Parser)]
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


    let memory_increment = args.memory_increment;
    let memory_limit = args.memory_limit;

    let one_second = Duration::from_secs(1);

    let mut mem_vector: Vec<u8> = vec![0; 0];
    loop{
        thread::sleep(one_second);
        if mem_vector.len() <= memory_limit {
            mem_vector.resize(mem_vector.len() + memory_increment , 0);
        }
        log::info!("Vector len={}B cap={}B", mem_vector.len(), mem_vector.capacity());
    }
}
