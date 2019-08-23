use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write as IoWrite;
use std::path::Path;

extern crate serde_json;

fn process_file(path: &Path) {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let map: HashMap<String, String> = serde_json::from_reader(reader).unwrap();

    let stem = path.file_stem().unwrap().to_str().unwrap();
    let output = Path::new(".")
        .join("src")
        .join("i18n")
        .join(format!("{}.rs", stem));
    let mut file = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(output)
        .unwrap();

    let _ = file.write(b"pub const OUTPUT: super::Language = super::Language {");
    for (key, value) in map.iter() {
        let _ =
            file.write(format!(r#"{}{}: "{}","#, "\n    ", key.to_lowercase(), value).as_bytes());
    }
    let _ = file.write(b"\n};");
}

fn process_json_files() -> std::io::Result<()> {
    let json_dir = Path::new(".").join("i18n");
    // let source_dir = Path::new(".").join("src").join("i18n");

    for entry in fs::read_dir(json_dir)? {
        let dir = entry?;
        process_file(&dir.path());
    }

    Ok(())
}

fn process_json_entry_file() -> std::io::Result<()> {
    let file = File::open("i18n.json").unwrap();
    let reader = BufReader::new(file);
    let map: HashMap<String, Vec<String>> = serde_json::from_reader(reader).unwrap();

    let mut fields = "pub struct Language {\n".to_owned();
    let mut impls = "impl Language {\n".to_owned();
    for (key, params) in map.iter() {
        let key = key.to_lowercase();
        let _ = writeln!(&mut fields, "    {}: &'static str,", key);
        if params.len() == 0 {
            let _ = writeln!(&mut impls, "    pub fn {}(&self) -> String {{", key);
            let _ = writeln!(&mut impls, "        self.{}.to_owned()", key);
            let _ = writeln!(&mut impls, "    }}");
        } else {
            let args = params
                .iter()
                .map(|k| format!("{}: &str", k))
                .collect::<Vec<String>>()
                .join(", ");
            let formats = params
                .iter()
                .map(|k| format!(".replace(\"{{{k}}}\", {k})", k = k))
                .collect::<Vec<String>>()
                .join("\n            ");
            let _ = writeln!(
                &mut impls,
                "    pub fn {}(&self, {}) -> String {{",
                key, args
            );
            let _ = writeln!(&mut impls, "        self.{}.to_owned()", key);
            let _ = writeln!(&mut impls, "            {}", formats);
            let _ = writeln!(&mut impls, "    }}");
        }
    }

    let _ = writeln!(&mut fields, "}}");
    let _ = writeln!(&mut impls, "}}");

    let path = Path::new(".").join("src").join("i18n").join("mod.rs");
    let mut file = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(path)
        .unwrap();

    let _ = file.write(fields.as_bytes());
    let _ = file.write(b"\n");
    let _ = file.write(impls.as_bytes());

    Ok(())
}

fn main() -> std::io::Result<()> {
    process_json_entry_file()?;
    process_json_files()?;
    Ok(())
}
