// Check if cargo is available
let cargo_check = system::command("cargo", ["--version"]);
if cargo_check.is_err() {
    error("❌ Cargo is not installed or not in PATH");
}

// Check if git is available
let git_check = system::command("git", ["--version"]);
if git_check.is_err() {
    error("❌ Git is not installed or not in PATH");
}

print("✅ All required tools are available");
