// creates a repo with user-specified path (which must not exist)
// adds initial commit with empty tree

use anyhow::Context;
use git::objs::tree;
use git_odb::Write;
use git_repository as git;

fn main() -> anyhow::Result<()> {
    // Note use of args_os:
    // paths may not be UTF-8 encoded and thus can't be forced into a String.
    // gitoxide does not assume encodings that aren't there
    // to match the way git does it as a bare minimum and be just as flexible.
    let git_dir = std::env::args_os()
        .nth(1)
        .context("First argument needs to be the directory to initialize the repository in")?;
    let repo = git::init_bare(git_dir)?;

    println!("Repo (bare): {:?}", repo.git_dir());

    let mut tree = git::objs::Tree::empty();
    let empty_tree_id = repo.write_object(&tree)?;

    let author = git::actor::SignatureRef {
        name: "Maria Sanchez".into(),
        email: "maria@example.com".into(),
        time: git_date::Time::now_local_or_utc(),
    };
    let initial_commit_id = repo.commit(
        "HEAD",
        author,
        author,
        "initial commit",
        empty_tree_id,
        git::commit::NO_PARENT_IDS,
    )?;

    println!("initial commit id with empty tree: {:?}", initial_commit_id);

    let blob_id = repo.objects.write_buf(git_object::Kind::Blob, b"hello world")?;

    let entry = tree::Entry {
        mode: tree::EntryMode::Blob,
        oid: blob_id,
        filename: "hello.txt".into(),
    };

    tree.entries.push(entry);
    let hello_tree_id = repo.write_object(&tree)?;

    let blob_commit_id = repo.commit(
        "HEAD",
        author,
        author,
        "hello commit",
        hello_tree_id,
        [initial_commit_id],
    )?;

    println!("commit id for 'hello world' blob: {:?}", blob_commit_id);

    Ok(())
}
