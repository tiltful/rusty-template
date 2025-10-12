// Check if cargo is available
if system::which("cargo").is_none() {
    error("❌ Cargo is not installed or not in PATH");
}

// Check if git is available
if system::which("git").is_none() {
    error("❌ Git is not installed or not in PATH");
}

print("✅ All required tools are available");
