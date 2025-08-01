// Update the compiler to the latest version
pub fn update_compiler() -> Result<(), String> {
    println!("Pidgin Compiler Update");
    println!("=====================");

    // Detect platform
    let platform = if cfg!(target_os = "windows") {
        "windows-x86_64"
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "macos-aarch64"
        } else {
            "macos-x86_64"
        }
    } else if cfg!(target_os = "linux") {
        if cfg!(target_arch = "aarch64") {
            "linux-aarch64"
        } else {
            "linux-x86_64"
        }
    } else {
        return Err("Unsupported platform".to_string());
    };

    println!("Detected platform: {platform}");

    // Get current version
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: v{current_version}");

    // Get latest version from GitHub API
    println!("Checking for latest version...");
    match get_latest_version() {
        Ok(latest_version) => {
            println!("Latest version: {latest_version}");

            // Check if update is needed
            if latest_version == format!("v{current_version}") {
                println!("✓ You already have the latest version!");
                return Ok(());
            }

            // Download and install update
            println!("Downloading update...");
            download_and_install_update(&latest_version, platform)?;

            println!("✓ Update completed successfully!");
            println!("New version: {latest_version}");
        }
        Err(e) => {
            println!("⚠️  Could not check for updates: {e}");
            println!("Current version: v{current_version}");
            println!("To get the latest version, please visit:");
            println!("https://github.com/ojutalayomi/pidgin/releases");
            return Ok(());
        }
    }

    Ok(())
}

// Get the latest version from GitHub releases
fn get_latest_version() -> Result<String, String> {
    use std::process::Command;

    let output = Command::new("curl")
        .args([
            "-s",
            "https://api.github.com/repos/ojutalayomi/pidgin/releases/latest",
        ])
        .output()
        .map_err(|e| format!("Failed to execute curl: {e}"))?;

    if !output.status.success() {
        // Check if it's a 404 (repository not found) or other error
        let status_code = output.status.code().unwrap_or(0);
        if status_code == 404 {
            return Err(
                "Repository not found. Please check if the repository exists and is public."
                    .to_string(),
            );
        } else {
            return Err(format!(
                "Failed to fetch latest version (HTTP {status_code})"
            ));
        }
    }

    let response =
        String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 response: {e}"))?;

    // Debug: Print first 200 characters of response for troubleshooting
    if response.len() > 200 {
        println!("API Response (first 200 chars): {}", &response[..200]);
    } else {
        println!("API Response: {response}");
    }

    // More robust JSON parsing for tag_name
    let tag_patterns = ["\"tag_name\":\"", "\"tag_name\": \"", "\"tag_name\":"];

    for pattern in &tag_patterns {
        if let Some(tag_start) = response.find(pattern) {
            let after_pattern = &response[tag_start + pattern.len()..];

            // Find the end of the tag value
            if let Some(tag_end) = after_pattern.find('"') {
                let tag = &after_pattern[..tag_end];
                if !tag.is_empty() {
                    return Ok(tag.to_string());
                }
            } else if let Some(tag_end) = after_pattern.find(',') {
                let tag = after_pattern[..tag_end].trim_matches('"').trim_matches(' ');
                if !tag.is_empty() {
                    return Ok(tag.to_string());
                }
            }
        }
    }

    Err(format!(
        "Failed to parse version from API response. Response: {}",
        &response[..response.len().min(500)]
    ))
}

// Download and install the update
fn download_and_install_update(version: &str, platform: &str) -> Result<(), String> {
    use std::process::Command;

    let download_url = format!(
        "https://github.com/ojutalayomi/pidgin/releases/download/{version}/pidgin-{platform}.zip"
    );

    let temp_dir = std::env::temp_dir().join("pidgin-update");
    let zip_path = temp_dir.join("pidgin.zip");

    // Create temp directory
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {e}"))?;

    // Download the release
    println!("Downloading from: {download_url}");
    let status = Command::new("curl")
        .args(["-L", "-o", zip_path.to_str().unwrap(), &download_url])
        .status()
        .map_err(|e| format!("Failed to execute curl: {e}"))?;

    if !status.success() {
        return Err("Failed to download update".to_string());
    }

    // Extract the zip file
    println!("Extracting update...");
    let status = Command::new("unzip")
        .args([
            "-q",
            "-o",
            zip_path.to_str().unwrap(),
            "-d",
            temp_dir.to_str().unwrap(),
        ])
        .status()
        .map_err(|e| format!("Failed to execute unzip: {e}"))?;

    if !status.success() {
        return Err("Failed to extract update".to_string());
    }

    // Find the executable in the extracted directory
    let executable_name = if cfg!(target_os = "windows") {
        "pidgin.exe"
    } else {
        "pidgin"
    };

    let extracted_dir = temp_dir.join("pidgin-".to_string() + platform);
    let new_executable = extracted_dir.join(executable_name);

    if !new_executable.exists() {
        return Err("Executable not found in downloaded release".to_string());
    }

    // Get current executable path
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {e}"))?;

    // Create backup
    let backup_path = current_exe.with_extension("backup");
    std::fs::copy(&current_exe, &backup_path)
        .map_err(|e| format!("Failed to create backup: {e}"))?;

    // Replace current executable
    std::fs::copy(&new_executable, &current_exe)
        .map_err(|e| format!("Failed to replace executable: {e}"))?;

    // Make executable (on Unix systems)
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&current_exe)
            .map_err(|e| format!("Failed to get file metadata: {e}"))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&current_exe, perms)
            .map_err(|e| format!("Failed to set executable permissions: {e}"))?;
    }

    // Clean up temp directory
    let _ = std::fs::remove_dir_all(&temp_dir);

    Ok(())
}
