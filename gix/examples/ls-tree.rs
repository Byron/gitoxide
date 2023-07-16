use clap::Parser;
use gix::{object::Kind, objs::tree::EntryMode, objs::tree::EntryMode::Tree, traverse::tree::Recorder, ObjectId};

fn main() {
    let args = Args::parse_from(gix::env::args_os());
    match run(&args) {
        Ok(()) => {}
        Err(e) => eprintln!("error: {e}"),
    }
}

#[derive(Debug, clap::Parser)]
#[clap(name = "ls-tree", about = "git ls-tree example", version = option_env!("GITOXIDE_VERSION"))]
#[clap(arg_required_else_help = true)]
struct Args {
    #[clap(short = 'r')]
    /// Recurse into subtrees
    recursive: bool,
    #[clap(short = 'd')]
    /// Only show trees
    tree_only: bool,
    #[clap(short = 't')]
    /// Show trees when recursing
    tree_recursing: bool,
    #[clap(name = "tree-ish")]
    /// A revspec pointing to a tree-ish object, e.g. 'HEAD', 'HEAD:src/'
    treeish: String,
}

fn run(args: &Args) -> anyhow::Result<()> {
    let repo = gix::discover(".")?;
    let rev_spec = repo.rev_parse_single(&*args.treeish)?;
    let object = rev_spec.object()?;
    let tree = match object.kind {
        Kind::Commit => object.try_into_commit()?.tree()?,
        Kind::Tree => object.try_into_tree()?,
        _ => anyhow::bail!("not a tree-ish object"),
    };
    // Would like to take the entry arguments directly, but now there is
    // no common trait implementing common field assessors for that.
    let entries = if args.recursive {
        let mut recorder = Recorder::default();
        tree.traverse().breadthfirst(&mut recorder)?;
        recorder
            .records
            .iter()
            .filter(|entry| args.tree_recursing || args.tree_only || entry.mode != Tree)
            .filter(|entry| !args.tree_only || (entry.mode == Tree))
            .map(|entry| Entry::new(entry.mode, entry.oid, entry.filepath.to_string()))
            .collect::<Vec<_>>()
    } else {
        tree.iter()
            .filter_map(std::result::Result::ok) // dropping errors silently
            .filter(|entry| !args.tree_only || (entry.mode() == Tree))
            .map(|entry| Entry::new(entry.inner.mode, entry.id().detach(), entry.inner.filename.to_string()))
            .collect::<Vec<_>>()
    };

    for entry in entries {
        println!("{entry}");
    }

    Ok(())
}

// Helper struct and impl to facilitate displaying as per `git ls-tree`.
use std::fmt::{Display, Formatter};

struct Entry {
    kind: EntryMode,
    hash: ObjectId,
    path: String,
}

impl Entry {
    fn new(kind: EntryMode, hash: ObjectId, path: String) -> Self {
        Self { kind, hash, path }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:06o} {:4} {}    {}",
            self.kind as u16,
            self.kind.as_str().to_string(),
            self.hash,
            self.path,
        )
    }
}
