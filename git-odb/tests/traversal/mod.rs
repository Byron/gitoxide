use crate::hex_to_id;
use git_odb::linked;
use git_odb::linked::Db;

#[test]
fn linear_history_no_branch() -> crate::Result {
    let (_temp_dir, db) = db()?;
    let oids: Result<Vec<_>, _> =
        git_odb::traverse::ancestors::Iter::new(&db, hex_to_id("ff7fe13a4f89b96754bbbc65d3b7e805ef288966")).collect();
    assert_eq!(
        oids?,
        vec![
            hex_to_id("ff7fe13a4f89b96754bbbc65d3b7e805ef288966"),
            hex_to_id("942fe2542ee4ffcdc2a1e0a8a7e39e437c0eca89"),
            hex_to_id("444f769bdf2e45f55119b2f0c59884f5d491a4f4"),
            hex_to_id("77c02176ea29ead1827fb02939f6ce7f2237232e"),
        ]
    );
    Ok(())
}

fn db() -> Result<(tempdir::TempDir, Db), Box<dyn std::error::Error>> {
    let dir = crate::assure_fixture_repo_present("make_traversal_repo.sh")?;
    let db = linked::Db::at(dir.path().join(".git").join("objects"))?;
    Ok((dir, db))
}
