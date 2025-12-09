use rand::prelude::*;

use crate::cli::{
    args::{ListArgs, RandArgs},
    error::BunError::{self, NoBun},
};

pub fn print_bun(bun: &str) -> Result<(), BunError> {
    println!("{}", BUNS.get(bun).ok_or(NoBun)?);

    Ok(())
}

pub fn random_bun(args: RandArgs) {
    let mut rng = rand::rng();
    let (name, bun) = BUNS.iter().choose(&mut rng).unwrap();

    println!(
        "{}{}",
        (!args.hide_name)
            .then(|| format!("{}\n", name))
            .unwrap_or_default(),
        bun
    );
}

pub fn list_buns(args: ListArgs) {
    BUNS.iter().for_each(|(k, v)| {
        println!(
            "{k}{}",
            (args.show).then(|| format!("\n{}", v)).unwrap_or_default()
        )
    });
}

include!(concat!(env!("OUT_DIR"), "/buns.rs"));
