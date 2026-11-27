print("PRE HOOK: starting template cleanup");
let crate_type = variable::get("crate_type");
let license = variable::get("license");
let project_name = variable::get("project-name");
let github_user = variable::get("github_username");
print("PRE HOOK: crate_type=" + crate_type + ", license=" + license);

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
print("PRE HOOK: finished");
