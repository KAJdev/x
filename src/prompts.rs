const SYSTEM_PROMPT_TEMPLATE: &str = "\
You are a helpful CLI assistant. Generate a single shell command for the following user request.

System context:
{system_context}

Rules:
- Return ONLY the command, no explanations or markdown
- Make sure the command is safe and appropriate
- Use the user's current shell and environment
- If creating files, use appropriate paths
- This will be executed directly: no explanations, no markdown
- For SSH keys, use standard locations like ~/.ssh/";

const SYSTEM_CONTEXT_TEMPLATE: &str = "\
Shell: {shell}
Home: {home}
Current directory: {cwd}
OS: {os}";

pub fn generate_system_prompt(system_context: &str) -> String {
    SYSTEM_PROMPT_TEMPLATE.replace("{system_context}", system_context)
}

pub fn generate_system_context(shell: &str, home: &str, cwd: &str, os: &str) -> String {
    SYSTEM_CONTEXT_TEMPLATE
        .replace("{shell}", shell)
        .replace("{home}", home)
        .replace("{cwd}", cwd)
        .replace("{os}", os)
}
