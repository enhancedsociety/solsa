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

use clap::{App, Arg, ArgGroup};

use tera::{Context, Tera};

use std::fs;

use std::sync::Arc;
use std::thread;

use std::process::Command;
use std::env;

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

enum OutputType {
    HTML,
    JSON,
    None,
}

fn docker_check(preload: bool) {
    let docker_executables = env::var("PATH")
        .unwrap_or(String::new())
        .split(":")
        .map(|p| format!("{}/docker", &p))
        .filter(|p_str| fs::metadata(p_str).is_ok())
        .collect::<Vec<String>>()
        .len();

    if docker_executables == 0 {
        panic!("Docker does not seem to be installed and is required.");
    }


    if preload {
        for tool in ["solc", "solium", "oyente", "mythril"].iter() {
            let mut dc = Command::new("docker");
            dc.arg("pull").arg(format!("enhancedsociety/{}", &tool));
            dc.status().expect(&format!(
                "Failed to get docker image for {}",
                &tool
            ));
        }
    }
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
                .long("contract-file")
                .takes_value(true)
                .help("Path to Solidity smart contract")
                .required(true),
        )
        .arg(
            Arg::with_name("html")
                .help("Output the report as an html file")
                .long("html"),
        )
        .arg(
            Arg::with_name("json")
                .help("Output the report as JSON")
                .long("json"),
        )
        .arg(
            Arg::with_name("silent")
                .help("Do not output the report, but only basic pass/fail info")
                .long("silent"),
        )
        .group(
            ArgGroup::with_name("output-format")
                .args(&["html", "json", "silent"])
                .multiple(false),
        )
        .arg(
            Arg::with_name("error-exit")
                .help("Exit with error code if issues are found")
                .long("error-exit")
                .requires("silent"),
        )
        .arg(
            Arg::with_name("preload")
                .help("Preload docker containers necessary for execution")
                .long("preload")
                .short("p"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("File to write report into")
                .conflicts_with("silent")
                .takes_value(true),
        )
        .get_matches();

    docker_check(matches.is_present("preload"));

    let contract_path: String = matches
        .value_of("contract-file")
        .expect("Contract file is required")
        .to_owned();


    let output_format = if matches.is_present("output-format") {
        match (
            matches.is_present("html"),
            matches.is_present("json"),
            matches.is_present("silent"),
        ) {
            (_, false, false) => OutputType::HTML,
            (false, true, false) => OutputType::JSON,
            (false, false, true) => OutputType::None,
            (_, _, _) => panic!("Only ONE output format can be chosen"),
        }
    } else {
        // default output_format
        OutputType::HTML
    };

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
        OutputType::HTML => {
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

            let output_path = matches.value_of("output").unwrap_or("index.html");
            fs::write(&output_path, &idx).expect("Unable to write file");
        }
        OutputType::JSON => {
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
            let s = serde_json::to_string_pretty(&all_encompassing_json_monstruosity)
                .expect("Failed to serialize report");

            let output_path = matches.value_of("output");
            match output_path {
                Some(p) => {

                    fs::write(&p, &s).expect("Unable to write file");
                }
                None => println!("{}", &s),
            };
        }
        OutputType::None => {
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
                if matches.is_present("error-exit") {
                    std::process::exit(1);
                }
            }
        }
    }

}
