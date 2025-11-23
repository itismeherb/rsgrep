use std::io::Read;

pub fn is_binary(path: &std::path::Path) -> bool {
    const SAMPLE: usize = 8192;

    let Ok(mut file) = std::fs::File::open(path) else { return false; };

    let mut buf = [0u8; SAMPLE];
    let Ok(n) = file.read(&mut buf) else { return false; };

    buf[..n].contains(&0)
}

