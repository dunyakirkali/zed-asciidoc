use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result};

const SERVER_NAME: &str = "acdc-lsp";

struct AsciiDocExtension;

impl zed::Extension for AsciiDocExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(SERVER_NAME, worktree)?.binary;
        let command = binary
            .as_ref()
            .and_then(|binary| binary.path.clone())
            .or_else(|| worktree.which(SERVER_NAME))
            .ok_or_else(|| {
                "acdc-lsp was not found on PATH. Install it with `cargo install --path acdc-lsp` \
                 from the acdc checkout, or set `lsp.acdc-lsp.binary.path` in Zed settings."
                    .to_string()
            })?;

        let mut args = Vec::new();
        let mut env = worktree.shell_env();
        if let Some(binary) = binary {
            args = binary.arguments.unwrap_or_default();
            if let Some(overrides) = binary.env {
                env.retain(|(key, _)| !overrides.contains_key(key));
                env.extend(overrides);
            }
        }

        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(LspSettings::for_worktree(SERVER_NAME, worktree)?.initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(SERVER_NAME, worktree)?.settings;
        // Zed looks up the section requested by workspace/configuration in this object.
        Ok(Some(serde_json::json!({ "acdc-lsp": settings })))
    }
}

zed::register_extension!(AsciiDocExtension);
