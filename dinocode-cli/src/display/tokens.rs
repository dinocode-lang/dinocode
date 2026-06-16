use colored::Colorize;
use dinocode::shared::types::Token;

pub fn display_tokens(tokens: &[Token]) {
    if tokens.is_empty() {
        println!("{}", "No tokens found.".yellow());
        return;
    }

    println!(
        "{:<4} {:<20} {:<25} {:<15} {:<20}",
        "#".bright_white().bold(),
        "Type".bright_white().bold(),
        "Value".bright_white().bold(),
        "Info".bright_white().bold(),
        "Attributes".bright_white().bold()
    );
    println!("{}", "─".repeat(80).bright_black());

    for (idx, token) in tokens.iter().enumerate() {
        let type_str = format!("{:?}", token.typ);
        let value_str = format!("{:?}", token.value);
        let value_truncated = if value_str.len() > 23 {
            format!("{}...", &value_str[..20])
        } else {
            value_str
        };

        let line = token.line.map(|l| l.to_string()).unwrap_or_else(|| "-".to_string());
        let col = token.column.map(|c| c.to_string()).unwrap_or_else(|| "-".to_string());
        let info = format!("Ln {}, Col {}", line, col);

        let mut attr_parts = Vec::new();
        if token.is_unary {
            attr_parts.push("unary");
        }
        if token.is_breaker {
            attr_parts.push("breaker");
        }
        let attrs_plain = if attr_parts.is_empty() {
            "-".to_string()
        } else {
            attr_parts.join(", ")
        };

        let attrs_str = if attrs_plain == "-" {
            attrs_plain.bright_black().to_string()
        } else {
            attrs_plain.bright_yellow().to_string()
        };

        println!(
            "{:<4} {:<20} {:<25} {:<15} {:<20}",
            idx.to_string().bright_black(),
            type_str.bright_cyan(),
            value_truncated.bright_green(),
            info,
            attrs_str
        );
    }

    println!();
    println!("{}", format!("Total tokens: {}", tokens.len()).bright_blue());
    println!();
}
