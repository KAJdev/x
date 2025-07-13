use crate::config::{Config, LlmProvider};
use crate::prompts::{generate_system_context, generate_system_prompt};
use crate::spinner::StreamingSpinner;
use anyhow::Result;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::{Value, json};
use std::env;

pub async fn generate_command(config: &Config, user_input: &[String]) -> Result<String> {
    let user_command = user_input.join(" ");
    let spinner = StreamingSpinner::new();
    spinner.start();

    spinner.update_text("");

    let system_context = get_system_context().await?;
    let system_prompt = generate_system_prompt(&system_context);

    let command = match config.provider {
        LlmProvider::OpenAI => {
            generate_with_openai_stream(config, &system_prompt, &user_command, &spinner).await?
        }
        LlmProvider::Claude => {
            generate_with_claude_stream(config, &system_prompt, &user_command, &spinner).await?
        }
    };

    spinner.stop();

    Ok(command)
}

async fn generate_with_openai_stream(
    config: &Config,
    system_prompt: &str,
    prompt: &str,
    spinner: &StreamingSpinner,
) -> Result<String> {
    let client = Client::new();

    let request_body = json!({
        "model": "gpt-4o",
        "messages": [
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 500,
        "stream": true,
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let mut stream = response.bytes_stream();
    let mut command = String::new();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let chunk_str = String::from_utf8_lossy(&chunk);

        process_stream_buffer(
            &mut buffer,
            &chunk_str,
            &mut command,
            &spinner,
            |json_value| json_value["choices"][0]["delta"]["content"].as_str(),
        )
        .await;
    }

    Ok(command.trim().to_string())
}

async fn generate_with_claude_stream(
    config: &Config,
    system_prompt: &str,
    prompt: &str,
    spinner: &StreamingSpinner,
) -> Result<String> {
    let client = Client::new();

    let request_body = json!({
        "model": "claude-3-5-sonnet-20241022",
        "max_tokens": 500,
        "system": system_prompt,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "stream": true,
    });

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .await?;

    let mut stream = response.bytes_stream();
    let mut command = String::new();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let chunk_str = String::from_utf8_lossy(&chunk);

        process_stream_buffer(
            &mut buffer,
            &chunk_str,
            &mut command,
            &spinner,
            |json_value| json_value["delta"]["text"].as_str(),
        )
        .await;
    }

    Ok(command.trim().to_string())
}

async fn process_stream_buffer(
    buffer: &mut String,
    chunk_str: &str,
    command: &mut String,
    spinner: &StreamingSpinner,
    extract_content: impl Fn(&Value) -> Option<&str>,
) {
    buffer.push_str(chunk_str);

    let mut lines_to_process = Vec::new();
    while let Some(line_end) = buffer.find('\n') {
        let line = buffer[..line_end].trim().to_string();
        lines_to_process.push(line);
        *buffer = buffer[line_end + 1..].to_string();
    }

    for line in lines_to_process {
        if line.starts_with("data: ") && !line.contains("[DONE]") {
            let json_str = &line[6..]; // remove "data: " prefix
            if let Ok(json_value) = serde_json::from_str::<Value>(json_str) {
                if let Some(content) = extract_content(&json_value) {
                    command.push_str(content);
                    spinner.update_text(command);
                }
            }
        }
    }
}

async fn get_system_context() -> Result<String> {
    let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let home = env::var("HOME").unwrap_or_else(|_| "unknown".to_string());
    let pwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let os = env::consts::OS;

    Ok(generate_system_context(&shell, &home, &pwd, &os))
}
