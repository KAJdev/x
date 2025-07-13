const SYSTEM_PROMPT_TEMPLATE: &str = "\
You are a helpful CLI assistant. Generate a single shell command for the following user request.

System context:
{system_context}

Rules:
- Return ONLY the command, no explanations or markdown
- Use the user's current shell and environment
- This will be executed directly: no explanations or markdown wrapping
- Do not use any special characters that would require escaping
- Do not include any comments or explanations
- Do not use any backticks or code blocks
- Do not use any special formatting
- Do not use any quotes around the command
- the string returned will be DIRECTLY executed. it MUST BE VALID SHELL SYNTAX WITH NO MARKDOWN";

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
