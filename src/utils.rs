use std::time::Duration;

use crate::walk::ReadSizes;

pub fn duration_display(duration: &Duration) -> String {
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

pub fn calculate_throughput_str(bytes: i128, duration: &Duration) -> String {
    let tps = bytes / duration.as_micros() as i128 * 1_000_000;
    format!("{}/s", as_file_size_str(tps))
}

// TODO: Implement rounding properly
pub fn as_file_size_str(n: i128) -> String {
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

pub fn display_output(sizes: &ReadSizes, time_took: &Duration) {
    let ReadSizes {
        metadata_size,
        actual_size,
    } = sizes;

    println!(
        "Size (in metadata): {}. Actually read: {}. Took time: {}. Throughput: {}",
        as_file_size_str(*metadata_size as i128),
        as_file_size_str(*actual_size as i128),
        duration_display(time_took),
        calculate_throughput_str(*actual_size as i128, &time_took),
    );
}
