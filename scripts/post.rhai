print("🎉 Project generated successfully!");
print("");
print("Next steps:");
print("  1. Review Cargo.toml and selected features.");
print("  2. Run: cargo check");
print("  3. Run: cargo make quality");
if file::exists("Dockerfile") {
    print("  4. Build the container: docker build -t app .");
}

if file::exists("cargo-generate.toml") {
    file::delete("cargo-generate.toml");
}
if file::exists(".github/workflows/template-ci.yml") {
    file::delete(".github/workflows/template-ci.yml");
}
if file::exists("tests/test_commit_message.py") {
    file::delete("tests/test_commit_message.py");
}
if file::exists("tests") {
    file::delete("tests");
}
if file::exists("scripts/init.rhai") {
    file::delete("scripts/init.rhai");
}
if file::exists("scripts/pre.rhai") {
    file::delete("scripts/pre.rhai");
}
if file::exists("scripts/post.rhai") {
    file::delete("scripts/post.rhai");
}
if file::exists("scripts") {
    file::delete("scripts");
}
