use clap::Parser;

#[derive(Parser)]
#[command(name = "x")]
#[command(about = "A CLI tool that uses LLM to generate and execute commands")]
pub struct Cli {
    /// Configure the LLM provider and API key
    #[arg(long)]
    pub config: bool,

    /// The LLM provider to use (openai, claude) - only used with --config
    #[arg(long)]
    pub provider: Option<String>,

    /// The API key for the provider - only used with --config
    #[arg(long, alias = "key")]
    pub api_key: Option<String>,

    /// The natural language description of what you want to do
    pub command: Vec<String>,
}
