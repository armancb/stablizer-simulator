//! Command-line CHP simulator (Section IV).
//!
//! Usage: chp [-s SEED] [-t] [-q] FILE.chp
//!   -s SEED  seed the random number generator
//!   -t       print the final tableau
//!   -q       do not print measurement outcomes

use stabilizer::{chp, Rng};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut seed = None;
    let mut print_tableau = false;
    let mut quiet = false;
    let mut file = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-s" => match args.next().and_then(|s| s.parse().ok()) {
                Some(s) => seed = Some(s),
                None => return usage("-s needs an integer seed"),
            },
            "-t" => print_tableau = true,
            "-q" => quiet = true,
            "-h" | "--help" => return usage(""),
            _ if file.is_none() => file = Some(a),
            _ => return usage(&format!("unexpected argument {a:?}")),
        }
    }
    let Some(file) = file else {
        return usage("no input file");
    };

    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("chp: cannot read {file}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let result = chp::parse(&text).and_then(|c| chp::run(&c, &mut Rng::from_seed(seed)));
    let (outcomes, tableau) = match result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("chp: {e}");
            return ExitCode::FAILURE;
        }
    };
    if !quiet {
        for o in &outcomes {
            let kind = if o.deterministic {
                "determinate"
            } else {
                "random"
            };
            println!(
                "Outcome of measuring qubit {}: {} ({kind})",
                o.qubit, o.outcome as u8
            );
        }
    }
    if print_tableau {
        println!("{tableau}");
    }
    ExitCode::SUCCESS
}

fn usage(msg: &str) -> ExitCode {
    if !msg.is_empty() {
        eprintln!("chp: {msg}");
    }
    eprintln!("usage: chp [-s SEED] [-t] [-q] FILE.chp");
    if msg.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
