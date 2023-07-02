use clap::Parser;
use gix::{
    date::time::format,
    object::Kind,
    objs::CommitRef,
    traverse::commit::Sorting,
};

fn main() {
    let args = Args::parse_from(gix::env::args_os());
    match run(&args) {
        Ok(()) => {}
        Err(e) => eprintln!("error: {e}"),
    }
}

#[derive(Debug, clap::Parser)]
#[clap(name = "log", about = "git log example", version = option_env!("GITOXIDE_VERSION"))]
struct Args {
    #[clap(name = "dir", long = "git-dir")]
    /// Alternative git directory to use
    git_dir: Option<String>,
    #[clap(name = "count", short = 'c', long = "count")]
    /// Number of commits to return
    count: Option<usize>,
    #[clap(name = "commit")]
    /// The starting commit
    commitish: Option<String>,
    #[clap(name = "path")]
    /// The path interested in log history of
    path: Option<String>,
}

fn run(args: &Args) -> anyhow::Result<()> {
    let repo = gix::discover(
        args.git_dir.as_ref().map_or(".", |s| &s[..])
    )?;
    let object = repo.rev_parse_single(
        args.commitish.as_ref().map_or("HEAD", |s| &s[..])
    )?.object()?;
    let commit = match object.kind {
        Kind::Commit => object.try_into_commit()?,
        _ => anyhow::bail!("not a commit object"),
    };

    let log_entry_iter = repo
        .rev_walk([commit.id])
        .sorting(Sorting::ByCommitTimeNewestFirst)
        .all()?
        .filter(|info| info.as_ref()
            .map_or(true, |info| args.path.as_ref().map_or(true, |path| {
                // TODO should make use of the `git2::DiffOptions`
                // counterpart in gix for a set of files and also to
                // generate diffs.
                let oid = repo.rev_parse_single(
                    format!("{}:{}", info.id, path).as_str()
                ).ok();
                !info.parent_ids
                    .iter()
                    .all(|id| repo.rev_parse_single(
                        format!("{id}:{path}").as_str()
                    ).ok() == oid)
            }))
        )
        .map(|info| {
            let commit = info?.object()?;
            let commit_ref = CommitRef::from_bytes(&commit.data)?;
            let committer = commit_ref.committer;
            Ok(LogEntryInfo {
                commit_id: format!("{}", commit.id()),
                author: format!("{} <{}>",
                    commit_ref.author.name, commit_ref.author.email),
                committer: format!("{} <{}>",
                    commit_ref.committer.name, commit_ref.committer.email),
                commit_time: committer.time.format(format::DEFAULT),
                message: commit_ref.message.to_string(),
            })
        });

    // Collect all items into a Vec to be lazy in code writing
    let log_entries = match args.count {
        Some(count) => log_entry_iter
            .take(count)
            .collect::<anyhow::Result<Vec<_>>>()?,
        None => log_entry_iter
            .collect::<anyhow::Result<Vec<_>>>()?,
    };

    for entry in log_entries {
        println!("commit {}", entry.commit_id);
        println!("Author: {}", entry.committer);
        println!("Date:   {}\n", entry.commit_time);
        for line in entry.message.lines() {
            println!("    {line}");
        }
        println!();
    }

    Ok(())
}

pub struct LogEntryInfo {
    pub commit_id: String,
    pub author: String,
    pub committer: String,
    pub commit_time: String,
    pub message: String,
}
