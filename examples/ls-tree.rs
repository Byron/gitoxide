use clap::Parser;
use gix::bstr::BString;
use std::io::{stdout, Write};

use gix::{object::Kind, objs::tree::EntryMode, objs::tree::EntryMode::Tree, traverse::tree::Recorder, ObjectId};

fn main() {
    let args = Args::parse_from(gix::env::args_os());
    match run(args) {
        Ok(()) => {}
        Err(e) => eprintln!("error: {e}"),
    }
}

#[derive(Debug, clap::Parser)]
#[clap(name = "ls-tree", about = "git ls-tree example", version = option_env!("GITOXIDE_VERSION"))]
#[clap(arg_required_else_help = true)]
struct Args {
    /// Recurse into subtrees
    #[clap(short = 'r')]
    recursive: bool,
    /// Only show trees
    #[clap(short = 'd')]
    tree_only: bool,
    /// Show trees when recursing
    #[clap(short = 't')]
    tree_recursing: bool,
    /// A revspec pointing to a tree-ish object, e.g. 'HEAD', 'HEAD:src/'
    #[clap(name = "tree-ish")]
    treeish: String,
}

fn run(args: Args) -> anyhow::Result<()> {
    let repo = gix::discover(".")?;
    let rev_spec = repo.rev_parse_single(&*args.treeish)?;
    let object = rev_spec.object()?;
    let tree = match object.kind {
        Kind::Commit => object.try_into_commit()?.tree()?,
        Kind::Tree => object.try_into_tree()?,
        _ => anyhow::bail!("not a tree-ish object"),
    };
    let entries = if args.recursive {
        let mut recorder = Recorder::default();
        tree.traverse().breadthfirst(&mut recorder)?;
        recorder
            .records
            .into_iter()
            .filter(|entry| args.tree_recursing || args.tree_only || entry.mode != Tree)
            .filter(|entry| !args.tree_only || (entry.mode == Tree))
            .map(|entry| Entry::new(entry.mode, entry.oid, entry.filepath))
            .collect::<Vec<_>>()
    } else {
        tree.iter()
            .filter_map(|res| res.ok().map(|entry| entry.inner)) // dropping errors silently
            .filter(|entry| !args.tree_only || (entry.mode == Tree))
            .map(|entry| Entry::new(entry.mode, entry.oid.to_owned(), entry.filename.to_owned()))
            .collect::<Vec<_>>()
    };

    let mut out = stdout().lock();
    for entry in entries {
        writeln!(
            out,
            "{:06o} {:4} {}    {}",
            entry.kind as u16,
            entry.kind.as_str(),
            entry.hash,
            entry.path
        )?;
    }

    Ok(())
}

struct Entry {
    kind: EntryMode,
    hash: ObjectId,
    path: BString,
}

impl Entry {
    fn new(kind: EntryMode, hash: ObjectId, path: BString) -> Self {
        Self { kind, hash, path }
    }
}
