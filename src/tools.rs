use std::process::Command;
use std::env;

use serde_json;

use tool_output;

#[derive(Debug)]
pub enum SolcResponse {
    Success(tool_output::SolcOutput),
    Failure(String),
}

#[derive(Debug)]
pub enum MythrilResponse {
    Success(tool_output::MythrilOutput),
    Failure(String),
}

#[derive(Debug)]
pub enum OyenteResponse {
    Success(tool_output::OyenteOutput),
    Failure(String),
}

#[derive(Debug)]
pub enum SoliumResponse {
    Success(Vec<tool_output::SoliumIssue>),
    Failure(String),
}

macro_rules! docker_cmd {
    ($e:expr) => {{
        let mut dc = Command::new("docker");
        dc
		.arg("run")
		.arg("--rm")
		.arg("-v")
        .arg(format!("{}:/src:ro", env::current_dir().unwrap().display()))
        .arg(format!("enhancedsociety/{}", $e));
        dc
    }}
}

pub fn run_solc(solidity_contract_path: &str) -> Option<SolcResponse> {
    let mut cmd = docker_cmd!("solc");
    cmd.arg("--pretty-json")
        .arg("--combined-json")
        .arg("abi,bin")
        .arg("--allow-paths")
        .arg(".")
        .arg(solidity_contract_path);
    return cmd.output().ok().and_then(|output| {
        return if output.status.success() {
            String::from_utf8(output.stdout)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .and_then(|o| Some(SolcResponse::Success(o)))
        } else {
            String::from_utf8(output.stderr).ok().and_then(|s| {
                Some(SolcResponse::Failure(s))
            })
        };
    });
}


pub fn run_mythril(solidity_contract_path: &str) -> Option<MythrilResponse> {
    let mut cmd = docker_cmd!("mythril");
    cmd.arg("-xo").arg("json").arg(solidity_contract_path);
    return cmd.output().ok().and_then(|output| {
        return if output.status.success() {
            String::from_utf8(output.stdout)
                .ok()
                .and_then(|s| match serde_json::from_str(&s) {
                    Ok(o) => Some(MythrilResponse::Success(o)),
                    Err(s) => Some(MythrilResponse::Failure(
                        format!("Error deserializing: {:?}", &s),
                    )),
                })
                .or(Some(MythrilResponse::Failure(
                    "Unknown error deserializing".to_owned(),
                )))
        } else {
            String::from_utf8(output.stderr)
                .ok()
                .and_then(|s| Some(MythrilResponse::Failure(s)))
                .or(Some(MythrilResponse::Failure("Unknown error".to_owned())))
        };
    });
}

pub fn run_oyente(solidity_contract_path: &str) -> Option<OyenteResponse> {
    let mut cmd = docker_cmd!("oyente");
    cmd.arg("-w").arg("-ce").arg("-ap").arg(".").arg("-s").arg(
        solidity_contract_path,
    );
    return cmd.output().ok().and_then(|output| {
        return if output.status.success() {
            String::from_utf8(output.stdout)
                .ok()
                .and_then(|s| match serde_json::from_str(&s) {
                    Ok(o) => Some(OyenteResponse::Success(o)),
                    Err(s) => Some(OyenteResponse::Failure(
                        format!("Error deserializing: {:?}", &s),
                    )),
                })
                .or(Some(OyenteResponse::Failure(
                    "Unknown error deserializing".to_owned(),
                )))
        } else {
            String::from_utf8(output.stderr)
                .ok()
                .and_then(|s| Some(OyenteResponse::Failure(s)))
                .or(Some(OyenteResponse::Failure("Unknown error".to_owned())))
        };
    });
}


// from https://github.com/duaraghav8/Solium/blob/master/lib/reporters/gcc.js
// filename + ":" + error.line + ":" + error.column + ": " + error.type + ": " + error.message
pub fn run_solium(solidity_contract_path: &str) -> Option<SoliumResponse> {
    let mut cmd = docker_cmd!("solium");
    cmd.arg("-R").arg("gcc").arg("-f").arg(
        solidity_contract_path,
    );
    return cmd.output().ok().and_then(|output| {
        return if output.status.success() {
            String::from_utf8(output.stdout)
                .ok()
                .and_then(|o| {
                    let resp = o.lines()
                        .map(|s| {
                            s.splitn(5, ":")
                                .map(|s| s.to_owned())
                                .collect::<Vec<String>>()
                        })
                        .filter(|l| l.len() == 5)
                        .map(|components| {
                            tool_output::SoliumIssue {
                                filename: components[0].clone(),
                                line: components[1].parse::<u32>().unwrap_or(0),
                                column: components[2].parse::<u32>().unwrap_or(0),
                                type_: components[3].clone(),
                                message: components[4].clone(),
                            }
                        })
                        .collect();
                    Some(SoliumResponse::Success(resp))
                })
                .or(Some(SoliumResponse::Failure(
                    "Unknown error deserializing".to_owned(),
                )))
        } else {
            String::from_utf8(output.stderr)
                .ok()
                .and_then(|s| Some(SoliumResponse::Failure(s)))
                .or(Some(SoliumResponse::Failure("Unknown error".to_owned())))
        };
    });
}
