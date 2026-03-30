use std::io::{self, Write};

pub fn prompt(label: &str) -> Result<String, String> {
    print!("{label}: ");
    io::stdout().flush().map_err(|err| err.to_string())?;

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .map_err(|err| err.to_string())?;

    Ok(value.trim().to_string())
}

pub fn prompt_optional(label: &str) -> Result<Option<String>, String> {
    let value = prompt(label)?;
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

pub fn confirm(label: &str) -> Result<bool, String> {
    let value = prompt(label)?;
    Ok(matches!(value.to_lowercase().as_str(), "y" | "yes"))
}