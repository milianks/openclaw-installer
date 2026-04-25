use crate::utils::file;
use crate::utils::platform;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::io;
use std::process::{Command, Output};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// Windows CREATE_NO_WINDOW 标志，用于隐藏控制台窗口
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub const DEFAULT_NPM_REGISTRY: &str = "https://registry.npmmirror.com";

pub fn resolve_npm_registry_from_env(
    openclaw_registry: Option<&str>,
    npm_config_registry: Option<&str>,
    npm_config_registry_lower: Option<&str>,
) -> String {
    for registry in [
        openclaw_registry,
        npm_config_registry,
        npm_config_registry_lower,
    ]
    .into_iter()
    .flatten()
    {
        let registry = registry.trim();
        if !registry.is_empty() {
            return registry.to_string();
        }
    }

    DEFAULT_NPM_REGISTRY.to_string()
}

fn resolve_npm_registry() -> String {
    let openclaw_registry = std::env::var("OPENCLAW_NPM_REGISTRY").ok();
    let npm_config_registry = std::env::var("NPM_CONFIG_REGISTRY").ok();
    let npm_config_registry_lower = std::env::var("npm_config_registry").ok();

    resolve_npm_registry_from_env(
        openclaw_registry.as_deref(),
        npm_config_registry.as_deref(),
        npm_config_registry_lower.as_deref(),
    )
}

fn apply_npm_registry_env(command: &mut Command) {
    let registry = resolve_npm_registry();
    command.env("NPM_CONFIG_REGISTRY", &registry);
    command.env("npm_config_registry", &registry);
}

fn apply_extended_path_env(command: &mut Command) {
    let extended_path = get_extended_path();
    command.env("PATH", extended_path);
}

fn apply_child_env(command: &mut Command) {
    apply_npm_registry_env(command);
    apply_extended_path_env(command);
}

pub fn npm_registry_bash_prelude() -> String {
    format!(
        r#"export npm_config_registry="${{OPENCLAW_NPM_REGISTRY:-${{NPM_CONFIG_REGISTRY:-{}}}}}"
export NPM_CONFIG_REGISTRY="$npm_config_registry"
echo "使用 npm registry: $npm_config_registry"
if command -v npm >/dev/null 2>&1; then
  npm config set registry "$npm_config_registry" --location=user >/dev/null 2>&1 || true
fi
"#,
        DEFAULT_NPM_REGISTRY
    )
}

pub fn npm_registry_powershell_prelude() -> String {
    format!(
        r#"$registry = $env:OPENCLAW_NPM_REGISTRY
if (-not $registry) {{ $registry = $env:NPM_CONFIG_REGISTRY }}
if (-not $registry) {{ $registry = "{}" }}
$env:npm_config_registry = $registry
$env:NPM_CONFIG_REGISTRY = $registry
Write-Host "使用 npm registry: $registry"
if (Get-Command npm -ErrorAction SilentlyContinue) {{
  npm config set registry $registry --location=user | Out-Null
}}
"#,
        DEFAULT_NPM_REGISTRY
    )
}

/// 获取扩展的 PATH 环境变量
/// GUI 应用启动时可能没有继承用户 shell 的 PATH，需要手动添加常见路径
pub fn get_extended_path() -> String {
    let mut paths = vec![
        "/opt/homebrew/bin".to_string(), // Homebrew on Apple Silicon
        "/usr/local/bin".to_string(),    // Homebrew on Intel / 常规安装
        "/usr/bin".to_string(),
        "/bin".to_string(),
    ];

    if let Some(home) = dirs::home_dir() {
        let home_str = home.display().to_string();

        // nvm 路径（尝试获取当前版本）
        let nvm_default = format!("{}/.nvm/alias/default", home_str);
        if let Ok(version) = std::fs::read_to_string(&nvm_default) {
            let version = version.trim();
            if !version.is_empty() {
                paths.insert(
                    0,
                    format!("{}/.nvm/versions/node/v{}/bin", home_str, version),
                );
            }
        }
        // 动态扫描 nvm 已安装版本目录，避免 PATH 只覆盖写死版本
        let nvm_versions_dir = std::path::Path::new(&home).join(".nvm/versions/node");
        if let Ok(entries) = std::fs::read_dir(&nvm_versions_dir) {
            let mut bins = Vec::new();
            for entry in entries.flatten() {
                let nvm_bin = entry.path().join("bin");
                if nvm_bin.exists() {
                    bins.push(nvm_bin.display().to_string());
                }
            }
            bins.sort();
            bins.reverse();
            for bin in bins {
                paths.push(bin);
            }
        }

        // 也添加常见 nvm 版本路径作为兜底
        for version in ["v22.22.0", "v22.12.0", "v22.11.0", "v22.0.0", "v23.0.0"] {
            paths.push(format!("{}/.nvm/versions/node/{}/bin", home_str, version));
        }

        // fnm
        paths.push(format!("{}/.fnm/aliases/default/bin", home_str));

        // volta
        paths.push(format!("{}/.volta/bin", home_str));

        // asdf
        paths.push(format!("{}/.asdf/shims", home_str));

        // mise
        paths.push(format!("{}/.local/share/mise/shims", home_str));
    }

    // 获取当前 PATH 并合并
    let current_path = std::env::var("PATH").unwrap_or_default();
    if !current_path.is_empty() {
        paths.push(current_path);
    }

    let separator = if platform::is_windows() { ";" } else { ":" };
    paths.join(separator)
}

/// 执行 Shell 命令（带扩展 PATH）
pub fn run_command(cmd: &str, args: &[&str]) -> io::Result<Output> {
    let mut command = Command::new(cmd);
    command.args(args);
    apply_child_env(&mut command);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    command.output()
}

/// 执行 Shell 命令并获取输出字符串
pub fn run_command_output(cmd: &str, args: &[&str]) -> Result<String, String> {
    match run_command(cmd, args) {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// 执行 Bash 命令（带扩展 PATH）
pub fn run_bash(script: &str) -> io::Result<Output> {
    let mut command = Command::new("bash");
    command.arg("-c").arg(script);
    apply_child_env(&mut command);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    command.output()
}

/// 执行 Bash 命令并获取输出
pub fn run_bash_output(script: &str) -> Result<String, String> {
    match run_bash(script) {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                if stderr.is_empty() {
                    Err(format!(
                        "Command failed with exit code: {:?}",
                        output.status.code()
                    ))
                } else {
                    Err(stderr)
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// 执行 cmd.exe 命令（Windows）- 避免 PowerShell 执行策略问题
pub fn run_cmd(script: &str) -> io::Result<Output> {
    let mut cmd = Command::new("cmd");
    cmd.args(["/c", script]);
    apply_npm_registry_env(&mut cmd);

    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    cmd.output()
}

/// 执行 cmd.exe 命令并获取输出（Windows）
pub fn run_cmd_output(script: &str) -> Result<String, String> {
    match run_cmd(script) {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                if stderr.is_empty() {
                    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if stdout.is_empty() {
                        Err(format!(
                            "Command failed with exit code: {:?}",
                            output.status.code()
                        ))
                    } else {
                        Err(stdout)
                    }
                } else {
                    Err(stderr)
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// 执行 PowerShell 命令（Windows）- 仅在需要 PowerShell 特定功能时使用
/// 注意：某些 Windows 系统的 PowerShell 执行策略可能禁止运行脚本
pub fn run_powershell(script: &str) -> io::Result<Output> {
    let mut cmd = Command::new("powershell");
    // 使用 -ExecutionPolicy Bypass 绕过执行策略限制
    cmd.args([
        "-NoProfile",
        "-NonInteractive",
        "-ExecutionPolicy",
        "Bypass",
        "-Command",
        script,
    ]);
    apply_npm_registry_env(&mut cmd);

    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    cmd.output()
}

/// 执行 PowerShell 命令并获取输出（Windows）
pub fn run_powershell_output(script: &str) -> Result<String, String> {
    match run_powershell(script) {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                if stderr.is_empty() {
                    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if stdout.is_empty() {
                        Err(format!(
                            "Command failed with exit code: {:?}",
                            output.status.code()
                        ))
                    } else {
                        Err(stdout)
                    }
                } else {
                    Err(stderr)
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// 跨平台执行脚本命令
/// Windows 上使用 cmd.exe（避免 PowerShell 执行策略问题）
pub fn run_script_output(script: &str) -> Result<String, String> {
    if platform::is_windows() {
        run_cmd_output(script)
    } else {
        run_bash_output(script)
    }
}

/// 获取 openclaw 可执行文件路径
/// 检测多个可能的安装路径，因为 GUI 应用不继承用户 shell 的 PATH
pub fn get_openclaw_path() -> Option<String> {
    // Windows: 检查常见的 npm 全局安装路径
    if platform::is_windows() {
        let possible_paths = get_windows_openclaw_paths();
        for path in possible_paths {
            if std::path::Path::new(&path).exists() {
                info!("[Shell] 在 {} 找到 openclaw", path);
                return Some(path);
            }
        }
    } else {
        // Unix: 检查常见的 npm 全局安装路径
        let possible_paths = get_unix_openclaw_paths();
        for path in possible_paths {
            if std::path::Path::new(&path).exists() {
                info!("[Shell] 在 {} 找到 openclaw", path);
                return Some(path);
            }
        }
    }

    // 回退：检查是否在 PATH 中
    if command_exists("openclaw") {
        return Some("openclaw".to_string());
    }

    // 最后尝试：通过用户 shell 查找
    if !platform::is_windows() {
        if let Ok(path) = run_bash_output("source ~/.zshrc 2>/dev/null || source ~/.bashrc 2>/dev/null; which openclaw 2>/dev/null") {
            if !path.is_empty() && std::path::Path::new(&path).exists() {
                info!("[Shell] 通过用户 shell 找到 openclaw: {}", path);
                return Some(path);
            }
        }
    }

    None
}

/// 获取 Unix 系统上可能的 openclaw 安装路径
fn get_unix_openclaw_paths() -> Vec<String> {
    let mut paths = Vec::new();

    // npm 全局安装路径
    paths.push("/usr/local/bin/openclaw".to_string());
    paths.push("/opt/homebrew/bin/openclaw".to_string()); // Homebrew on Apple Silicon
    paths.push("/usr/bin/openclaw".to_string());

    if let Some(home) = dirs::home_dir() {
        let home_str = home.display().to_string();

        // npm 全局安装到用户目录
        paths.push(format!("{}/.npm-global/bin/openclaw", home_str));

        // nvm 安装的 npm 全局包：优先 default alias，再动态扫描已安装版本
        let nvm_default = format!("{}/.nvm/alias/default", home_str);
        if let Ok(version) = std::fs::read_to_string(&nvm_default) {
            let version = version.trim();
            if !version.is_empty() {
                let normalized = if version.starts_with('v') {
                    version.to_string()
                } else {
                    format!("v{}", version)
                };
                paths.push(format!(
                    "{}/.nvm/versions/node/{}/bin/openclaw",
                    home_str, normalized
                ));
            }
        }

        let nvm_versions_dir = std::path::Path::new(&home).join(".nvm/versions/node");
        if let Ok(entries) = std::fs::read_dir(&nvm_versions_dir) {
            let mut version_paths = Vec::new();
            for entry in entries.flatten() {
                let openclaw_path = entry.path().join("bin/openclaw");
                if openclaw_path.exists() {
                    version_paths.push(openclaw_path.display().to_string());
                }
            }
            version_paths.sort();
            version_paths.reverse();
            paths.extend(version_paths);
        }

        // 常见回退版本
        for version in ["v22.22.0", "v22.12.0", "v22.11.0", "v22.0.0", "v23.0.0"] {
            paths.push(format!(
                "{}/.nvm/versions/node/{}/bin/openclaw",
                home_str, version
            ));
        }

        // fnm
        paths.push(format!("{}/.fnm/aliases/default/bin/openclaw", home_str));

        // volta
        paths.push(format!("{}/.volta/bin/openclaw", home_str));

        // pnpm 全局安装
        paths.push(format!("{}/.pnpm/bin/openclaw", home_str));
        paths.push(format!("{}/Library/pnpm/openclaw", home_str)); // macOS pnpm 默认路径

        // asdf
        paths.push(format!("{}/.asdf/shims/openclaw", home_str));

        // mise (formerly rtx)
        paths.push(format!("{}/.local/share/mise/shims/openclaw", home_str));

        // yarn 全局安装
        paths.push(format!("{}/.yarn/bin/openclaw", home_str));
        paths.push(format!(
            "{}/.config/yarn/global/node_modules/.bin/openclaw",
            home_str
        ));
    }

    paths
}

/// 获取 Windows 上可能的 openclaw 安装路径
fn get_windows_openclaw_paths() -> Vec<String> {
    let mut paths = Vec::new();

    // 1. nvm4w 安装路径
    paths.push("C:\\nvm4w\\nodejs\\openclaw.cmd".to_string());

    // 2. 用户目录下的 npm 全局路径
    if let Some(home) = dirs::home_dir() {
        let npm_path = format!("{}\\AppData\\Roaming\\npm\\openclaw.cmd", home.display());
        paths.push(npm_path);
    }

    // 3. Program Files 下的 nodejs
    paths.push("C:\\Program Files\\nodejs\\openclaw.cmd".to_string());

    paths
}

/// 执行 openclaw 命令并获取输出
pub fn run_openclaw(args: &[&str]) -> Result<String, String> {
    debug!("[Shell] 执行 openclaw 命令: {:?}", args);

    let openclaw_path = get_openclaw_path().ok_or_else(|| {
        warn!("[Shell] 找不到 openclaw 命令");
        "找不到 openclaw 命令，请确保已通过 npm install -g openclaw 安装".to_string()
    })?;

    debug!("[Shell] openclaw 路径: {}", openclaw_path);

    let gateway_token =
        resolve_gateway_token().unwrap_or_else(|| DEFAULT_GATEWAY_TOKEN.to_string());

    let output = if openclaw_path.ends_with(".cmd") {
        // Windows: .cmd 文件需要通过 cmd /c 执行
        let mut cmd_args = vec!["/c", &openclaw_path];
        cmd_args.extend(args);
        let mut cmd = Command::new("cmd");
        cmd.args(&cmd_args)
            .env("OPENCLAW_GATEWAY_TOKEN", &gateway_token);
        apply_child_env(&mut cmd);

        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.output()
    } else {
        let mut cmd = Command::new(&openclaw_path);
        cmd.args(args).env("OPENCLAW_GATEWAY_TOKEN", &gateway_token);
        apply_child_env(&mut cmd);

        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.output()
    };

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            debug!("[Shell] 命令退出码: {:?}", out.status.code());
            if out.status.success() {
                debug!("[Shell] 命令执行成功, stdout 长度: {}", stdout.len());
                Ok(stdout)
            } else {
                debug!("[Shell] 命令执行失败, stderr: {}", stderr);
                Err(format!("{}\n{}", stdout, stderr).trim().to_string())
            }
        }
        Err(e) => {
            warn!("[Shell] 执行 openclaw 失败: {}", e);
            Err(format!("执行 openclaw 失败: {}", e))
        }
    }
}

/// 默认的 Gateway Token
pub const DEFAULT_GATEWAY_TOKEN: &str = "openclaw-manager-local-token";

fn read_gateway_token_from_config() -> Option<String> {
    let config_path = platform::get_config_file_path();
    let content = file::read_file(&config_path).ok()?;
    let config: serde_json::Value = serde_json::from_str(&content).ok()?;
    config
        .pointer("/gateway/auth/token")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_string)
}

pub fn resolve_gateway_token() -> Option<String> {
    read_gateway_token_from_config().or_else(|| {
        let env_path = platform::get_env_file_path();
        file::read_env_value(&env_path, "OPENCLAW_GATEWAY_TOKEN")
            .map(|token| token.trim().to_string())
            .filter(|token| !token.is_empty())
    })
}

/// 从 ~/.openclaw/env 文件读取所有环境变量
/// 与 shell 脚本 `source ~/.openclaw/env` 行为一致
fn unescape_env_value(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                out.push(next);
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn load_openclaw_env_vars() -> HashMap<String, String> {
    let mut env_vars = HashMap::new();
    let env_path = platform::get_env_file_path();

    if let Ok(content) = file::read_file(&env_path) {
        for line in content.lines() {
            let line = line.trim();
            // 跳过注释和空行
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // 解析 export KEY=VALUE 或 KEY=VALUE 格式
            let line = line.strip_prefix("export ").unwrap_or(line);
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                // 去除值周围的引号
                let value = value.trim().trim_matches('"').trim_matches('\'');
                env_vars.insert(key.to_string(), unescape_env_value(value));
            }
        }
    }

    env_vars
}

/// 后台启动 openclaw gateway
/// 与 shell 脚本行为一致：先加载 env 文件，再启动 gateway
pub fn spawn_openclaw_gateway() -> io::Result<()> {
    info!("[Shell] 后台启动 openclaw gateway...");

    let openclaw_path = get_openclaw_path().ok_or_else(|| {
        warn!("[Shell] 找不到 openclaw 命令");
        io::Error::new(
            io::ErrorKind::NotFound,
            "找不到 openclaw 命令，请确保已通过 npm install -g openclaw 安装",
        )
    })?;

    info!("[Shell] openclaw 路径: {}", openclaw_path);

    // 加载用户的 env 文件环境变量（与 shell 脚本 source ~/.openclaw/env 一致）
    info!("[Shell] 加载用户环境变量...");
    let user_env_vars = load_openclaw_env_vars();
    info!("[Shell] 已加载 {} 个环境变量", user_env_vars.len());
    for key in user_env_vars.keys() {
        debug!("[Shell] - 环境变量: {}", key);
    }

    // Windows 上 .cmd 文件需要通过 cmd /c 来执行
    // 设置环境变量 OPENCLAW_GATEWAY_TOKEN，这样所有子命令都能自动使用
    let mut cmd = if openclaw_path.ends_with(".cmd") {
        info!("[Shell] Windows 模式: 使用 cmd /c 执行");
        let mut c = Command::new("cmd");
        c.args(["/c", &openclaw_path, "gateway", "--port", "18789"]);
        c
    } else {
        info!("[Shell] Unix 模式: 直接执行");
        let mut c = Command::new(&openclaw_path);
        c.args(["gateway", "--port", "18789"]);
        c
    };

    // 注入用户的环境变量（如 ANTHROPIC_API_KEY, OPENAI_API_KEY 等）
    for (key, value) in &user_env_vars {
        cmd.env(key, value);
    }

    // 设置 PATH、npm 镜像和 gateway token。token 优先来自 openclaw.json，其次 env 文件。
    apply_child_env(&mut cmd);
    let gateway_token =
        resolve_gateway_token().unwrap_or_else(|| DEFAULT_GATEWAY_TOKEN.to_string());
    cmd.env("OPENCLAW_GATEWAY_TOKEN", gateway_token);

    // Windows: 隐藏控制台窗口
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    // 将 stdout/stderr 重定向到日志文件，以便 get_logs 可以读取
    let logs_dir = format!("{}/logs", platform::get_config_dir());
    let _ = std::fs::create_dir_all(&logs_dir);

    let stdout_log_path = format!("{}/gateway.log", logs_dir);
    let stderr_log_path = format!("{}/gateway.err.log", logs_dir);

    info!(
        "[Shell] 日志输出到: {} / {}",
        stdout_log_path, stderr_log_path
    );

    if let Ok(stdout_file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&stdout_log_path)
    {
        cmd.stdout(std::process::Stdio::from(stdout_file));
    }
    if let Ok(stderr_file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&stderr_log_path)
    {
        cmd.stderr(std::process::Stdio::from(stderr_file));
    }

    info!("[Shell] 启动 gateway 进程...");
    let child = cmd.spawn();

    match child {
        Ok(c) => {
            info!("[Shell] ✓ Gateway 进程已启动, PID: {}", c.id());
            Ok(())
        }
        Err(e) => {
            warn!("[Shell] ✗ Gateway 启动失败: {}", e);
            Err(io::Error::new(
                e.kind(),
                format!("启动失败 (路径: {}): {}", openclaw_path, e),
            ))
        }
    }
}

/// 检查命令是否存在
pub fn command_exists(cmd: &str) -> bool {
    if platform::is_windows() {
        // Windows: 使用 where 命令
        let mut command = Command::new("where");
        command.arg(cmd);

        #[cfg(windows)]
        command.creation_flags(CREATE_NO_WINDOW);

        command
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    } else {
        // Unix: 使用 which 命令
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::{resolve_npm_registry_from_env, DEFAULT_NPM_REGISTRY};

    #[test]
    fn resolve_npm_registry_defaults_to_china_mirror() {
        assert_eq!(
            resolve_npm_registry_from_env(None, None, None),
            DEFAULT_NPM_REGISTRY
        );
    }

    #[test]
    fn resolve_npm_registry_ignores_empty_overrides() {
        assert_eq!(
            resolve_npm_registry_from_env(Some(""), Some("   "), None),
            DEFAULT_NPM_REGISTRY
        );
    }

    #[test]
    fn resolve_npm_registry_prefers_explicit_openclaw_override() {
        assert_eq!(
            resolve_npm_registry_from_env(
                Some("https://custom.example"),
                Some("https://npm.example"),
                None
            ),
            "https://custom.example"
        );
    }

    #[test]
    fn resolve_npm_registry_respects_existing_npm_registry() {
        assert_eq!(
            resolve_npm_registry_from_env(None, Some("https://npm.example"), None),
            "https://npm.example"
        );
    }
}
