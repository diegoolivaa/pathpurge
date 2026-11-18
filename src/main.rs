use std::env;
use std::collections::HashSet;
use std::io::{self, Write};
use clap::{Parser, Subcommand, CommandFactory};

mod hosts;
use hosts::HostInfo;

const BANNER: &str = r#"
┏━┓┏━┓╺┳╸╻ ╻   ┏━┓╻ ╻┏━┓┏━╸┏━╸
┣━┛┣━┫ ┃ ┣━┫   ┣━┛┃ ┃┣┳┛┃╺┓┣╸ 
╹  ╹ ╹ ╹ ╹ ╹   ╹  ┗━┛╹┗╸┗━┛┗━╸
Made by @diegoolivaa on Github!
"#;

#[derive(Parser)]
#[command(name = "pathpurge")]
#[command(author = "@diegoolivaa on Github!")]
#[command(version = "0.1.0")]
#[command(about = "Audits, analyzes and cleans up your system PATH variable", long_about = None)]
#[command(before_help = BANNER)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check and audit the $PATH variable for issues
    Check,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check) => {
            println!("{}", BANNER);
            run_audit();
        }
        None => {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!("\n");
        }
    }
}

fn prompt_action(path_str: &str, issue_type: &str) -> String {
    print!("[{}] {} --> Action [k]eep / [d]elete? : ", issue_type, path_str);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_lowercase()
}

fn run_audit() {
    let host = HostInfo::gather();
    let path_string = env::var("PATH").unwrap_or_default();
    let path = env::split_paths(&path_string);
    let mut duplicates = 0;
    let mut not_existing = 0;
    let mut valid = 0;
    let mut files = 0;
    let mut cleaned_paths: Vec<std::path::PathBuf> = Vec::new();

    println!("Running on: {} ({})", host.os, host.arch);
    println!("Shell: {}", host.shell);
    println!("---------------------------------------------");

    let mut seen = HashSet::new();

    for path in path {
        if path.exists() {
            if path.is_dir() {
                if seen.insert(path.clone()) {
                    println!("[OK] --> {}", path.display());
                    valid += 1;
                    cleaned_paths.push(path);
                } else {
                    let choice = prompt_action(&path.display().to_string(), "WARN: DUPLICATE");
                    if choice == "d" {
                        println!("  -> Marked for deletion (skipped).");
                        duplicates += 1;
                    } else {
                        println!("  -> Kept.");
                        valid += 1;
                        cleaned_paths.push(path);
                    }
                }
            } else {
                let choice = prompt_action(&path.display().to_string(), "WARN: FILE");
                if choice == "d" {
                    println!("  -> Marked for deletion (skipped).");
                    files += 1;
                } else {
                    println!("  -> Kept.");
                    cleaned_paths.push(path);
                }
            }
        } else {
            let choice = prompt_action(&path.display().to_string(), "ERR: NOT FOUND");
            if choice == "d" {
                println!("  -> Marked for deletion (skipped).");
                not_existing += 1;
            } else {
                println!("  -> Kept.");
                cleaned_paths.push(path);
            }
        }
    }

    println!();
    println!("---------------------------------------------");
    println!("Valid Directories: {}", valid);
    println!("Nonexisting Directories: {}", not_existing);
    println!("Duplicate Directories: {}", duplicates);
    println!("Not Directories: {}", files);
    
    if let Ok(new_path_string) = env::join_paths(cleaned_paths) {
        println!();
        println!(">> Suggested clean PATH export:");
        println!("export PATH=\"{}\"", new_path_string.to_string_lossy());
    }
}