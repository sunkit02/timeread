use std::{
    env,
    fs::File,
    io::Read,
    path::PathBuf,
    process::exit,
    time::{Duration, Instant},
};

const BUFFER_SIZE: usize = 4096; // one block

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let path = PathBuf::from(&args[0]);
    let app_name = path.file_name().unwrap().to_str().unwrap();

    if args.len() < 2 {
        println!("Usage: {} <file> [buffer size (bytes)]", app_name);
        exit(1);
    }

    let mut file = File::open(&args[1]).unwrap();
    let md = file.metadata().unwrap();

    let buf_size = args
        .get(2)
        .map(|size| size.parse::<usize>().unwrap())
        .unwrap_or(BUFFER_SIZE);
    let mut buf = vec![0u8; buf_size];

    let start = Instant::now();

    let file_size = md.len();
    let mut bytes_read = 0;
    loop {
        let read = file.read(&mut buf).unwrap();
        if read == 0 {
            break;
        }

        bytes_read += read;
    }

    let took = start.elapsed();
    println!(
        "Size (in metadata): {}. Actually read: {}. Took time: {}. Throughput: {}",
        as_file_size_str(file_size as i128),
        as_file_size_str(bytes_read as i128),
        duration_display(&took),
        calculate_throughput_str(bytes_read as i128, &took)
    );
}

fn duration_display(duration: &Duration) -> String {
    if duration < &Duration::from_secs(60) {
        return format!("{duration:?}");
    }

    let mut duration = duration.as_secs();
    let mut res = [0u64; 3];

    let mut i = 2;
    let mut unit = 60;
    while i >= 1 && duration > 0 {
        res[i] = duration % unit;
        duration /= unit;
        unit *= 60;
        i -= 1;
    }

    res[i] = duration;

    let mut s = format!("{:02}:", res[0]);
    s.push_str(&format!("{:02}:", res[1]));
    s.push_str(&format!("{:02}", res[2]));
    s
}

fn calculate_throughput_str(bytes: i128, duration: &Duration) -> String {
    let tps = bytes / duration.as_micros() as i128 * 1_000_000;
    format!("{}/s", as_file_size_str(tps))
}

fn as_file_size_str(n: i128) -> String {
    let sign = if n < 0 { -1 } else { 1 };
    let n = n.abs();
    if n < 1024 {
        format!("{}B", n * sign)
    } else if n < 1024i128.pow(2) {
        format!("{}KB", n / 1024 * sign)
    } else {
        format!("{}MB", n / 1024i128.pow(2) * sign)
    }
}
