//! Windows registry read helpers for Jetbrains Rustrover.

pub const APP_KEY: &str = "Software\\Jetbrains\\Jetbrains Rustrover";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Jetbrains Rustrover";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Jetbrains Rustrover") },
        RegValue { name: "Version",     value: String::from("v3.0.22") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Jetbrains Rustrover") },
        RegValue { name: "Publisher",       value: String::from("Jetbrains") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Jetbrains Rustrover\\uninstall.exe\"") },
    ]
}
