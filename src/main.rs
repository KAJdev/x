mod cli;
mod config;
mod executor;
mod llm;
mod prompts;
mod spinner;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::Config;
use console;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.config {
        config::handle_config(cli.provider, cli.api_key).await?;
        return Ok(());
    }

    if cli.command.is_empty() {
        let mut cmd = clap::Command::new("x")
            .about("A CLI tool that uses LLM to generate and execute commands")
            .version("0.1.0")
            .arg(
                clap::Arg::new("config")
                    .long("config")
                    .help("Configure the LLM provider and API key")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .help("The LLM provider to use (openai, claude)")
                    .value_name("PROVIDER"),
            )
            .arg(
                clap::Arg::new("api-key")
                    .long("api-key")
                    .alias("key")
                    .help("The API key for the provider")
                    .value_name("KEY"),
            )
            .arg(
                clap::Arg::new("command")
                    .help("The natural language description of what you want to do")
                    .value_name("COMMAND")
                    .action(clap::ArgAction::Append),
            );

        cmd.print_help()?;
        println!("\n\nExamples:");
        println!("  x ssh key named id_rsa");
        println!("  x create new git repository and initial commit");
        println!("  x install docker on ubuntu");
        println!("  x --config --provider openai --api-key your-key-here");
        return Ok(());
    }

    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            println!("🔧 No configuration found!");
            println!();
            println!("To get started, please configure your LLM provider:");
            println!(
                "  {} Interactive setup",
                console::style("x --config").cyan().bold()
            );
            println!(
                "  {} Direct setup",
                console::style("x --config --provider openai --api-key your-key")
                    .cyan()
                    .bold()
            );
            println!();
            println!("Supported providers: openai, claude");
            return Ok(());
        }
    };

    println!();

    let generated_command = llm::generate_command(&config, &cli.command).await?;

    if executor::confirm_execution(&generated_command)? {
        println!();
        executor::execute_command(&generated_command).await?;
    }

    Ok(())
}
