#![windows_subsystem = "windows"]
//! Setup.exe: abre o instalador PT-BR (tools/windows_setup.ps1) sem console.

#[cfg(windows)]
fn main() {
    use std::os::windows::process::CommandExt;
    let root = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let _ = std::process::Command::new("powershell.exe")
        .args(["-NoLogo", "-NoProfile", "-STA", "-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-File"])
        .arg(root.join("tools").join("windows_setup.ps1"))
        .current_dir(&root)
        .creation_flags(0x0800_0000)
        .spawn();
}

#[cfg(not(windows))]
fn main() {}
