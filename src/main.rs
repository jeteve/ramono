use std::{time::Duration, thread};

fn main() {
    env_logger::Builder::new()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting Ramono");
    let one_second = Duration::from_secs(1);
    loop{
        thread::sleep(one_second);
    }
}
