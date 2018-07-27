macro_rules! docker_cmd {
    ($e:expr) => {{
        let mut dc = Command::new("docker");
        dc.arg("run")
            .arg("--rm")
            .arg("-m")
            .arg("1.5g")
            .arg("-v")
            .arg(format!("{}:/src:ro", env::current_dir().unwrap().display()))
            .arg(format!("enhancedsociety/{}", $e));
        dc
    }};
}
