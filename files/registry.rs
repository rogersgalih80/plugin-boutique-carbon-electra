//! Windows registry read helpers for Plugin Boutique Carbon Electra.

pub const APP_KEY: &str = "Software\\Plugin\\Plugin Boutique Carbon Electra";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Plugin Boutique Carbon Electra";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Plugin Boutique Carbon Electra") },
        RegValue { name: "Version",     value: String::from("v1.4.6") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Plugin Boutique Carbon Electra") },
        RegValue { name: "Publisher",       value: String::from("Plugin") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Plugin Boutique Carbon Electra\\uninstall.exe\"") },
    ]
}
