use std::{fs::{self, File}, io::Read, thread};

pub fn debug_hotreload() {
    let r = read_files();
    println!("{:?}", r);
}

fn read_files() -> Vec<String> {
    fs::read_dir("templates").unwrap().filter_map(
        |entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                let filename = format!("templates/{}", entry.file_name().to_string_lossy().into_owned());
                println!("{}", filename);
                let mut f = File::open(filename).unwrap();
                let mut contents = String::new();
                f.read_to_string(&mut contents).unwrap();
                Some(contents)
            } else {
                None
            }
        }
    ).collect::<Vec<String>>()
}
