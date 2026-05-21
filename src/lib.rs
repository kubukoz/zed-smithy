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
        let cs = worktree
            .which("cs")
            .ok_or_else(|| "Coursier (`cs`) must be installed and on PATH to launch the Smithy Language Server. See https://get-coursier.io/.".to_string())?;

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
