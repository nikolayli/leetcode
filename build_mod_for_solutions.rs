use std::fs;
use std::path::Path;

fn main() {
    let dest_path = Path::new("src/solutions/mod.rs");
    let mut modules = Vec::new();

    if let Ok(entries) = fs::read_dir("src/solutions") {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.starts_with("s_") && file_name.ends_with(".rs") {
                    let mod_name = file_name.trim_end_matches(".rs");
                    modules.push(mod_name.to_string());
                }
            }
        }
    }

    modules.sort();
    let mods = modules
        .iter()
        .map(|mod_name| format!("pub mod {};\n", mod_name))
        .collect::<String>();

    fs::write(&dest_path, mods).expect("Failed to write mod.rs");
    println!("cargo:rerun-if-changed=src/solutions");
}
