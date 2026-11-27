print("🎉 Project generated successfully!");
print("");
print("Next steps:");
print("  1. Review Cargo.toml and selected features.");
print("  2. Run: cargo check");
print("  3. Run: cargo make quality");
if file::exists("Dockerfile") {
    print("  4. Build the container: docker build -t app .");
}
file::delete("scripts/init.rhai");
file::delete("scripts/pre.rhai");
file::delete("scripts/post.rhai");
file::delete("scripts");
