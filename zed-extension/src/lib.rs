use zed_extension_api::{self as zed, LanguageServerId, Result};

struct SlateExtension;

impl zed::Extension for SlateExtension {
    fn new() -> Self {
        SlateExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("slate-lsp")
            .ok_or("slate-lsp not found in PATH. Install it with: luma build --name slate-lsp")?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(SlateExtension);
