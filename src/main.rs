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

mod tools;
mod tool_output;

lazy_static! {
    pub static ref TERA: Tera = {
        // Get templates at compile time, remove a runtime dependency
        let mut tera = Tera::default();
        tera.add_raw_template("index.html", include_str!("../templates/index.html")).unwrap();
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

    let contract_path = matches.value_of("contract-file").expect(
        "Contract file is necessary",
    );

    let output_path = matches.value_of("output").unwrap();

    let solc_out = tools::run_solc(&contract_path);
    let solium_out = tools::run_solium(&contract_path);
    let myth_out = tools::run_mythril(&contract_path);
    let oyente_out = tools::run_oyente(&contract_path);

    let mut ctx = Context::new();
    ctx.add("contract_file", &contract_path);
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
