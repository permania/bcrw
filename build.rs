use indexmap::IndexMap;
use quote::quote;
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:warning=build script started");
    println!("cargo:rerun-if-changed=data.bun");

    let p = std::path::Path::new("data.bun");

    if !p.exists() {
        panic!("where are the bunnies??");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let input = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("data.bun");
    let parsed = parse_something(fs::read_to_string(&input).unwrap());

    fs::write(out_dir.join("buns.rs"), parsed.to_string()).unwrap();
}

fn parse_something(s: String) -> proc_macro2::TokenStream {
    let mut map = IndexMap::new();

    let mut lines = s.lines().peekable();
    while lines.peek().is_some() {
        let name = lines.next().unwrap().trim();
        let s1 = lines.next().expect("expected content line 1");
        let s2 = lines.next().expect("expected content line 2");
        let bunny = format!("{}\n{}", s1, s2);

        println!("cargo:warning=adding {}: {:?}", name, bunny);

        map.insert(name.to_string(), bunny);
    }

    let buns = map
        .iter()
        .map(|(k, v)| quote! { m.insert(String::from(#k), #v); });

    quote! {
    use std::{sync::LazyLock};
    use indexmap::IndexMap;

    pub static BUNS: LazyLock<IndexMap<String, &'static str>> = LazyLock::new(|| {
        let mut m = IndexMap::<String, &'static str>::new();
        #(#buns)*
        m
    });
    }
}
