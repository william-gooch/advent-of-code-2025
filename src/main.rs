use std::{env::args, path::Path};

use anyhow::anyhow;

use crate::challenge::*;

mod challenge;
mod utils;

macro_rules! generate {
    ( $opt_var:expr, $($i:ident),+ ) => {
        match $opt_var {
            $(stringify!($i) => $i::default().run_file(Path::new(format!("input/{}.txt", stringify!($i)).as_str())),)+
            _ => Err(anyhow!("No such challenge exists."))
        }
    }
}

fn main() {
    println!("Hello, world!");

    let challenge_to_run = args()
        .skip(1)
        .next()
        .expect("Needs at least one argument for challenge to run.");

    println!("{:?}", challenge_to_run);

    let output = generate!(
        challenge_to_run.as_str(),
        Challenge1,
        Challenge2,
        Challenge3,
        Challenge4,
        Challenge5,
        Challenge6,
        Challenge7,
        Challenge8,
        Challenge9,
        Challenge10,
        Challenge11,
        Challenge12
    );

    match output {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{:#?}", error),
    }
}
