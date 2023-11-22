pub mod ast;
pub mod error;
pub mod parse;

pub use ast::Model;
pub use error::Error;
mod solvers;

use versions::Versioning;

pub fn conjure_executable() -> Result<String, String> {
    let path = std::env::var("PATH")
        .map_err(|_| "Could not read PATH environment variable".to_string())?;
    let mut paths = std::env::split_paths(&path);
    let conjure_dir = paths
        .find(|path| path.join("conjure").exists())
        .ok_or("Could not find conjure in PATH")?;
    let conjure_exec = conjure_dir.join("conjure")
        .to_str()
        .ok_or("Could not unwrap conjure executable path")?
        .to_string();
    
    let mut cmd = std::process::Command::new(&conjure_exec);
    let output = cmd
        .arg("--version")
        .output()
        .map_err(|_| "Could not execute conjure")?;
    let stdout = String::from_utf8(output.stderr)
        .map_err(|_| "Could not read conjure's stdout")?;

    let first = stdout
        .lines()
        .next()
        .ok_or("Could not read conjure's stdout")?;
    if first != "Conjure: The Automated Constraint Modelling Tool" {
        return Err("'conjure' points to an incorrect executable".to_string());
    }
    let second = stdout
        .lines()
        .nth(1)
        .ok_or("Could not read conjure's stdout")?;
    let version = second
        .strip_prefix("Release version ")
        .ok_or("Could not read conjure's stdout")?;
    if Versioning::new(version) < Versioning::new("2.5.0") {
        return Err("Conjure version is too old".to_string());
    }

    Ok(conjure_exec)
}
