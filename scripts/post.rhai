print("🎉 Project generated successfully!");

// Remove empty scripts directory
if file::exists("scripts") {
    let script_files = file::list_dir("scripts");
    if script_files.len() == 0 {
        file::delete("scripts");
        print("🗑️  Removed empty scripts directory");
    }
}

print("👉  You can manually run:");
print("git init && git add . && git commit -m '🎉 feat: initial commit'");
