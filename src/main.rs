use std::time::Instant;

use cli::{parse_cli_args, CliArgs};
use utils::display_output;
use walk::read_path;

mod cli;
mod utils;
mod walk;

fn main() {
    let CliArgs { path, buf_size } = dbg!(parse_cli_args());

    let mut buf = vec![0u8; buf_size];

    let start = Instant::now();
    let sizes = read_path(path, &mut buf);
    let time_took = start.elapsed();

    display_output(&sizes, &time_took);
}
