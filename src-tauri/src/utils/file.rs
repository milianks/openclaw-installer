use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// 读取文件内容
pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

/// 写入文件内容
pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    // 确保父目录存在
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

/// 检查文件是否存在
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// 读取文件最后 N 行
pub fn read_last_lines(path: &str, n: usize) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    let start = if lines.len() > n { lines.len() - n } else { 0 };
    Ok(lines[start..].to_vec())
}

fn validate_env_key(key: &str) -> io::Result<()> {
    let mut chars = key.chars();
    let Some(first) = chars.next() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "环境变量名不能为空",
        ));
    };

    if !(first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("非法环境变量名: {}", key),
        ));
    }

    Ok(())
}

fn escape_env_value(value: &str) -> io::Result<String> {
    if value.contains('\n') || value.contains('\r') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "环境变量值不能包含换行符",
        ));
    }

    let mut escaped = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '$' => escaped.push_str("\\$"),
            '`' => escaped.push_str("\\`"),
            _ => escaped.push(c),
        }
    }

    Ok(escaped)
}

fn unescape_env_value(value: &str) -> String {
    let mut unescaped = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                unescaped.push(next);
            }
        } else {
            unescaped.push(c);
        }
    }
    unescaped
}

fn parse_env_value(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        unescape_env_value(&trimmed[1..trimmed.len() - 1])
    } else if trimmed.len() >= 2 && trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

/// 从环境变量文件读取值
pub fn read_env_value(env_file: &str, key: &str) -> Option<String> {
    let content = read_file(env_file).ok()?;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(&format!("export {}=", key)) {
            let value = line.trim_start_matches(&format!("export {}=", key));
            return Some(parse_env_value(value));
        }
    }

    None
}

/// 设置环境变量文件中的值
pub fn set_env_value(env_file: &str, key: &str, value: &str) -> io::Result<()> {
    validate_env_key(key)?;
    let escaped_value = escape_env_value(value)?;
    let content = read_file(env_file).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let new_line = format!("export {}=\"{}\"", key, escaped_value);
    let mut found = false;

    for line in &mut lines {
        if line.starts_with(&format!("export {}=", key)) {
            *line = new_line.clone();
            found = true;
            break;
        }
    }

    if !found {
        lines.push(new_line);
    }

    write_file(env_file, &lines.join("\n"))
}

/// 从环境变量文件中删除指定的值
pub fn remove_env_value(env_file: &str, key: &str) -> io::Result<()> {
    validate_env_key(key)?;
    let content = read_file(env_file).unwrap_or_default();
    let lines: Vec<String> = content
        .lines()
        .filter(|line| !line.starts_with(&format!("export {}=", key)))
        .map(|s| s.to_string())
        .collect();

    write_file(env_file, &lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::{read_env_value, set_env_value};

    fn temp_env_file(name: &str) -> String {
        std::env::temp_dir()
            .join(format!(
                "openclaw-manager-file-test-{}-{}",
                std::process::id(),
                name
            ))
            .display()
            .to_string()
    }

    #[test]
    fn set_env_value_rejects_invalid_keys() {
        let path = temp_env_file("invalid-key");
        let result = set_env_value(&path, "BAD KEY", "value");
        let _ = std::fs::remove_file(path);

        assert!(result.is_err());
    }

    #[test]
    fn set_env_value_rejects_multiline_values() {
        let path = temp_env_file("multiline-value");
        let result = set_env_value(&path, "OPENCLAW_TEST", "line1\nline2");
        let _ = std::fs::remove_file(path);

        assert!(result.is_err());
    }

    #[test]
    fn set_env_value_escapes_shell_sensitive_characters_and_round_trips() {
        let path = temp_env_file("escaping");
        let value = r#"abc "quoted" $HOME `cmd` \ slash"#;

        set_env_value(&path, "OPENCLAW_TEST", value).expect("value should be written");
        let content = std::fs::read_to_string(&path).expect("env file should be readable");
        let loaded = read_env_value(&path, "OPENCLAW_TEST").expect("value should round trip");
        let _ = std::fs::remove_file(path);

        assert_eq!(
            content,
            r#"export OPENCLAW_TEST="abc \"quoted\" \$HOME \`cmd\` \\ slash""#
        );
        assert_eq!(loaded, value);
    }
}
