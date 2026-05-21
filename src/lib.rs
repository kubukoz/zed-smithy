use zed_extension_api::{self as zed, Result};

struct SmithyExtension;

impl zed::Extension for SmithyExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree
            .which("smithy-language-server")
            .or_else(|| worktree.which("smithy-ls"))
        {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: worktree.shell_env(),
            });
        }

        let cs = worktree
            .which("cs")
            .ok_or_else(|| "Smithy LSP not found on PATH (looked for `smithy-language-server` and `smithy-ls`). Install it directly, or install coursier (https://get-coursier.io/) so it can be launched via `cs launch --contrib smithy-language-server`.".to_string())?;

        Ok(zed::Command {
            command: cs,
            args: vec![
                "launch".to_string(),
                "--contrib".to_string(),
                "smithy-language-server".to_string(),
            ],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(SmithyExtension);
