use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write as IoWrite;
use std::path::Path;

extern crate serde_json;

fn process_file(path: &Path) -> String {
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

    let _ = file.write(b"use super::definition::Language;\n");
    let _ = file.write(b"pub const OUTPUT: Language = Language {");
    for (key, value) in map.iter() {
        let _ =
            file.write(format!(r#"{}{}: "{}","#, "\n    ", key.to_lowercase(), value).as_bytes());
    }
    let _ = file.write(b"\n};\n");

    stem.to_owned()
}

fn process_json_files() -> std::io::Result<Vec<String>> {
    let json_dir = Path::new(".").join("i18n");
    let mut languages = Vec::new();
    // let source_dir = Path::new(".").join("src").join("i18n");

    for entry in fs::read_dir(json_dir)? {
        let dir = entry?;
        languages.push(process_file(&dir.path()));
    }

    Ok(languages)
}

fn process_json_entry_file(languages: Vec<String>) -> std::io::Result<()> {
    let file = File::open("i18n.json").unwrap();
    let reader = BufReader::new(file);
    let map: HashMap<String, Vec<String>> = serde_json::from_reader(reader).unwrap();

    let mut imports = "pub mod definition;\n".to_owned();
    for language in languages.iter() {
        let _ = writeln!(&mut imports, "pub mod {};", language);
    }

    let mut fields = "pub struct Language {\n".to_owned();
    let mut impls = "impl Language {\n".to_owned();
    for (key, params) in map.iter() {
        let key = key.to_lowercase();
        let _ = writeln!(&mut fields, "    pub {}: &'static str,", key);
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

    let mut definition_file = {
        let path = Path::new(".")
            .join("src")
            .join("i18n")
            .join("definition.rs");
        fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .append(false)
            .open(path)
            .unwrap()
    };
    let _ = definition_file.write(b"#![rustfmt::skip]\n\n");
    let _ = definition_file.write(fields.as_bytes());
    let _ = definition_file.write(b"\n");
    let _ = definition_file.write(impls.as_bytes());

    let mut file = {
        let path = Path::new(".").join("src").join("i18n").join("mod.rs");
        fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .append(false)
            .open(path)
            .unwrap()
    };
    let _ = file.write(b"#![rustfmt::skip]\n\n");
    let _ = file.write(imports.as_bytes());
    let _ = file.write(b"\n");
    let _ = file.write(b"use std::collections::HashMap;\n");
    let _ = file.write(b"use std::iter::FromIterator;\n");

    let _ = file.write(b"lazy_static! {");
    let _ = file.write(
        b"\n    pub static ref OUTPUT: HashMap<String, definition::Language> = HashMap::from_iter(",
    );
    let _ = file.write(b"\n        vec![");
    for language in languages.iter() {
        let _ = file.write(
            format!(
                "\n            (\"{0}\".to_owned(), self::{0}::OUTPUT),",
                language
            )
            .as_bytes(),
        );
    }
    let _ = file.write(b"\n        ]");
    let _ = file.write(b"\n        .into_iter()");
    let _ = file.write(b"\n    );");
    let _ = file.write(b"\n}\n");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let languages = process_json_files()?;
    process_json_entry_file(languages)?;
    Ok(())
}
