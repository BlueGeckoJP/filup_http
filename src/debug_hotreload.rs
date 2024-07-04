use std::{fs::{self, File}, io::Read, thread, time::Duration};

use crate::TEMPLATES;

pub fn debug_hotreload() {
    info!("Hot reload thread is being started");
    thread::spawn(|| {
        let files_contents = read_files();
        let mut before_files = files_contents.clone();
        let mut after_files = files_contents.clone();

        loop {
            if before_files != after_files {
                TEMPLATES.update().unwrap();
                info!("TEMPLATES reloaded!!");
            }
            before_files = after_files.clone();
            after_files = read_files();
            thread::sleep(Duration::from_secs(3));
        }
    });
}

fn read_files() -> Vec<String> {
    fs::read_dir("templates").unwrap().filter_map(
        |entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                let filename = format!("templates/{}", entry.file_name().to_string_lossy().into_owned());
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
