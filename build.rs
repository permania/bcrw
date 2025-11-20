use quote::quote;
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

    fs::write(out_dir.join("buns.rs"), parsed.to_string()).unwrap();
}

fn parse_something(s: String) -> proc_macro2::TokenStream {
    let mut map = HashMap::new();

    let mut lines = s.lines().peekable();
    while lines.peek().is_some() {
        let name = lines.next().unwrap().trim();
        let s1 = lines.next().expect("expected content line 1");
        let s2 = lines.next().expect("expected content line 2");
        let bunny = format!("{}\n{}", s1, s2);

        println!("cargo:warning=adding {}: {:?}", name, bunny);

        map.insert(name, bunny);
    }

    let buns = map.iter().map(|(k, v)| quote! { m.insert(#k, #v); });

    quote! {
    use std::{sync::LazyLock, collections::HashMap};

    pub static BUNS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
	let mut m = HashMap::<&'static str, &'static str>::new();
	#(#buns)*
	m
    });
    }
}
