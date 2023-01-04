use std::io::Read;
use ntfs::Ntfs;

#[macro_use]
extern crate afl;

#[allow(unused)]
fn test_dir(path: String) {
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        test_crash(path.unwrap().path().display().to_string());
    }
}

#[allow(unused)]
fn test_crash(path: String) {
    println!("Testing file: {}", path);
    let file = std::fs::File::open(path);
    if let Ok(mut f) = file {
        let mut buf: Vec<u8> = Vec::new();
        let _ = f.read_to_end(&mut buf);
        let mut data = std::io::Cursor::new(&buf[..]);
        if let Ok(mut fs) = ntfs::Ntfs::new(&mut data) {
            let _ = fs.read_upcase_table(&mut data);
            println!("All good !");
        }
    } else {
        println!("Error: can't open the test file");
    }
}

#[allow(unused)]
fn fuzz() {
    fuzz!(|data: &[u8]| {
        let mut data = std::io::Cursor::new(data);
        if let Ok(mut fs) = Ntfs::new(&mut data) {
            let _ = fs.read_upcase_table(&mut data);
        }
    })
}

fn main() {
    // fuzz();
    test_crash("/home/arthur/vm-fuzz-results/session1/hangs/test47".to_owned());
    // test_dir("/home/arthur/vm-fuzz-results/session1/hangs".to_owned());
}
