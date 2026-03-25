use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {


    /// Init the `Emet::up` with the `private_key` provided (WARNING: Don't forget your `private_key`)
    Up {
        private_key: Option<String>,
        
        #[arg(short, long)]
        show: bool
    },

    /// Seal the `path` provided with `Emet::up().seal()`
    Seal {
        path: Option<PathBuf>
    },


    /// Check if `original file` and `.emet` encrypted file have your hashes matching
    Check {
        path: Option<PathBuf>,
        emet_path: Option<PathBuf>
    }

}