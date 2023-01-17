//! Collection of convention validators.

use anyhow::{Context, Result};
use lenient_semver;
use pep440;
use semver;

/// Version validator.

pub enum VersionValidator {
    /// Semantic Versioning validator:
    /// <https://semver.org>
    Semver,
    /// Lenient Semantic Version validator:
    /// <https://docs.rs/lenient_semver/latest/lenient_semver/>
    LenientSemver,
    /// PEP 440 validator:
    /// <https://peps.python.org/pep-0440/>
    Pep440,
    /// Validator that accepts all values.
    AcceptAll,
}

impl VersionValidator {
    /// Validate a version string.
    pub fn validate(&self, version: &str) -> Result<()> {
        match self {
            Self::Semver => semver::Version::parse(version)
                .map(|_| Ok(()))
                .context(format!("Invalid semver version string: {version}"))?,
            Self::LenientSemver => lenient_semver::parse(version)
                .map(|_| Ok(()))
                .map_err(|e| e.owned())
                .context(format!("Invalid lenient semver version string: {version}"))?,
            Self::Pep440 => pep440::Version::is_canonical(version)
                .then(|| Ok(()))
                .context(format!("Invalid pep440 version string: {version}"))?,
            Self::AcceptAll => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1.2.3+semver")]
    fn test_semver_convention_valid(#[case] version: &str) {
        let convention = VersionValidator::Semver;
        assert!(convention.validate(version).is_ok());
    }

    #[rstest]
    #[case("invalid")]
    #[case("1.2+lenientsemver")]
    #[case("1.2.3.dev0")]
    fn test_semver_convention_invalid(#[case] version: &str) {
        let convention = VersionValidator::Semver;
        assert!(convention.validate(version).is_err());
    }

    #[rstest]
    #[case("1.2.3+semver")]
    #[case("1.2+lenientsemver")]
    #[case("1.2.3.dev0")]
    fn test_lenientsemver_convention_valid(#[case] version: &str) {
        let convention = VersionValidator::LenientSemver;
        assert!(convention.validate(version).is_ok());
    }

    #[rstest]
    #[case("invalid")]
    fn test_lenientsemver_convention_invalid(#[case] version: &str) {
        let convention = VersionValidator::LenientSemver;
        assert!(convention.validate(version).is_err());
    }

    #[rstest]
    #[case("1.2.3.dev0")]
    fn test_pep440_convention_valid(#[case] version: &str) {
        let convention = VersionValidator::Pep440;
        assert!(convention.validate(version).is_ok());
    }

    #[rstest]
    #[case("invalid")]
    #[case("1.2.3+semver")]
    #[case("1.2+lenientsemver")]
    fn test_pep440_convention_invalid(#[case] version: &str) {
        let convention = VersionValidator::Pep440;
        assert!(convention.validate(version).is_err());
    }

    #[rstest]
    #[case("invalid")]
    #[case("1.2.3+semver")]
    #[case("1.2+lenientsemver")]
    #[case("1.2.3.dev0")]
    fn test_accept_all_convention_valid(#[case] version: &str) {
        let convention = VersionValidator::AcceptAll;
        assert!(convention.validate(version).is_ok());
    }
}
