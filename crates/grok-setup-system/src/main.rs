//! The Grok Build setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with every other setup system, so a change to
//! behaviour lands once and a change to Grok Build's surface lands here.
//!
//! The owner assigned this harness the program lifecycle as well, and it is
//! declared: `src/software.rs` carries the six artifacts xAI publishes, and
//! grok is the one product whose bytes *are* the program -- its direct
//! distribution needs no unpacking at all.

use std::process::ExitCode;

mod software;

#[cfg(test)]
mod project_tests;

use harness_runtime::{Harness, LaunchBinding, PreservationSurface, Scoped, Shadow};
use provider_v3::{ComponentKind, ProjectionKind, TargetScope};

/// Everything specific to Grok Build, verified against `grok-baseline.json`.
pub const GROK: Harness = Harness {
    // The consumer's closed harness enum spells this `grok-build`.
    harness_id: "grok-build",
    provider_id: "grok-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "Grok Build",
    vendor: "xAI",
    documented_config_home: "~/.grok",
    config_home_env: "GROK_HOME",
    // Measured 2026-08-28 by making the product write: `mcp add` reported
    // `File modified: $GROK_HOME/config.toml`.
    launch_binding: LaunchBinding::Complete {
        how: "measured by making the product write its own configuration into the target",
    },
    // Not measured. The two artifacts this estate has read for this question are
    // claude's, which carries `DISABLE_UPDATES`, and codex's, which carries no
    // such literal. This product has been asked nothing, and an empty value here
    // says the launch environment is untouched rather than that the product
    // leaves the bytes alone.
    // Measured 2026-08-31 in the pinned 1.0.13 artifact, from the product's own
    // embedded reference. It tabulates four ways to suppress an update check
    // and their scopes: `--no-auto-update` (session), `GROK_DISABLE_AUTOUPDATER=1`
    // (**process**), a non-TTY stderr (automatic), and `[cli] auto_update = false`
    // (persistent). The variable is the one a launch can set without writing
    // into a person's configuration, so it is the one set here.
    //
    // **A falsy value counts as not set** -- `0`, `false`, `off`, `no` or empty,
    // any case -- which is why `launch_environment` sends `1` rather than
    // anything shorter, and why inheriting somebody's `GROK_DISABLE_AUTOUPDATER=0`
    // would have left updates on. Launch overwrites it.
    //
    // The same reference notes that the background update is skipped anyway
    // unless the product runs from `$GROK_HOME/bin/grok`, which is not where
    // this provider installs. That covers one path and not the manual one, so
    // it is a reason to set the variable rather than a reason not to.
    updates_off_env: "GROK_DISABLE_AUTOUPDATER",
    // One home, one variable: nothing here is conditional.
    config_home_note: "",
    control_directory: ".grok-setup-system",
    state_file: "NDDEV-GROK-PROVIDER.json",
    predecessor_state_file: "NDDEV-GROK-BUILD-SETUP.json",
    profile_id: "grok/native-and-plugins/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    // Two entries here were not Grok's. `commands`, because Grok Build has no
    // commands directory -- a user-invocable skill *is* a slash command,
    // `/<skill-name>`, qualified `/local:<name>` on collision -- and
    // `.mcp.json`, which is a project-level compatibility file Grok merges
    // below `config.toml`, not a surface under this home.
    //
    // `workflows` is the one that was missing: `$GROK_HOME/workflows/<name>.rhai`
    // is documented and real, and no component kind projects there.
    native_namespaces: &[
        "AGENTS.md",
        "config.toml",
        // `managed_config.toml` and `requirements.toml` were here until
        // 2026-08-28 and are now in `never_touch`, because owning them deleted
        // them. Measured on the shipped 0.0.11 binary against a managed home:
        // `install` removed the administrator's signed policy and **kept its
        // signature sidecars**, which is exactly the state the product's own
        // gate refuses -- *"refusing session -- the signed is-managed claim
        // requires an authentic policy sidecar and none is present"*.
        //
        // The record's reason for owning them was that a backup returns them
        // byte-exact after an operation touches the home. That argument is
        // circular: the only reason an operation touched them was that they
        // were owned. A restore does bring them back, measured -- but the
        // person's next `grok` run happens before it, and is refused.
        "sandbox.toml",
        "skills",
        "agents",
        "plugins",
        "hooks",
        "workflows",
        // Added 2026-08-28, both from the product's own embedded reference
        // rather than from a page. `rules/` is listed as *Always scanned;
        // applies to all projects*; `commands/` sits beside the already-owned
        // `skills/` at User tier in the same row of the same table, so a
        // consumer could route a skill here and not a command.
        "rules",
        "commands",
        // Added 2026-08-28 from the product's own embedded reference in the
        // pinned 1.0.5 binary, which names both directories in one sentence:
        // *"Both are also discovered from `.grok/roles/*.toml` and
        // `.grok/personas/*.toml` files respectively."* The user tier is
        // spelled out beside it -- *"`~/.grok/personas/*.toml` (user)"*.
        //
        // Neither routes a kind. A persona is a behavioural overlay applied
        // during subagent resolution, not an agent a consumer can install, and
        // the closed kind set has no word for it -- so this is owned for the
        // same reason as `workflows` and `rules`: a backup captures it and a
        // restore returns it. Declaring a kind here would promise a rollback
        // for a component nobody can route.
        "personas",
        "roles",
    ],
    // The product's own: credentials, session history and runtime caches. Never
    // read, never written, and never copied into a backup slot.
    // Two spellings of the instruction file the product accepts and this
    // provider does not own, measured for this baseline. Grok Build reads
    // `AGENTS.md`, `Agents.md` and `AGENT.md`; this provider writes the
    // first only -- on a case-insensitive filesystem the second is the
    // same file, and on a case-sensitive one owning all three would let a
    // target hold two instruction documents that disagree while the
    // product reads one and this provider reports the other. Which wins
    // where several exist is not documented, so a target holding another
    // spelling is reported rather than resolved.
    shadowing_names: &[
        Shadow {
            name: "Agents.md",
            over: "AGENTS.md",
            effect: "an accepted spelling of the instruction file; which \
                     of the three the product reads when several exist \
                     is not documented, so its presence is reported, \
                     not resolved",
        },
        Shadow {
            name: "AGENT.md",
            over: "AGENTS.md",
            effect: "an accepted spelling of the instruction file; which \
                     of the three the product reads when several exist \
                     is not documented, so its presence is reported, \
                     not resolved",
        },
    ],
    // Owned, and nothing this build can install ever lands here: no
    // component kind routes to them and no setup in this catalogue
    // carries files there. So a posture selecting itself must not empty
    // them -- every posture agrees there is nothing, which makes the
    // emptiness a statement none of them made.
    custody_namespaces: &[
        "commands",
        "personas",
        "roles",
        "rules",
        "sandbox.toml",
        "workflows",
    ],
    preservation_surfaces: &[
        PreservationSurface {
            scope: None,
            roots: &[
                "Agents.md",
                "AGENT.md",
                "lsp.json",
                "pager.toml",
                "installed-plugins",
            ],
            excluded: &[
                "auth.json",
                "auth.json.lock",
                "sessions",
                "active_sessions.json",
                "active_sessions.lock",
                "logs",
                "memory-v2",
                "trusted_folders.toml",
                "trusted-plugins",
                "managed_config.toml",
                "requirements.toml",
                "managed_config.sig.json",
                "managed_identity.sig.json",
                "managed_config_cache.json",
            ],
        },
        PreservationSurface {
            scope: Some(TargetScope::UserRoot),
            roots: &["commands"],
            excluded: &[],
        },
    ],
    never_touch: &[
        // Credentials first, and it took a sweep across all seven to notice
        // this one was missing. Grok's own embedded reference names
        // `~/.grok/auth.json` as *Authentication credentials
        // (auto-managed)*, and five of the seven providers already listed
        // their equivalent. No live leak -- `capture` walks
        // `native_namespaces` and this file is inside none of them -- but a
        // safety list that depends on a namespace never widening is a safety
        // list waiting for one declaration change.
        "auth.json",
        // A person's marketplace sources, written by the product's own
        // `plugin marketplace` command, and it lives *inside* the owned
        // `plugins` namespace rather than beside it. Measured 2026-08-31 with
        // the released binary: a `select nddev-builder` then `select minimal`
        // took it, because replacement removes a namespace whole.
        //
        // Listing it here is what stops that now: `remove_keeping` spares a
        // `never_touch` path under a namespace being replaced, which is the
        // effect this list always claimed and only had for capture and
        // identity. `plugins` still routes the plugin kind, so it cannot be
        // dropped the way cursor's redundant parent was.
        "plugins/known_marketplaces.json",
        // The same product-owned registry must remain outside project writes.
        ".grok/plugins/known_marketplaces.json",
        // A person's installed plugins and the registry that records them.
        // `grok plugin install <dir> --trust` writes both, measured at 1.0.13
        // on 2026-08-31, and writes no `plugins/` at all -- the two are one
        // unit, since the same manifest under `installed-plugins/` without the
        // registry loads nothing. Same argument as `auth.json` above: outside
        // every owned namespace today, and listed so it stays outside one.
        "installed-plugins",
        "sessions",
        "active_sessions.json",
        "active_sessions.lock",
        "logs",
        // A person's accumulated cross-session context, GA shape since 1.0.34:
        // `global/`, `workspaces/`, `topics/`, `observations/`, sqlite indexes.
        // Capturing it into a slot or deleting it under a replaced namespace is
        // a privacy decision, not a configuration write -- the same argument as
        // `auth.json`, and the reason the `memory` decline row exists.
        "memory-v2",
        // The credential's lock file: deleting it mid-write corrupts the store
        // it guards.
        "auth.json.lock",
        // User-granted security state -- which folders and plugins are trusted.
        // Not this provider's to delete, capture, or hash into an identity,
        // for the same reason as the managed policy files below.
        "trusted_folders.toml",
        "trusted-plugins",
        // An administrator's policy and everything that proves it. Not this
        // provider's to delete, to capture into a slot, or to hash into an
        // identity -- an org's signed policy in a backup slot is the same
        // shape as a credential in one.
        //
        // All five together, because the harm was the *split*: owning the
        // policy and not its sidecars is what left a signature with nothing to
        // verify. `grok setup` writes these, server-fetched and
        // signature-checked; its own `--json` help says the flag exists so it
        // "writes nothing to ~/.grok".
        "managed_config.toml",
        "requirements.toml",
        "managed_config.sig.json",
        "managed_identity.sig.json",
        "managed_config_cache.json",
    ],
    // No near neighbour measured for this product. A marker listed here is a
    // refusal waiting to happen, so nothing is listed without evidence.
    foreign_homes: &[],
    permission_profiles: &["default"],
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Hook,
        ComponentKind::Plugin,
        ComponentKind::Setting,
        // `Command` was declared here on 2026-08-28 beside the `commands`
        // namespace and is **withdrawn the same week**, because running the
        // product answered the question reading it could not.
        //
        // Measured on the pinned 1.0.5 binary in a contained HOME: a file at
        // `~/.grok/commands/<name>.md` is loaded, and `grok inspect` lists it
        // under **Skills**, `user` tier -- beside one placed in `skills/`.
        // Two controls: a file under a directory nothing routes to is not
        // listed, and removing the file removes the entry. So the directory is
        // read, and what it holds becomes a skill.
        //
        // The product's own embedded reference says the same thing in a
        // precedence table -- `~/.grok/skills/`, `~/.grok/commands/` share one
        // row, *"Personal skills for all projects"*, and the project-scope rows
        // call the directory *"legacy command markdown"*.
        //
        // The namespace above stays owned: it is read, so backup, remove and
        // identity must cover it. What comes out is the promise that a
        // `command` component routed there stays a command -- it would arrive
        // as a skill, and `skill` already routes to `skills`. This is the third
        // case in this estate for a per-kind route the wire cannot yet express,
        // beside grok's own `rules` and claude's `workflows`.
    ],
    projection_kinds: &[
        ProjectionKind::NativeFiles,
        ProjectionKind::Marketplace,
        ProjectionKind::Plugin,
    ],
    // User-root is `~/.agents`, the one root in this estate
    // that belongs to a convention rather than to a product: a *sibling* of
    // this product's configuration home, not a child, so nothing declared
    // against the target above can reach it. That is what `user_root` is for.
    //
    // Its own embedded reference scans `.agents/skills/` at every tier and the tier table names User.
    //
    // **This was a declined row until now, and the reason it carried had
    // stopped being true.** It read *a namespace is removed whole, so a second
    // declaration would make either provider's remove take the other's
    // skills.* Correct when written; false since `written_paths` shipped.
    // Under a scope every verb acts on the files this provider recorded
    // writing -- the removal refuses rather than widening when it cannot read
    // the record, the capture takes ours and not a neighbour's, and a restore
    // leaves a neighbour's file as it was. Five of the seven products read
    // this root. This sentence used to say one declared it; all five do now --
    // the reason had simply not been re-read when the thing it described
    // changed, and the sentence kept its old answer.
    scoped_projections: &[
        Scoped {
            target_scope: TargetScope::UserRoot,
            // Distinct from the global identity, because the digest binds a
            // declaration together with the scope it owns.
            profile_id: "grok/native-files/user-root/1",
            component_kinds: &[ComponentKind::Skill],
            projection_kinds: &[ProjectionKind::NativeFiles],
            // Relative to `~/.agents`, which is the target this scope names -- so a
            // skill is `skills/<name>` rather than `.agents/skills/<name>`. Writing
            // the root into the path would put the skills at
            // `~/.agents/.agents/skills`.
            native_namespaces: &["skills"],
            // `~/.agents/commands/*.md` is discovered as a skill (grok 1.0.18
            // `inspect --json`) and is declined in the baseline so this kind is
            // not declared on two surfaces.
        },
        Scoped {
            target_scope: TargetScope::Project,
            profile_id: "grok/native-files/project/1",
            component_kinds: &[
                ComponentKind::Instruction,
                ComponentKind::Skill,
                ComponentKind::Agent,
                ComponentKind::Hook,
                ComponentKind::Plugin,
            ],
            projection_kinds: &[ProjectionKind::NativeFiles],
            // Project config has narrower semantics than user config. It is
            // not routed until those content restrictions are qualified.
            native_namespaces: &[
                "AGENTS.md",
                ".grok/skills",
                ".grok/agents",
                ".grok/hooks",
                ".grok/plugins",
            ],
        },
    ],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    // Generated by `build.rs` from this harness's `setups/` directory, so the
    // binary carries the catalog it is named after instead of hoping to find
    // one on a disk it was never shipped to.
    embedded_setups: include!(concat!(env!("OUT_DIR"), "/embedded_setups.rs")),
    software: Some(software::SOFTWARE),
    instruction_region: Some("AGENTS.md"),
};

fn main() -> ExitCode {
    harness_runtime::run(&GROK, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    /// The directory name this harness's setups live under in the workspace.
    const TOOL: &str = "grok";
    /// The declaration under test, named once so the shared test below reads
    /// the same in all seven crates.
    const HARNESS: Harness = GROK;

    /// `build.rs` put the whole catalog in, under the paths it will be read by.
    ///
    /// This does **not** test for staleness, and an earlier version of this
    /// comment claimed it did. It cannot: `build.rs` declares
    /// `rerun-if-changed` on the catalog directory, so editing a setup rebuilds
    /// the table before this runs, and the test would be comparing the tree
    /// with itself. Observed — a deliberately edited setup left it green.
    ///
    /// What it does test is the build script, against a walk written
    /// independently of it: every file present, none invented, bytes exact, and
    /// paths relative and slash-separated. That last one is the one that would
    /// really break — `join("/")` is the only reason these keys are usable on
    /// Windows, and a path built with the platform separator would still look
    /// perfectly correct in the generated source.
    /// The bytes this harness ships, pinned so they cannot change unseen.
    ///
    /// A setup's `definition_digest` is what makes two setups the same setup,
    /// and it appears in `list`, in a plan and in provider state -- and until
    /// this, nothing compared it to anything. A stray character in a setup file
    /// changed what the estate installs and every test stayed green.
    ///
    /// One aggregate rather than one per setup, because the claim is about the
    /// catalogue: sorted definition digests, joined by a newline, hashed. A
    /// deliberate change to a setup updates the line in the baseline, which is
    /// the point -- the peer calls this a golden and it earns itself the first
    /// time a row moves without anyone meaning it to.
    ///
    /// **And it is the three-OS check nothing else makes.** The setups are
    /// embedded with `include_bytes!`, so whatever the checkout holds is what
    /// ships; `.gitattributes` pins `eol=lf` to keep a Windows checkout from
    /// rewriting them, and this is the assertion that would notice if it ever
    /// stopped working. The matrix runs it on all three systems, so a digest
    /// that differed by platform could not stay hidden.
    #[test]
    fn the_catalogue_this_harness_ships_is_the_one_the_baseline_records() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let mut digests: Vec<String> = catalog
            .list()
            .unwrap()
            .iter()
            // **Both digests, because one of them holds nothing a person
            // reads.** `definition_digest` is the payload tree; the manifest --
            // `id`, `sources`, `description` -- was covered by no digest in this
            // estate, and those three are what a consumer renders on the surface
            // that precedes an install. A description was rewritten and the
            // whole gate stayed clean, which is how this was found.
            .map(|setup| format!("{}\n{}", setup.definition_digest, setup.manifest_digest))
            .collect();
        digests.sort();
        let joined = digests.join("\n");
        let aggregate = harness_runtime::digest_of_bytes(&joined);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let recorded = baseline["setup_catalogue_digest"].as_str().unwrap_or("");
        assert_eq!(
            aggregate, recorded,
            "the setups this binary ships are not the ones {TOOL}-baseline.json \
             records; if the change was meant, put this digest there"
        );
    }

    #[test]
    fn the_catalog_this_binary_carries_is_the_one_in_the_tree() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // The workspace holds one directory per harness; a rendered public tree
        // ships one harness and holds it flat. Same two candidates `build.rs`
        // chooses between, asked the same way.
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };

        // Only the setup directories, which is what the reader lists and what
        // `build.rs` embeds. A rendered public tree also carries a
        // `setups/README.md` at the catalog root, which belongs to no setup.
        let mut on_disk = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.join("setup.json").is_file())
            .collect();
        while let Some(directory) = stack.pop() {
            for entry in std::fs::read_dir(&directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    on_disk.push(path);
                }
            }
        }

        assert_eq!(
            HARNESS.embedded_setups.len(),
            on_disk.len(),
            "the binary carries {} files and the tree holds {}",
            HARNESS.embedded_setups.len(),
            on_disk.len()
        );

        for (relative, bytes) in HARNESS.embedded_setups {
            assert!(
                !relative.contains('\\') && !relative.starts_with('/'),
                "{relative:?} is not a relative slash path; a key built with the \
                 platform separator reads correctly on Unix and finds nothing on Windows"
            );
            let path = root.join(relative);
            let found = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("{relative} is compiled in but not in the tree: {e}"));
            assert_eq!(
                &found, bytes,
                "{relative} differs between the binary and the tree"
            );
        }
    }

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = GROK.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.harness_id, "grok-build");
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in GROK.never_touch {
            assert!(
                !GROK.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }

    /// Everything this harness claims to own, against the vendor page that
    /// decided it.
    ///
    /// What this replaced only checked that the baseline parsed. The block it
    /// reads now is hand-authored beside the rest of the baseline, and this is
    /// what keeps that block from being decoration: a namespace no vendor
    /// document names, or a declared kind no owned surface routes, is red here.
    ///
    /// Both directions, because the defect it was written for ran both ways --
    /// `~/.cursor/rules` was owned and does not exist, `~/.pi/agent/prompts`
    /// exists and was not owned. Conformance caught neither: its
    /// `declared_native_route_is_compilable` case asks for **one** route, not
    /// every one.
    #[test]
    fn every_surface_this_harness_owns_is_one_the_vendor_documents() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let problems = harness_runtime::surfaces::disagreements(&HARNESS, &baseline);
        assert!(
            problems.is_empty(),
            "the declaration and {TOOL}-baseline.json disagree:
  {}",
            problems.join(
                "
  "
            )
        );
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(GROK.control_directory.contains("setup-system"));
        assert!(GROK.state_file.starts_with("NDDEV-"));
        assert!(!GROK.native_namespaces.contains(&GROK.state_file));
    }
    /// A setup that writes a configuration file says where its format came from.
    ///
    /// The release before this one made the *surfaces* sourced: a path this
    /// provider owns cites the page that documents it. This is the same rule
    /// one level down, and it was written because two of the seven failed it.
    ///
    /// opencode's baseline set `"permission": "ask"` where the product
    /// documents an object of tool names, and antigravity's set
    /// `toolPermissions` where the product reads `toolPermission` with four
    /// values, none of them the one written. Both were valid JSON in the right
    /// file at the right path. Both installed, verified and restored cleanly.
    /// Neither changed anything about the product — a target that looks
    /// configured and is not, which is the failure this estate refuses one
    /// level up and had been shipping one level down.
    /// Two files in one setup that a case-insensitive filesystem would merge.
    ///
    /// macOS and Windows fold case, so such a pair is one file there and two on
    /// Linux -- the setup would install different content depending on the
    /// machine, and its catalogue digest would differ per platform. The bundle
    /// reader has refused this for an arriving bundle since 0.0.11; this is the
    /// same rule applied to what this repository authors.
    /// Every component entry point describes itself.
    ///
    /// A `SKILL.md` or an agent whose frontmatter lost its `description` still
    /// installs, verifies and restores cleanly -- and the product names it after
    /// its directory and gives the model nothing to choose on. Documents under
    /// `references/` and files under `commands/` are exempt, because the
    /// products measured do not read frontmatter from either.
    /// Supporting documents are reachable from an entry point.
    ///
    /// A `references/` folder whose skill has no `SKILL.md` is prose nothing
    /// routes to. A generator in this repository produced exactly that, and
    /// every other guard passed it: the files are documents, so `unsourced`
    /// exempts them, and there is no `SKILL.md`, so `undescribed` has nothing
    /// to check.
    /// Nothing shipped sends a reader to a file this setup does not carry.
    ///
    /// A routing table naming `references/surfaces.md` in a setup that ships no
    /// such file sends the reader nowhere -- and the reader is a model, which
    /// will not say so. The generator here did exactly that: it pointed every
    /// harness's agent at that path, and codex ships no skill at all.
    #[test]
    fn nothing_shipped_names_a_document_it_does_not_carry() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::dangling_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn every_reference_folder_has_an_entry_point() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unreachable_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    /// Nothing inside a skill is a file no reader is sent to.
    ///
    /// Two findings in one hour were of exactly this shape and every guard in
    /// this estate was silent on both: an executable validator shipped into
    /// people's homes that nothing named, and eleven authoring pages written
    /// into four harnesses and routed to from none. The estate asked whether a
    /// *named* file exists and never whether an *existing* file is named.
    #[test]
    fn nothing_inside_a_skill_is_stranded() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let found = harness_runtime::catalog::stranded(
            &harness_runtime::Catalog::at(&root).list().unwrap(),
        );
        assert!(found.problems.is_empty(), "{}", found.problems.join("\n  "));
        // grok carries 12 file(s) inside its skill. Stated so that a layout change emptying the skill fails here rather than passing a guard with nothing left to walk.
        assert_eq!(
            found.entry_points, 12,
            "the stranded-file guard walked {} files inside skills, not 12",
            found.entry_points
        );
    }

    #[test]
    fn every_component_entry_point_describes_itself() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let examined = harness_runtime::catalog::undescribed(&catalog.list().unwrap());
        assert!(
            examined.problems.is_empty(),
            "{}",
            examined.problems.join("\n  ")
        );
        // grok ships 2 entry point(s) across its four postures. Stated so that a layout change removing them fails here rather than passing a guard with nothing left to check.
        assert_eq!(
            examined.entry_points, 2,
            "the description guard examined {} entry points, not 2",
            examined.entry_points
        );
    }

    #[test]
    fn no_two_files_in_a_setup_differ_only_in_case() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::colliding(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn a_setup_that_writes_configuration_says_where_its_format_came_from() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unsourced(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Three postures, on every one of the seven.
    ///
    /// `baseline` is a working floor, `minimal` is the shared autonomous posture and nothing else,
    /// and `full-auto` asks nothing and sandboxes nothing. A caller who learns
    /// them on one product knows them on all seven, which is the whole reason
    /// the names are the estate's rather than each harness's.
    ///
    /// The second half of the check is the one worth having: two setups with
    /// the same bytes mean one of them is a posture in name only, and it would
    /// still read as offered in `list`.
    #[test]
    fn the_three_postures_are_offered_and_are_actually_different() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::asymmetric(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Nothing this setup ships tells a reader to run something that is not here.
    ///
    /// A setup carries documents an agent reads and acts on -- a skill, a rule,
    /// a command file -- and nothing was checking them. One shipped
    /// `software-status --target <dir> --json` and `list --json` for six
    /// releases; the binary refuses both, and says so in those words.
    ///
    /// Two refusals: a name belonging to the frozen estate, and any line naming
    /// this provider followed by a verb `into_command` does not accept. English
    /// is not judged -- `install` in a sentence is a word, and only
    /// `<provider> install` is an instruction.
    #[test]
    fn nothing_this_harness_ships_names_a_command_it_refuses() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems =
            harness_runtime::catalog::misdirecting(HARNESS.provider_id, &catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    /// The instruction-file spellings the baseline measured are declared, so
    /// `status` can report a target running a file this provider never wrote
    /// instead of answering `managed` beside nothing.
    #[test]
    fn the_measured_instruction_shadows_are_declared() {
        let names: Vec<&str> = GROK
            .shadowing_names
            .iter()
            .map(|shadow| shadow.name)
            .collect();
        for expected in &["Agents.md", "AGENT.md"] {
            assert!(
                names.contains(expected),
                "{expected} is measured in the baseline and not declared"
            );
        }
    }
}
