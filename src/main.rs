use std::fs;
use std::env;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let paths = fs::read_dir(args[1].as_str()).unwrap();

    for path in paths {
        if let Ok(entry) = path {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            let modified_file_name = file_name_str.replace(args[2].as_str(), "");
            let modified_path = entry.path().with_file_name(modified_file_name);
            fs::rename(entry.path(), modified_path)?;
        }
    }

    Ok(())
}
