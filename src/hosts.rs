use std::env;

pub struct HostInfo {
    pub os: String,
    pub arch: String,
    pub shell: String,
}

impl HostInfo {
    pub fn gather() -> Self {
        let os = env::consts::OS.to_string();
        let arch = env::consts::ARCH.to_string();
        let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());

        HostInfo { os, arch, shell }
    }
}