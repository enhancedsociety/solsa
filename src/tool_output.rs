use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SolcOutput {
    contracts: HashMap<String, SolcContract>,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SolcContract {
    abi: String,
    bin: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MythrilOutput {
    success: bool,
    error: Option<String>,
    issues: Vec<MythrilIssue>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MythrilIssue {
    title: String,
    description: String,
    #[serde(rename = "type")]
    type_: String,
    code: String,
    function: String,
    debug: String,
    filename: String,
    lineno: u32,
    address: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OyenteOutput {
    #[serde(flatten)]
    files: HashMap<String, OyenteSolidityFile>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OyenteSolidityFile {
    #[serde(flatten)]
    contracts: HashMap<String, OyenteContract>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OyenteContract {
    evm_code_coverage: String,
    vulnerabilities: OyenteVulnerabilities,
}

#[derive(Serialize, Deserialize, Debug)]
struct OyenteVulnerabilities {
    integer_overflow: Vec<String>,
    integer_underflow: Vec<String>,
    callstack: Vec<String>,
    money_concurrency: Vec<String>,
    time_dependency: Vec<String>,
    reentrancy: Vec<String>,
    assertion_failure: Vec<String>,
    parity_multisig_bug_2: Vec<String>,
}
