use std::collections::LinkedList;
use std::fs;
use std::path::{Path, PathBuf};

fn process_file(root_dir: &Path, file_path: &Path) {
    // println!("Processing file: {:?}", file_path);
}

fn visit_dirs(root_dir: &Path) {
    let mut organized_titles: LinkedList<String> = LinkedList::new();
    let mut current_title: String;
    if let Ok(entries) = fs::read_dir(root_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path);
            } else {
                if let Some(file_name) = path.file_stem() {
                    let mut file_name_str: String = file_name.to_string_lossy().to_string();
                    file_name_str = file_name_str
                        .chars()
                        .rev()
                        .skip_while(|&c| c.is_numeric())
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect();
                    if organized_titles.contains(&file_name_str) {
                        println!("Already organized title {:?}", path);
                    } else {
                        current_title = file_name_str;
                        println!("{:?}", current_title);
                        process_file(&root_dir, &path);
                        organized_titles.push_back(current_title);
                    }
                }
            }
        }
    } else {
        eprintln!("{:?} is not a directory!", root_dir);
    }
}

fn main() {
    let path: PathBuf = PathBuf::from(std::env::args().nth(1).expect("no path given"));
    visit_dirs(&path);
}
