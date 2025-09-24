use std::process::Command;

fn main() {
    // Get git commit hash
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok();

    let git_hash = match output {
        Some(output) if output.status.success() => {
            String::from_utf8(output.stdout)
                .unwrap_or_else(|_| "unknown".to_string())
                .trim()
                .to_string()
        }
        _ => "unknown".to_string(),
    };

    // Check if working tree is dirty
    let dirty_output = Command::new("git")
        .args(&["diff-index", "--quiet", "HEAD"])
        .status()
        .ok();

    let is_dirty = match dirty_output {
        Some(status) => !status.success(),
        None => false,
    };

    let git_suffix = if is_dirty {
        format!("{}-dirty", git_hash)
    } else {
        git_hash
    };

    // Get the current tag if on a tag
    let tag_output = Command::new("git")
        .args(&["describe", "--exact-match", "--tags", "HEAD"])
        .output()
        .ok();

    let on_tag = match tag_output {
        Some(output) if output.status.success() => {
            let tag = String::from_utf8(output.stdout)
                .unwrap_or_else(|_| String::new())
                .trim()
                .to_string();
            !tag.is_empty()
        }
        _ => false,
    };

    // Only set GIT_VERSION_SUFFIX if not on a tag
    if on_tag {
        println!("cargo:rustc-env=GIT_VERSION_SUFFIX=");
    } else {
        println!("cargo:rustc-env=GIT_VERSION_SUFFIX= ({})", git_suffix);
    }

    // Rebuild if git state changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
}
