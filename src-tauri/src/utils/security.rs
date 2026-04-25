#[cfg(unix)]
use std::io::Read;

/// 生成用于本地 Gateway 认证的安全 token。
pub fn generate_secure_token() -> Result<String, String> {
    let bytes = secure_random_bytes(32)?;
    Ok(format!("ocm-{}", to_hex(&bytes)))
}

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", byte);
    }
    out
}

#[cfg(unix)]
fn secure_random_bytes(len: usize) -> Result<Vec<u8>, String> {
    let mut bytes = vec![0u8; len];
    std::fs::File::open("/dev/urandom")
        .map_err(|e| format!("打开系统随机源失败: {}", e))?
        .read_exact(&mut bytes)
        .map_err(|e| format!("读取系统随机源失败: {}", e))?;
    Ok(bytes)
}

#[cfg(windows)]
fn secure_random_bytes(len: usize) -> Result<Vec<u8>, String> {
    let command = format!(
        "[Convert]::ToHexString([System.Security.Cryptography.RandomNumberGenerator]::GetBytes({})).ToLowerInvariant()",
        len
    );
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &command])
        .output()
        .map_err(|e| format!("调用 Windows 随机源失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let hex = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if hex.len() != len * 2 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Windows 随机源返回了无效数据".to_string());
    }

    let mut bytes = Vec::with_capacity(len);
    for pair in hex.as_bytes().chunks(2) {
        let s = std::str::from_utf8(pair).map_err(|e| e.to_string())?;
        bytes.push(u8::from_str_radix(s, 16).map_err(|e| e.to_string())?);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::generate_secure_token;

    #[test]
    fn generate_secure_token_returns_ocm_prefixed_32_byte_hex_token() {
        let token = generate_secure_token().expect("token should be generated");

        assert!(token.starts_with("ocm-"));
        assert_eq!(token.len(), 68);
        assert!(token[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn generate_secure_token_returns_different_values() {
        let first = generate_secure_token().expect("first token");
        let second = generate_secure_token().expect("second token");

        assert_ne!(first, second);
    }
}
