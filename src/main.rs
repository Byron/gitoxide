#![forbid(unsafe_code)]

#[cfg(feature = "pretty-cli")]
mod pretty {
    use anyhow::Result;
    use gitoxide_core as core;
    use std::io::{stderr, stdout};
    use structopt::StructOpt;

    mod options {
        use std::path::PathBuf;
        use structopt::clap::AppSettings;
        use structopt::StructOpt;

        #[derive(Debug, StructOpt)]
        #[structopt(about = "The git, simply swift")]
        #[structopt(settings = &[AppSettings::SubcommandRequired,
                            AppSettings::ColoredHelp])]
        pub struct Args {
            #[structopt(subcommand)]
            pub cmd: Subcommands,
        }

        /// Low-level commands that are not used every day
        #[derive(Debug, StructOpt)]
        pub enum Plumbing {
            /// Verify the integrity of a pack or index file
            #[structopt(setting = AppSettings::ColoredHelp)]
            VerifyPack {
                /// The '.pack' or '.idx' file whose checksum to validate.
                #[structopt(parse(from_os_str))]
                path: PathBuf,
            },
        }

        #[derive(Debug, StructOpt)]
        pub enum Subcommands {
            /// Initialize the repository in the current directory.
            #[structopt(alias = "initialize")]
            #[structopt(setting = AppSettings::ColoredHelp)]
            Init,

            #[structopt(alias = "p")]
            #[structopt(setting = AppSettings::ColoredHelp)]
            Plumbing(Plumbing),
        }
    }

    pub fn main() -> Result<()> {
        use options::*;
        let args = Args::from_args();
        match args.cmd {
            Subcommands::Init => core::init(),
            Subcommands::Plumbing(cmd) => match cmd {
                Plumbing::VerifyPack { path } => {
                    core::verify_pack_or_pack_index(path, stdout(), stderr())
                }
            },
        }?;
        Ok(())
    }
}

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
mod lean {
    use argh::FromArgs;

    #[derive(FromArgs)]
    /// A simple calculation tool
    struct Args {
        #[argh(subcommand)]
        subcommand: SubCommands,
    }

    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand)]
    enum SubCommands {
        Init(Init),
        VerifyPack(VerifyPack),
    }

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "init")]
    struct Init {}

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "verify-pack")]
    struct VerifyPack {
        /// the '.pack' or '.idx' file whose checksum to validate.
        #[argh(option)]
        path: PathBuf,
    }

    use anyhow::Result;
    use gitoxide_core as core;
    use std::{
        io::{stderr, stdout},
        path::PathBuf,
    };

    pub fn main() -> Result<()> {
        let cli: Args = argh::from_env();
        match cli.subcommand {
            SubCommands::Init(_) => core::init(),
            SubCommands::VerifyPack(VerifyPack { path }) => {
                core::verify_pack_or_pack_index(path, stdout(), stderr())
            }
        }
    }
}

use anyhow::Result;

#[cfg(feature = "pretty-cli")]
fn main() -> Result<()> {
    pretty::main()
}

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
fn main() -> Result<()> {
    lean::main()
}
