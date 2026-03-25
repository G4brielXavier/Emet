use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {


    /// Init the `Emet::up`
    Up {
        /// Your `private_key`
        private_key: Option<String>,
        
        /// To show the `private_key` defined
        #[arg(short, long)]
        show: bool
    },

    /// Seal the `path` provided with `Emet::up().seal()` and save the file signed in a `.emet` file
    Seal {
        path: Option<PathBuf>
    },


    /// Check if `original file` and `.emet` file signed is authentic
    Check {
        path: Option<PathBuf>,
        emet_path: Option<PathBuf>
    }

}