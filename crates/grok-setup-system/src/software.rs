//! Grok Build's own program, as measured rather than as described.
//!
//! Generated from the `software_artifacts` block of
//! `references/grok-baseline.json`. Every member path below was read out
//! of the archive it names, not assumed: codex's carries the target triple and
//! so genuinely differs per platform.
//!
//! Where a `previous_software_artifacts` block is present, it is transcribed
//! too. It is not a second choice: the outgoing current pin is stored there on
//! a bump, so the pair is always two consecutive real releases and there is
//! still exactly one value to keep fresh.
//!
//! Do not edit. The test at the bottom re-reads that baseline and compares it
//! field by field, so an edit here fails rather than silently installing bytes
//! nobody measured.

use harness_runtime::{Artifact, Delivery, Previous, Shape, Software};

/// The artifacts grok is published as.
pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://x.ai/cli/grok-1.0.31-linux-aarch64",
        bytes: 135_462_856,
        sha256: "sha256:42705891cfbe56076eb61b3c87592302729f72b017b713946e3b3a507a3146af",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://x.ai/cli/grok-1.0.31-linux-x86_64",
        bytes: 162_293_824,
        sha256: "sha256:d37fa3e50c5509a12af4bb4fc4f3c6bb2eadfc28f8f3e88dc21935b47e4e342f",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://x.ai/cli/grok-1.0.31-macos-aarch64",
        bytes: 142_384_128,
        sha256: "sha256:6444143255f8c5095d26b40ebd362bbedee76bddb98fa8f634efe871fde8ba23",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://x.ai/cli/grok-1.0.31-macos-x86_64",
        bytes: 159_439_936,
        sha256: "sha256:9c02e59cff998bb0d003a7c345cae3d25dae84dcbb818b647bf6b191dcca3664",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://x.ai/cli/grok-1.0.31-windows-aarch64.exe",
        bytes: 130_216_776,
        sha256: "sha256:7484fc86249d9e5207cac355139cf6c0d352acb3fc99a024e90440ebb66ccf10",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://x.ai/cli/grok-1.0.31-windows-x86_64.exe",
        bytes: 150_586_696,
        sha256: "sha256:73d2c31b90a21140fbe1c62d3c3efde541a3e254cf0f2571dc86addadb483eba",
        shape: Shape::Raw,
        member: "",
    },
];

/// The artifacts 1.0.24 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://x.ai/cli/grok-1.0.24-linux-aarch64",
        bytes: 130_988_392,
        sha256: "sha256:f83f70fa746cee2b954af2761bd10e2dc5fdea1c51355be75040abab486a5410",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://x.ai/cli/grok-1.0.24-linux-x86_64",
        bytes: 157_083_520,
        sha256: "sha256:a31a1c270246beb8e18f1fce91121a5e83d9ba8f51a8b24346bbc75fe727bdf5",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://x.ai/cli/grok-1.0.24-macos-aarch64",
        bytes: 137_364_416,
        sha256: "sha256:4291021c1570a7c8610277a3d65490a5e54b50311e222c6b4614264f02a215b3",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://x.ai/cli/grok-1.0.24-macos-x86_64",
        bytes: 154_067_200,
        sha256: "sha256:0c7a287a0b177f8a975487fd665c5213c130c06b3fe9367c9db8fdb16eb1008b",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://x.ai/cli/grok-1.0.24-windows-aarch64.exe",
        bytes: 124_818_760,
        sha256: "sha256:221cd18e6ad9ffb50eb488fa5b154736cc6fa3a206187f06ede4c89f2f885d32",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://x.ai/cli/grok-1.0.24-windows-x86_64.exe",
        bytes: 144_390_984,
        sha256: "sha256:4dc9038205649ec377ae37e09661e6083ee0a9776b2cc3019e7fc8dc74236ef5",
        shape: Shape::Raw,
        member: "",
    },
];

/// Grok Build's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.0.31",
    command: "grok",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.0.24",
        artifacts: PREVIOUS_ARTIFACTS,
    }),
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    // Named rather than glob-imported: a product delivered by a package manager
    // has no `Artifact` in scope, and the test is the same text for all seven.
    use harness_runtime::{Delivery, Shape};

    use super::SOFTWARE;

    fn measured() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/grok-baseline.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn every_artifact_compiled_in_is_the_one_the_baseline_measured() {
        let block = &measured()["software_artifacts"];
        assert_eq!(block["version"], SOFTWARE.version);
        assert_eq!(block["command"], SOFTWARE.command);

        let Delivery::Artifacts(compiled) = SOFTWARE.delivery else {
            // A product delivered by a package manager has no artifacts, and
            // the baseline must agree that it has none.
            assert_eq!(block["shape"], "manager");
            assert!(block["platforms"].as_object().unwrap().is_empty());
            return;
        };
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            compiled.len(),
            published.len(),
            "the table and the baseline disagree on how many platforms exist"
        );
        for artifact in compiled {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
            let member = entry.get("member").and_then(serde_json::Value::as_str);
            assert_eq!(
                member.unwrap_or(""),
                artifact.member,
                "{} names a different member",
                artifact.platform
            );
            assert_eq!(
                artifact.shape == Shape::Raw,
                member.is_none(),
                "{} disagrees about whether the bytes are the program",
                artifact.platform
            );
        }
    }

    /// The second pin is the baseline's, or it is absent in both places.
    ///
    /// Asserted from either side rather than only where it exists: a harness
    /// that has never been bumped must compile in `None`, and a build that
    /// dropped the block while the baseline still carried it would otherwise
    /// pass by having nothing to compare.
    #[test]
    fn the_version_this_build_can_move_between_is_the_one_measured_before_it() {
        let baseline = measured();
        let recorded = baseline.get("previous_software_artifacts");
        let Some(earlier) = SOFTWARE.previous else {
            assert!(
                recorded.is_none(),
                "the baseline records a previous release and this build names none"
            );
            return;
        };
        let block = recorded.unwrap_or_else(|| {
            panic!("this build names a previous release the baseline does not record")
        });
        assert_eq!(block["version"], earlier.version);
        assert_ne!(
            earlier.version, SOFTWARE.version,
            "a second pin equal to the first is one version wearing two names"
        );
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            earlier.artifacts.len(),
            published.len(),
            "the previous table and the baseline disagree on how many platforms exist"
        );
        for artifact in earlier.artifacts {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
        }
    }

    #[test]
    fn a_platform_the_vendor_does_not_publish_is_listed_rather_than_missing() {
        let block = &measured()["software_artifacts"];
        let unpublished: Vec<&str> = block
            .get("unpublished")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(unpublished, SOFTWARE.unsupported);
    }

    #[test]
    fn no_release_calls_a_platform_both_published_and_unpublished() {
        let baseline = measured();
        for name in ["software_artifacts", "previous_software_artifacts"] {
            let Some(block) = baseline.get(name) else {
                continue;
            };
            let published = block["platforms"].as_object().unwrap();
            let unpublished = block
                .get("unpublished")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str);
            for platform in unpublished {
                assert!(
                    !published.contains_key(platform),
                    "{name}: {platform} is both published and unpublished"
                );
            }
        }
    }
}
