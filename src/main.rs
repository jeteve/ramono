use std::{time::Duration, thread, sync::{Arc, atomic::{AtomicBool, Ordering}}, env};


use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct CliArg{
    #[clap(short = 'm', long = "mem-increment", default_value_t = 1000)]
    memory_increment: usize,
    #[clap(short = 'l', long = "mem-limit", default_value_t = 1000 * 1000 * 1000 )]
    memory_limit: usize,

    #[clap(long = "file-increment", default_value_t = 0 )]
    file_increment: usize,
    #[clap(long = "file-limit", default_value_t = 1000 * 1000 * 1000 )]
    file_limit: usize,
}

fn main() {
    let stop = Arc::new(AtomicBool::new(false));

    let stop_me = stop.clone();
    ctrlc::set_handler(move || {
        log::debug!("received Ctrl-C");
        stop_me.store(true, Ordering::Relaxed );
    })
    .expect("Error setting Ctrl-C handler");

    env_logger::Builder::new().parse_filters(env::var("RUST_LOG").unwrap_or(String::from("info")).as_str()).init();

    let args = CliArg::parse();
    log::debug!("{:?}", args);

    /* env_logger::Builder::new()
    .target(env_logger::Target::Stdout)
    .filter_level(log::LevelFilter::Info)
    .init();
*/

    log::info!("Starting Ramono");

    let mut memory_hog = ramono::MemoryHog::new(args.memory_increment, args.memory_limit);
    let mut files_hog = ramono::FileHandlesHog::new( args.file_increment , args.file_limit);

    let one_second = Duration::from_secs(1);

    while ! stop.load(Ordering::Relaxed) {
        thread::sleep(one_second);
        log::trace!("Tick");
        memory_hog.tick();
        files_hog.tick();
    }
    log::info!("Bye");
}
