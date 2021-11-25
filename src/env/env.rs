use nix::unistd::{execve, getcwd};
use std::ffi::CStr;

struct EnvCollection<'a> {
    pub envs: Vec<&'a CStr>,
}

impl<'a> EnvCollection<'a> {
    pub fn new() -> Self {
        EnvCollection { envs: Vec::new() }
    }

    pub fn insert(&mut self, s: &'a str) {
        self.envs
            .push(CStr::from_bytes_with_nul(s.as_bytes()).unwrap());
    }
}

pub fn envs() -> Result<(), std::io::Error> {
    let mut env: Vec<String> = Vec::new();
    std::env::vars_os().for_each(|(key, value)| {
        env.push(format!(
            "{}={}\0",
            key.to_str().unwrap(),
            value.to_str().unwrap()
        ));
    });

    let mut envs = EnvCollection::new();
    env.iter().for_each(|value| envs.insert(&value));
    let path = CStr::from_bytes_with_nul(b"/usr/bin/nsenter\0").unwrap();
    let workdir = format!("-w{}\0", getcwd().unwrap().to_str().unwrap());
    let args = &[
        CStr::from_bytes_with_nul(b"-r/\0").unwrap(),
        CStr::from_bytes_with_nul(workdir.as_bytes()).unwrap(),
        CStr::from_bytes_with_nul(b"--preserve-credentials\0").unwrap(),
        CStr::from_bytes_with_nul(b"-U\0").unwrap(),
        CStr::from_bytes_with_nul(b"-m\0").unwrap(),
        CStr::from_bytes_with_nul(b"-n\0").unwrap(),
        CStr::from_bytes_with_nul(b"-t\0").unwrap(),
        CStr::from_bytes_with_nul(b"63851\0").unwrap(),
        CStr::from_bytes_with_nul(b"-F\0").unwrap(),
        CStr::from_bytes_with_nul(b"./target/debug/examples/namespaces\0").unwrap(),
    ];
    execve(path, args, envs.envs.as_slice()).unwrap_err();
    Ok(())
}
