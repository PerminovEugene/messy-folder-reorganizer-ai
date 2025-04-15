use std::env;
use std::path::PathBuf;

/// Returns absolute path to home folder
pub fn get_home_path() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    fn returns_home_when_set() {
        let old_home = env::var("HOME").ok();
        env::set_var("HOME", "/tmp/fake_home");
        let path = get_home_path();
        assert_eq!(path, Path::new("/tmp/fake_home"));

        if let Some(val) = old_home {
            env::set_var("HOME", val);
        } else {
            env::remove_var("HOME");
        }
    }

    #[test]
    fn returns_dot_when_home_not_set() {
        let old_home = env::var("HOME").ok();
        env::remove_var("HOME");

        let path = get_home_path();
        assert_eq!(path, Path::new("."));

        if let Some(val) = old_home {
            env::set_var("HOME", val);
        }
    }
}
