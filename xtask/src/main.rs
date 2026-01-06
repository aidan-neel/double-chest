use anyhow::{bail, Result};
use std::process::{Command, Child};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str());

    match command {
        Some("dev") => run_dev(),
        _ => {
            println!("Usage: cargo xtask dev");
            Ok(())
        }
    }
}

fn run_dev() -> Result<()> {
    println!("🚀 Building all services...");

    let services = ["storage", "auth", "api"];
    for service in services {
        let status = Command::new("cargo")
            .args(["build", "-p", service])
            .status()?;
        
        if !status.success() {
            bail!("Failed to compile {}", service);
        }
    }

    println!("🏃 Starting Background Services...");

    let mut storage = Command::new("cargo")
        .args(["run", "-p", "storage", "--offline"])
        .spawn()?;

    let mut auth = Command::new("cargo")
        .args(["run", "-p", "auth", "--offline"])
        .spawn()?;

    println!("⏳ Waiting for gRPC nodes to stabilize...");
    thread::sleep(Duration::from_secs(3));

    println!("📡 Starting API gateway last...");
    let mut api = Command::new("cargo")
        .args(["run", "-p", "api", "--offline"])
        .spawn()?;

    let storage_id = storage.id();
    let auth_id = auth.id();
    let api_id = api.id();

    ctrlc::set_handler(move || {
        println!("\n🛑 Shutting down all services...");
        
        #[cfg(windows)]
        {
            let _ = Command::new("taskkill").args(["/F", "/T", "/PID", &storage_id.to_string()]).spawn();
            let _ = Command::new("taskkill").args(["/F", "/T", "/PID", &auth_id.to_string()]).spawn();
            let _ = Command::new("taskkill").args(["/F", "/T", "/PID", &api_id.to_string()]).spawn();
        }

        #[cfg(not(windows))]
        {
            let _ = Command::new("kill").arg(storage_id.to_string()).spawn();
            let _ = Command::new("kill").arg(auth_id.to_string()).spawn();
            let _ = Command::new("kill").arg(api_id.to_string()).spawn();
        }
        
        std::process::exit(0);
    })?;

    let status = api.wait()?;
    
    let _ = storage.kill();
    let _ = auth.kill();

    println!("API exited with: {}", status);
    Ok(())
}