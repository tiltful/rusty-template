// Initialize git repository
system::exec("git", ["init"]);

// Format Rust code
if system::which("cargo").is_some() {
    system::exec("cargo", ["fmt"]);
}

// Add initial commit
system::exec("git", ["add", "."]);
system::exec("git", ["commit", "-m", "🎉 feat: initial commit"]);

print("🎉 Project generated successfully!");
print("📁 Generated files have been formatted and committed to git");
