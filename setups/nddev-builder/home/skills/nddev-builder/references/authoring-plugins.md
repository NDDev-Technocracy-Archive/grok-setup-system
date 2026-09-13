# Writing a plugin for this harness

Generated from the vendor's own reference and the pinned binary. Do not edit: the next render overwrites it, and a correction belongs in the source this file is derived from.

**Where it goes**: `~/.grok/plugins/<name>/plugin.json`

**Decided by**: measured by running the 1.0.5 binary against a temporary GROK_HOME

**How it runs**: Enabled plugins load from the plugins directory; a client can read the index to show the contents before installing.

## Manifest

| key | required | what it does |
|---|---|---|
| `name` | no | Defaults to the directory name. |
| `description` | no | What the plugin is for. |
| `skills` | no | Skills it contributes, as a `skills/` directory beside the manifest. |
| `commands` | no | Commands it contributes. |
| `agents` | no | Agents it contributes, addressed as `plugin-name:agent-name`. |
| `mcpServers` | no | MCP servers it attaches to the session. |
| `hooks` | no | Hooks it registers. |
| `lspServers` | no | Language servers it provides. |

## What bites

- **A plugin here is a directory with `plugin.json` in it**, bundling the same components you would otherwise write loose and running no code of its own. That is the opposite of the two harnesses that load a module, and the safer of the two designs to install unattended. Measured: a directory holding `plugin.json` and a `skills/` subdirectory is reported as *Plugins (1) nddev-probe (user, enabled) 1 skills*, and the same directory without the manifest is reported as none. The product says so in its own words when it refuses one: *"no plugins found in the source (no plugin.json or convention components)"*.
- **This toolkit named a different file until 2026-08-28**, `.grok-plugin/plugin-index.json`, which is what a third-party page says and what the product does not read. It was found by running the product, not by any check here -- which is the argument for the `exercised by` column in `surfaces`.
- **A plugin's components are not allowed everything a loose one is.** Plugin agent frontmatter cannot declare `mcpServers` or hooks, and cannot set `permissionMode: bypassPermissions`. A component that works from the agents directory can be refused once it ships inside a plugin, so bundling is a change of behaviour and not just of packaging.
- Plugin hooks receive `GROK_PLUGIN_ROOT` and `GROK_PLUGIN_DATA` in their environment, which loose hooks do not.

## The same file on the other harnesses

Generated from the same rows as the section above, for every harness in this estate that routes this kind. `—` means the product's own reference does not name the field, and **dropped** means it names it as one it accepts and does not act on.

| field | `claude` | `grok` | `cursor` | `antigravity` |
|---|---|---|---|---|
| `name` | **required** | yes | **required** | **required** |
| `displayName` | yes | — | — | — |
| `description` | yes | yes | yes | yes |
| `version` | yes | — | yes | — |
| `author` | yes | — | yes | — |
| `skills` | yes | yes | yes | — |
| `commands` | yes | yes | yes | — |
| `agents` | yes | yes | yes | — |
| `hooks` | yes | yes | yes | — |
| `mcpServers` | yes | yes | yes | — |
| `outputStyles` | yes | — | — | — |
| `lspServers` | yes | yes | — | — |
| `userConfig` | yes | — | — | — |
| `homepage` | yes | — | yes | — |
| `repository` | yes | — | yes | — |
| `license` | yes | — | yes | — |
| `keywords` | yes | — | yes | — |
| `metadata` | yes | — | — | — |
| `defaultEnabled` | yes | — | — | — |
| `dependencies` | yes | — | — | — |
| `logo` | — | — | yes | — |
| `rules` | — | — | yes | — |
| `variables` | — | — | yes | — |

**The part that travels**: `name`, `description`. Everything else is a bet on one product.

**The part that does not, and says nothing when it does not**: a field absent from a column is not rejected there -- it is read past. Nothing warns, no run fails, and the component behaves differently with the same bytes. Where the field was carrying a restriction, the restriction is simply gone. Check the column before relying on one.

## Before you ship one

- **The surface is declared, so the component is a promise.** Every kind   this provider declares is a promise of a rollback. A component written   to a path the declaration does not carry is installed by nobody and   removed by nobody.
- **Name it once.** Where the product derives identity from the directory   or the filename, the frontmatter `name` is either redundant or a second   place to be wrong. Keep them equal.
- **Read it back.** After an install, look at the file where the product   reads it, not at the step that put it there.
