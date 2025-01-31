use std::{env, path::PathBuf, process::exit};

const BUFFER_SIZE: usize = 4096; // one block

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArgs {
    pub path: PathBuf,
    pub buf_size: usize,
}

pub fn parse_cli_args() -> CliArgs {
    let args = env::args().collect::<Vec<_>>();

    let exe_path = PathBuf::from(&args[0]);
    let app_name = exe_path.file_name().unwrap().to_str().unwrap();

    if args.len() < 2 {
        println!("Usage: {} <path> [buffer size (bytes)]", app_name);
        exit(1);
    }

    let buf_size = args
        .get(2)
        .map(|size| size.parse::<usize>().unwrap())
        .unwrap_or(BUFFER_SIZE);

    let path = PathBuf::from(&args[1]);

    CliArgs { path, buf_size }
}
