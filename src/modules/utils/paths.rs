use directories::ProjectDirs;
use std::{io, path::PathBuf};

pub const QUALIFIER: &str = "org";
pub const ORGANIZATION: &str = "infinite";
pub const APPLICATION: &str = "zsh-infiniteinfinite";
pub const ZSH_THEME_FILE_NAME: &str = "infinite.zsh-theme";
pub const ZSH_RC_SNIPPET_FILE_NAME: &str = "infinite_zshrc_snippet"; // To be sourced by user's .zshrc

#[derive(Debug)]
pub struct InstallPaths {
    pub bin_dir: PathBuf,
    pub theme_file_path: PathBuf,
    pub zshrc_snippet_path: PathBuf,
}

pub fn get_install_paths() -> Result<InstallPaths, io::Error> {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        let bin_dir = proj_dirs.data_local_dir().join("bin");
        let theme_file_path = proj_dirs.config_dir().join(ZSH_THEME_FILE_NAME);
        let zshrc_snippet_path = proj_dirs.config_dir().join(ZSH_RC_SNIPPET_FILE_NAME);

        Ok(InstallPaths {
            bin_dir,
            theme_file_path,
            zshrc_snippet_path,
        })
    } else {
        Err(io::Error::other(
            "Could not determine project directories for installation.",
        ))
    }
}
