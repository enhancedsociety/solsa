#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate clap;

extern crate tera;

#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};

use tera::{Context, Tera};

use std::fs;

use std::thread;
use std::sync::Arc;

mod tools;
mod tool_output;

lazy_static! {
    pub static ref TERA: Tera = {
        // Get templates at compile time, remove a runtime dependency
        let mut tera = Tera::default();
        tera.add_raw_template("index.html", include_str!("../templates/index.html")).unwrap();
        tera.register_filter("float", |s, _| Ok(serde_json::value::to_value(s.as_str().unwrap().parse::<f32>().unwrap()).unwrap()));
        tera
    };
}


fn main() {
    let matches = App::new("solsa")
        .version("1.0")
        .about(
            "Aggregates static analysis tooling for ethereum smart contracts.",
        )
        .author("Enhanced Society")
        .arg(
            Arg::with_name("contract-file")
                .short("f")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .default_value("index.html"),
        )
        .get_matches();

    let contract_path: String = matches.value_of("contract-file").expect(
        "Contract file is necessary",
    ).to_owned();

    let output_path = matches.value_of("output").unwrap();

    // very fast to complete, the penalty to run in parallel is unnecesary
    let solc_out = tools::run_solc(&contract_path);
    let solium_out = tools::run_solium(&contract_path);

    // slower tools gain a bit by running in parallel

    let cp_arc = Arc::new(contract_path);
    let cp_arc_myth = cp_arc.clone();
    let cp_arc_oyente = cp_arc_myth.clone();

    let myth_handle = thread::spawn(move || {
        tools::run_mythril(cp_arc_myth.as_ref())
    });

    let oyente_handle = thread::spawn(move || {
        tools::run_oyente(cp_arc_oyente.as_ref())
    });

    let myth_out = myth_handle.join().expect("Failed to run mythril");
    let oyente_out = oyente_handle.join().expect("Failed to run oyente");

    let mut ctx = Context::new();
    ctx.add("contract_file", cp_arc.as_ref());
    match solc_out {
        Some(s) => {
            match s {
                tools::SolcResponse::Success(j) => ctx.add("solc_out", &j),
                tools::SolcResponse::Failure(s) => ctx.add("solc_err", &s),
            }
        }
        _ => (),
    }
    match solium_out {
        Some(s) => {
            match s {
                tools::SoliumResponse::Success(j) => ctx.add("solium_out", &j),
                tools::SoliumResponse::Failure(s) => ctx.add("solium_err", &s),
            }
        }
        _ => (),
    }
    match myth_out {
        Some(s) => {
            match s {
                tools::MythrilResponse::Success(j) => ctx.add("myth_out", &j),
                tools::MythrilResponse::Failure(s) => ctx.add("myth_err", &s),
            }
        }
        _ => (),
    }
    match oyente_out {
        Some(s) => {
            match s {
                tools::OyenteResponse::Success(j) => ctx.add("oyente_out", &j),
                tools::OyenteResponse::Failure(s) => ctx.add("oyente_err", &s),
            }
        }
        _ => (),
    }

    let idx = TERA.render("index.html", &ctx).expect(
        "Failed to render reports",
    );
    fs::write(&output_path, &idx).expect("Unable to write file");
}
