#[macro_use]
extern crate serde_derive;

extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate clap;

extern crate tera;

#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};

use tera::{Context, Tera};

use std::fs;

use std::sync::Arc;
use std::thread;

mod tool_output;
mod tools;

lazy_static! {
    pub static ref TERA: Tera = {
        // Get templates at compile time, remove a runtime dependency
        let mut tera = Tera::default();
        tera.add_raw_template("index.html", include_str!("../templates/index.html")).unwrap();
        tera.register_filter("float", |s, _|
        Ok(serde_json::value::to_value(
            s.as_str().unwrap().parse::<f32>().unwrap()
        ).unwrap()));
        tera
    };
}

fn main() {
    let matches = App::new("solsa")
        .version(crate_version!())
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
            Arg::with_name("output-format")
                .short("x")
                .takes_value(true)
                .possible_values(&["html", "json", "silent"])
                .default_value("html"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .default_value("index.html"),
        )
        .get_matches();

    let contract_path: String = matches
        .value_of("contract-file")
        .expect("Contract file is necessary")
        .to_owned();

    let output_path = matches.value_of("output").unwrap();
    let output_format = matches.value_of("output-format").unwrap();

    // very fast to complete, the penalty to run in parallel is unnecesary
    let solc_out = tools::run_solc(&contract_path);
    let solium_out = tools::run_solium(&contract_path);

    // slower tools gain a bit by running in parallel

    let cp_arc = Arc::new(contract_path);
    let cp_arc_myth = cp_arc.clone();
    let cp_arc_oyente = cp_arc_myth.clone();

    let myth_handle = thread::spawn(move || tools::run_mythril(cp_arc_myth.as_ref()));

    let oyente_handle = thread::spawn(move || tools::run_oyente(cp_arc_oyente.as_ref()));

    let myth_out = myth_handle.join().expect("Failed to run mythril");
    let oyente_out = oyente_handle.join().expect("Failed to run oyente");

    match output_format {
        "html" => {
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
                        tools::OyenteResponse::Success(j, b) => {
                            ctx.add("oyente_out", &j);
                            ctx.add("oyente_issues", &b)
                        }
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
        "json" => {
            let all_encompassing_json_monstruosity = json!({
                "solc" : match solc_out {
                    Some(tools::SolcResponse::Success(s)) =>
                    json!({"error": false, "result": s}),
                    Some(tools::SolcResponse::Failure(s)) =>
                     json!({"error": true, "result": s}),
                    None => json!({"error": false, "result": ""}),
                },
                "solium" : match solium_out {
                    Some(tools::SoliumResponse::Success(s)) =>
                    json!({"error": false, "result": s}),
                    Some(tools::SoliumResponse::Failure(s)) =>
                    json!({"error": true, "result": s}),
                    None => json!({"error": false, "result": ""}),
                },
                "mythril" : match myth_out {
                    Some(tools::MythrilResponse::Success(s)) =>
                    json!({"error": false, "result": s}),
                    Some(tools::MythrilResponse::Failure(s)) =>
                     json!({"error": true, "result": s}),
                    None => json!({"error": false, "result": ""}),
                },
                "oyente" : match oyente_out {
                    Some(tools::OyenteResponse::Success(s, _)) =>
                    json!({"error": false, "result": s}),
                    Some(tools::OyenteResponse::Failure(s)) =>
                    json!({"error": true, "result": s}),
                    None => json!({"error": false, "result": ""}),
                }
            });
            let s = serde_json::to_string_pretty(&all_encompassing_json_monstruosity);
            println!("{}", s.unwrap());
        }
        "silent" => {
            let mut tools_with_issues = Vec::new();
            match solc_out {
                Some(tools::SolcResponse::Failure(_)) => tools_with_issues.push("solc"),
                None => tools_with_issues.push("solc"),
                _ => {}
            };
            match solium_out {
                Some(tools::SoliumResponse::Success(ref l)) if l.len() > 0 => {
                    tools_with_issues.push("solium")
                }
                Some(tools::SoliumResponse::Failure(_)) => tools_with_issues.push("solium"),
                None => tools_with_issues.push("solium"),
                _ => {}
            };
            match myth_out {
                Some(tools::MythrilResponse::Success(ref o)) if !o.success => {
                    tools_with_issues.push("solium")
                }
                Some(tools::MythrilResponse::Failure(_)) => tools_with_issues.push("mythril"),
                None => tools_with_issues.push("mythril"),
                _ => {}
            };
            match oyente_out {
                Some(tools::OyenteResponse::Success(_, true)) => tools_with_issues.push("oyente"),
                Some(tools::OyenteResponse::Failure(_)) => tools_with_issues.push("oyente"),
                None => tools_with_issues.push("oyente"),
                _ => {}
            };
            if tools_with_issues.len() == 0 {
                println!("No issues found");
            } else {
                println!("Issues found in {}", tools_with_issues.join(", "));
            }
        }
        _ => {
            // should never happen
            panic!("output format outside of allowed values");
        }
    }

}
