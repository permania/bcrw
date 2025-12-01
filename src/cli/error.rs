use std::{io, process};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BunError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("no such bunny")]
    NoBun,
}

pub fn handle_error(r: Result<(), BunError>) -> ! {
    if let Err(e) = r {
        eprintln!("{e}");
        process::exit(1);
    } else {
        process::exit(0);
    }
}
