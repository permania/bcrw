use std::{collections::HashMap, env, fs, path::PathBuf};

fn main() {
    println!("cargo:warning=build script started");
    println!("cargo:rerun-if-changed=data.bun");
    let p = std::path::Path::new("data.bun");
    if !p.exists() {
        panic!("where are the bunnies??");
    }
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let input = manifest_dir.join("data.bun");
    let contents = fs::read_to_string(&input).unwrap();
    let parsed = parse_something(contents);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("buns.rs"), parsed).unwrap();
}

fn add_line(to: &mut String, line: &str) -> () {
    to.push_str(line)
}

fn parse_something(s: String) -> String {
    let mut lines = s.lines();
    let mut res = String::new();
    res.push_str("use std::sync::LazyLock;");
    res.push_str("use std::collections::HashMap;");

    res.push_str("pub static MAP: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| { let mut m = HashMap::new();");
    while let Some(name) = lines.next() {
        let second1 = lines.next().expect("expected 2 lines but file ended (1)");
        let second2 = lines.next().expect("expected 2 lines but file ended (2)");
        let bunny = format!("{}\n{}", second1, second2);

        println!("cargo:warning=adding {}: {:?}", name, bunny);

        add_line(
            &mut res,
            &format!(r##"m.insert("{}", r#"{}"#);"##, name, bunny),
        );
    }

    res.push_str("m });");

    res
}
