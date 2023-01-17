use std::env;
use std::process::ExitCode;

use crate::errors::{CliError, CliResult};
use huak::env::venv::create_venv;
use huak::ops;
use huak::project::Project;

/// Run the `run` command.
pub fn run(command: Vec<String>) -> CliResult<()> {
    let cwd = env::current_dir()?;
    let project =
        Project::from(cwd).map_err(|e| CliError::new(e, ExitCode::FAILURE))?;
    let venv = create_venv(project.root())
        .map_err(|e| CliError::new(e, ExitCode::FAILURE))?;

    ops::run::run_command(&venv, &command)
        .map_err(|e| CliError::new(e, ExitCode::FAILURE))?;

    Ok(())
}
