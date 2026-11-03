use std::fs;
use std::path::Path;

fn check_dir_for_auth(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                check_dir_for_auth(&path);
            } else if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                let lower = file_name.to_lowercase();
                assert!(
                    !lower.contains("login") && !lower.contains("signup") && !lower.contains("register"),
                    "Framework crate must NEVER contain auth pages: {:?}",
                    path
                );
            }
        }
    }
}

#[test]
fn test_no_auth_in_framework_src() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    check_dir_for_auth(&src_dir);
}
