use std::fs::{File, read_to_string, write};
use std::path::Path;

pub fn update_index(path: &Path, hash: &str) {
    let snap_dir = Path::new(".snappy");
    let index_file = snap_dir.join("index");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }
    if !index_file.exists() {
        File::create(&index_file).unwrap();
    }

    let contents = read_to_string(&index_file).unwrap();
    let lines = contents.lines();
    let mut lines = lines.collect::<Vec<&str>>();
    let updated_line = format!("{}\t{}", path.display(), hash);
    let mut found_line = false;

    for i in 0..lines.len() {
        if lines[i].starts_with(&path.display().to_string()) {
            lines[i] = &updated_line;
            found_line = true;
        }
    }

    if !found_line {
        lines.push(&updated_line);
    }

    write(index_file, lines.join("\n").as_bytes()).unwrap();
}
