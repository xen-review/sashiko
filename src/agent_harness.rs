use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionFile {
    pub path: PathBuf,
    pub contents: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionSnapshot {
    pub files: Vec<InstructionFile>,
    pub fingerprint: String,
}

impl InstructionSnapshot {
    pub fn load(settings_path: &Path, manifest: &[PathBuf], max_bytes: usize) -> Result<Self> {
        if manifest.is_empty() || max_bytes == 0 {
            bail!("instruction manifest and byte budget must be non-empty");
        }
        let base = settings_path.parent().unwrap_or_else(|| Path::new("."));
        let mut files = Vec::with_capacity(manifest.len());
        let mut remaining = max_bytes;
        for path in manifest {
            let path = base
                .join(path)
                .canonicalize()
                .with_context(|| format!("cannot resolve instruction file {}", path.display()))?;
            if !path.is_file() {
                bail!("instruction path {} is not a regular file", path.display());
            }
            let file = std::fs::File::open(&path)
                .with_context(|| format!("cannot open instruction file {}", path.display()))?;
            let mut contents = String::new();
            file.take((remaining as u64).saturating_add(1))
                .read_to_string(&mut contents)
                .with_context(|| {
                    format!("cannot read UTF-8 instruction file {}", path.display())
                })?;
            if contents.len() > remaining {
                bail!("instruction manifest exceeds its {max_bytes}-byte budget");
            }
            remaining -= contents.len();
            let body = instruction_body(&contents)
                .with_context(|| format!("invalid instructions in {}", path.display()))?;
            files.push(InstructionFile {
                path,
                body,
                contents,
            });
        }
        let identity = serde_json::to_vec(&("agent-instructions-v1", &files))?;
        let fingerprint = Sha256::digest(identity)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        Ok(Self { files, fingerprint })
    }
}

fn instruction_body(contents: &str) -> Result<String> {
    let normalized = contents.replace("\r\n", "\n");
    let body = if let Some(after) = normalized.strip_prefix("---\n") {
        let (metadata, body) = after
            .split_once("\n---\n")
            .context("instruction frontmatter has no closing delimiter")?;
        let metadata: serde_json::Value = config::Config::builder()
            .add_source(config::File::from_str(metadata, config::FileFormat::Yaml))
            .build()?
            .try_deserialize()?;
        if !metadata.is_object() {
            bail!("instruction frontmatter must be a mapping");
        }
        body
    } else {
        &normalized
    };
    if body.trim().is_empty() {
        bail!("instruction body must not be empty");
    }
    Ok(body.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_resolves_manifest_and_preserves_order_and_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let agent = directory.path().join("agent.md");
        let skill = directory.path().join("skill.md");
        let original = "---\r\nname: Reviewer\r\ntools: [execute]\r\n---\r\nReview source.\r\n";
        std::fs::write(&agent, original).unwrap();
        std::fs::write(&skill, "Check callers.").unwrap();
        let settings = directory.path().join("Settings.toml");
        let manifest = vec![PathBuf::from("agent.md"), skill.clone()];
        let snapshot = InstructionSnapshot::load(&settings, &manifest, 1000).unwrap();
        assert_eq!(snapshot.files[0].path, agent.canonicalize().unwrap());
        assert_eq!(snapshot.files[0].contents, original);
        assert_eq!(snapshot.files[0].body, "Review source.\n");
        assert!(!snapshot.files[0].body.contains("execute"));
        let again = InstructionSnapshot::load(&settings, &manifest, 1000).unwrap();
        assert_eq!(snapshot.fingerprint, again.fingerprint);
        let reversed = vec![skill, PathBuf::from("agent.md")];
        assert_ne!(
            snapshot.fingerprint,
            InstructionSnapshot::load(&settings, &reversed, 1000)
                .unwrap()
                .fingerprint
        );
        std::fs::write(&agent, "Changed instructions.").unwrap();
        assert_eq!(snapshot.files[0].contents, original);
        assert_ne!(
            snapshot.fingerprint,
            InstructionSnapshot::load(&settings, &manifest, 1000)
                .unwrap()
                .fingerprint
        );
    }

    #[test]
    fn snapshot_rejects_missing_empty_and_oversized_inputs() {
        let directory = tempfile::tempdir().unwrap();
        let settings = directory.path().join("Settings.toml");
        let manifest = vec![PathBuf::from("agent.md")];
        assert!(InstructionSnapshot::load(&settings, &manifest, 100).is_err());
        std::fs::write(directory.path().join("agent.md"), "Review source.").unwrap();
        assert!(InstructionSnapshot::load(&settings, &manifest, 3).is_err());
        assert!(InstructionSnapshot::load(&settings, &[], 100).is_err());
        assert!(InstructionSnapshot::load(&settings, &manifest, 0).is_err());
        assert!(
            InstructionSnapshot::load(&settings, &[directory.path().to_path_buf()], 100).is_err()
        );
    }

    #[test]
    fn frontmatter_is_validated_without_enabling_tools() {
        for invalid in [
            "",
            " \n",
            "---\nname: [\n---\nReview.",
            "---\nname: Reviewer\nReview.",
            "---\nname: Reviewer\n---\n",
        ] {
            assert!(instruction_body(invalid).is_err(), "{invalid}");
        }
        assert_eq!(
            instruction_body("Plain instructions.").unwrap(),
            "Plain instructions."
        );
    }
}
