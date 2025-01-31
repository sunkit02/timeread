use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadSizes {
    /// Size indicated by metadata
    pub metadata_size: u64,
    /// Actual number of bytes read
    pub actual_size: u64,
}

pub fn read_path<P: AsRef<Path>>(path: P, buf: &mut Vec<u8>) -> ReadSizes {
    let metadata = fs::metadata(&path).unwrap();
    if metadata.is_dir() {
        read_directory(&path, buf)
    } else if metadata.is_file() {
        read_file(&path, buf)
    } else {
        eprintln!(
            "Error: {:?} has unsupported file type. Only directories and plain files.",
            path.as_ref()
        );
        ReadSizes {
            metadata_size: 0,
            actual_size: 0,
        }
    }
}

pub fn read_directory<P: AsRef<Path>>(path: P, buf: &mut Vec<u8>) -> ReadSizes {
    let entries = fs::read_dir(path).unwrap();

    let mut dir_sizes = ReadSizes {
        metadata_size: 0,
        actual_size: 0,
    };

    for entry in entries {
        let entry = entry.unwrap();
        let entry_sizes = read_path(entry.path(), buf);
        dir_sizes.metadata_size += entry_sizes.metadata_size;
        dir_sizes.actual_size += entry_sizes.actual_size;
    }

    dir_sizes
}

pub fn read_file<P: AsRef<Path>>(path: P, buf: &mut Vec<u8>) -> ReadSizes {
    let mut file = File::open(path).unwrap();
    let md = file.metadata().unwrap();

    let file_size = md.len();
    let mut bytes_read = 0;
    loop {
        let read = file.read(buf).unwrap();
        if read == 0 {
            break;
        }

        bytes_read += read;
    }

    ReadSizes {
        metadata_size: file_size,
        actual_size: bytes_read as u64,
    }
}
