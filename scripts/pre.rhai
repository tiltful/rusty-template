let crate_type = variable::get("crate_type");
let license = variable::get("license");
let project_name = variable::get("project-name");
let github_user = variable::get("github_username");

if crate_type == "bin" {
    file::delete("src/lib.rs");
    file::delete("benches");
} else {
    file::delete("src/main.rs");
    file::delete("Dockerfile");
    file::delete(".dockerignore");
    file::delete(".github/workflows/release.yml");
}

file::delete("LICENSE");
if license == "MIT" {
    file::delete("NOTICE");
    file::delete("LICENSE-APACHE");
    file::rename("LICENSE-MIT", "LICENSE");
} else if license == "Apache-2.0" {
    file::delete("LICENSE-MIT");
    file::rename("LICENSE-APACHE", "LICENSE");
}

// cargo-generate excludes these files from Liquid processing; resolve their
// repository markers explicitly after the placeholders are known.
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

// Keep scripts until the post hook has completed.
