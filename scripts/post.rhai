let cliff_path = "cliff.toml";

// Replace placeholders in cliff.toml
if (path_exists(cliff_path)) {
    let content = read_file(cliff_path);
    let replaced = content
        .replace("<GITHUB_USER>", project_variable("github_username"))
        .replace("<PROJECT_NAME>", project_name());
    write_file(cliff_path, replaced);
}


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
