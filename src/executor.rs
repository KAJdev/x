use crate::spinner::prompt_single_char;
use anyhow::Result;
use console::style;
use std::io::Write;
use tokio::process::Command as AsyncCommand;

pub fn confirm_execution(command: &str) -> Result<bool> {
    print!(
        "{} {}",
        style(">").cyan().bold(),
        style(command).white().bold()
    );

    print!("\n\n[y/n] ");
    std::io::stdout().flush().unwrap();

    let confirmed = prompt_single_char("")?;

    println!();

    Ok(confirmed)
}

pub async fn execute_command(command: &str) -> Result<()> {
    if command.trim().is_empty() {
        return Err(anyhow::anyhow!("Empty command"));
    }

    let mut cmd = AsyncCommand::new("sh");
    cmd.arg("-c").arg(command);

    let output = cmd.output().await?;

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        let exit_code = output.status.code().unwrap_or(-1);
        return Err(anyhow::anyhow!(
            "Command failed with exit code: {}",
            exit_code
        ));
    }

    Ok(())
}
