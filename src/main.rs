use cargo_nds::command::Cargo;
use cargo_nds::{build_nds, check_rust_version, get_metadata, run_cargo};

use clap::Parser;

use std::process;

fn main() {
    check_rust_version();

    let Cargo::Input(mut input) = Cargo::parse();

    let message_format = match input.cmd.extract_message_format() {
        Ok(fmt) => fmt,
        Err(msg) => {
            eprintln!("{msg}");
            process::exit(1)
        }
    };

    let (status, messages) = run_cargo(&input.cmd, message_format);

    if !status.success() {
        process::exit(status.code().unwrap_or(1));
    }

    if !input.cmd.should_build_nds() {
        return;
    }

    eprintln!("Getting metadata");
    let app_conf = get_metadata(&messages);

    eprintln!("Building nds: {}", app_conf.path_nds().display());
    build_nds(&app_conf);
}
