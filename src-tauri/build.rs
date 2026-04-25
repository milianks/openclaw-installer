use std::{env, fs, path::PathBuf};
use tauri_build::{Attributes, WindowsAttributes};

fn target_build_root() -> Option<PathBuf> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR")?);
    Some(out_dir.parent()?.parent()?.to_path_buf())
}

fn set_env_from_build_output(env_key: &str, dir_prefix: &str, filename: &str) {
    if env::var_os(env_key).is_some() {
        return;
    }

    let Some(build_root) = target_build_root() else {
        return;
    };

    let Ok(entries) = fs::read_dir(build_root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if !name.starts_with(dir_prefix) {
            continue;
        }

        let candidate = path.join("out").join(filename);
        if candidate.exists() {
            env::set_var(env_key, candidate);
            break;
        }
    }
}

fn hydrate_tauri_acl_env() {
    let core_permissions = [
        (
            "DEP_TAURI_CORE__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-permission-files",
        ),
        (
            "DEP_TAURI_CORE_APP__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-app-permission-files",
        ),
        (
            "DEP_TAURI_CORE_EVENT__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-event-permission-files",
        ),
        (
            "DEP_TAURI_CORE_IMAGE__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-image-permission-files",
        ),
        (
            "DEP_TAURI_CORE_MENU__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-menu-permission-files",
        ),
        (
            "DEP_TAURI_CORE_PATH__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-path-permission-files",
        ),
        (
            "DEP_TAURI_CORE_RESOURCES__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-resources-permission-files",
        ),
        (
            "DEP_TAURI_CORE_TRAY__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-tray-permission-files",
        ),
        (
            "DEP_TAURI_CORE_WEBVIEW__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-webview-permission-files",
        ),
        (
            "DEP_TAURI_CORE_WINDOW__CORE_PLUGIN___PERMISSION_FILES_PATH",
            "tauri-core-window-permission-files",
        ),
    ];

    for (env_key, filename) in core_permissions {
        set_env_from_build_output(env_key, "tauri-", filename);
    }

    let plugin_permissions = [
        (
            "DEP_TAURI_PLUGIN_SHELL_PERMISSION_FILES_PATH",
            "tauri-plugin-shell-",
            "tauri-plugin-shell-permission-files",
        ),
        (
            "DEP_TAURI_PLUGIN_FS_PERMISSION_FILES_PATH",
            "tauri-plugin-fs-",
            "tauri-plugin-fs-permission-files",
        ),
        (
            "DEP_TAURI_PLUGIN_PROCESS_PERMISSION_FILES_PATH",
            "tauri-plugin-process-",
            "tauri-plugin-process-permission-files",
        ),
        (
            "DEP_TAURI_PLUGIN_NOTIFICATION_PERMISSION_FILES_PATH",
            "tauri-plugin-notification-",
            "tauri-plugin-notification-permission-files",
        ),
    ];

    for (env_key, dir_prefix, filename) in plugin_permissions {
        set_env_from_build_output(env_key, dir_prefix, filename);
    }

    let plugin_scopes = [
        (
            "DEP_TAURI_PLUGIN_SHELL_GLOBAL_SCOPE_SCHEMA_PATH",
            "tauri-plugin-shell-",
            "global-scope.json",
        ),
        (
            "DEP_TAURI_PLUGIN_FS_GLOBAL_SCOPE_SCHEMA_PATH",
            "tauri-plugin-fs-",
            "global-scope.json",
        ),
    ];

    for (env_key, dir_prefix, filename) in plugin_scopes {
        set_env_from_build_output(env_key, dir_prefix, filename);
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=icons/icon.ico");
    println!("cargo:rerun-if-changed=icons/icon.png");
    println!("cargo:rerun-if-changed=icons/32x32.png");
    println!("cargo:rerun-if-changed=icons/128x128.png");
    println!("cargo:rerun-if-changed=icons/128x128@2x.png");

    if std::env::var("DEP_TAURI_DEV").is_err() {
        let dev = std::env::var_os("CARGO_FEATURE_CUSTOM_PROTOCOL").is_none();
        std::env::set_var("DEP_TAURI_DEV", if dev { "true" } else { "false" });
    }

    hydrate_tauri_acl_env();

    let temp_icon_dir = env::temp_dir().join("openclaw-manager-build");
    let _ = fs::create_dir_all(&temp_icon_dir);
    let temp_icon_path = temp_icon_dir.join("icon.ico");
    let _ = fs::copy("icons/icon.ico", &temp_icon_path);

    let attributes = Attributes::new().windows_attributes(
        WindowsAttributes::new().window_icon_path(&temp_icon_path),
    );

    if let Err(error) = tauri_build::try_build(attributes) {
        panic!("{error:#}");
    }
}
