use std::{env, fs::File, io::Read, path::PathBuf, process::exit};

const BUFFER_SIZE: usize = 4096; // one block

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let path = PathBuf::from(&args[0]);
    let app_name = path.file_name().unwrap().to_str().unwrap();

    if args.len() != 2 {
        println!("Usage: {} file [buffer size]", app_name);
        exit(1);
    }

    let mut file = File::open(&args[1]).unwrap();
    let md = file.metadata().unwrap();

    let mut buf = Vec::with_capacity(
        args.get(2)
            .map(|size| size.parse::<usize>().unwrap())
            .unwrap_or(BUFFER_SIZE),
    );

    let mut _file_size = md.len();
    let mut bytes_read = 0;
    loop {
        let read = file.read(&mut buf).unwrap();
        if read == 0 {
            break;
        }

        bytes_read += read;
    }

    println!("Finished reading {bytes_read} bytes");
}
