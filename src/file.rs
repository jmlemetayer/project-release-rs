use anyhow::Result;
use std::path::{Path, PathBuf};

pub enum VersionFile {
    Plain(PathBuf),
    Formatted(PathBuf),
    Edited(PathBuf),
}

impl VersionFile {
    pub fn version(&self) -> Result<&str> {
        match self {
            Self::Plain(pathbuf) => unimplemented!(),
            Self::Formatted(pathbuf) => unimplemented!(),
            Self::Edited(pathbuf) => unimplemented!(),
        }
    }

    pub fn set_version(&self, version: &str) -> Result<()> {
        match self {
            Self::Plain(pathbuf) => unimplemented!(),
            Self::Formatted(pathbuf) => unimplemented!(),
            Self::Edited(pathbuf) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_plain_get_version() {
        let version_file = VersionFile::Plain(Path::new("version").to_path_buf());
    }
}
