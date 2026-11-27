print("POST HOOK: finalizing generated project");
let crate_type = variable::get("crate_type");
let license = variable::get("license");
let project_name = variable::get("project-name");
let github_user = variable::get("github_username");

if crate_type == "bin" {
    if file::exists("src/lib.rs") { file::delete("src/lib.rs"); }
    if file::exists("benches") { file::delete("benches"); }
} else {
    if file::exists("src/main.rs") { file::delete("src/main.rs"); }
    if file::exists("Dockerfile") { file::delete("Dockerfile"); }
    if file::exists(".dockerignore") { file::delete(".dockerignore"); }
    if file::exists(".github/workflows/release.yml") { file::delete(".github/workflows/release.yml"); }
}

if file::exists("LICENSE") { file::delete("LICENSE"); }
if license == "MIT" {
    if file::exists("NOTICE") { file::delete("NOTICE"); }
    if file::exists("LICENSE-APACHE") { file::delete("LICENSE-APACHE"); }
    if file::exists("LICENSE-MIT") { file::rename("LICENSE-MIT", "LICENSE"); }
} else if license == "Apache-2.0" {
    if file::exists("LICENSE-MIT") { file::delete("LICENSE-MIT"); }
    if file::exists("LICENSE-APACHE") { file::rename("LICENSE-APACHE", "LICENSE"); }
}

if file::exists(".github/workflows/release.yml") {
    let content = file::read(".github/workflows/release.yml")
        .replace("__PROJECT_NAME__", project_name);
    file::write(".github/workflows/release.yml", content);
}
if file::exists("cliff.toml") {
    let content = file::read("cliff.toml")
        .replace("<GITHUB_USER>", github_user)
        .replace("<PROJECT_NAME>", project_name);
    file::write("cliff.toml", content);
}

print("Next steps:");
print("  1. Review Cargo.toml and selected features.");
print("  2. Run: cargo check");
print("  3. Run: cargo make quality");
if file::exists("Dockerfile") {
    print("  4. Build the container: docker build -t app .");
}

if file::exists("cargo-generate.toml") { file::delete("cargo-generate.toml"); }
if file::exists(".github/workflows/template-ci.yml") { file::delete(".github/workflows/template-ci.yml"); }
if file::exists("tests/test_commit_message.py") { file::delete("tests/test_commit_message.py"); }
if file::exists("tests") { file::delete("tests"); }
if file::exists("scripts/init.rhai") { file::delete("scripts/init.rhai"); }
if file::exists("scripts/pre.rhai") { file::delete("scripts/pre.rhai"); }
if file::exists("scripts/post.rhai") { file::delete("scripts/post.rhai"); }
if file::exists("scripts") { file::delete("scripts"); }

print("🎉 Project generated successfully!");
