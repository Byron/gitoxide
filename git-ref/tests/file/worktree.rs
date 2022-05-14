use git_odb::Find;
use git_ref::file::ReferenceExt;
use std::path::PathBuf;

fn dir(packed: bool) -> crate::Result<PathBuf> {
    let name = "make_worktree_repo.sh";
    if packed {
        git_testtools::scripted_fixture_repo_read_only_with_args(name, Some("packed"))
    } else {
        git_testtools::scripted_fixture_repo_read_only(name)
    }
}

fn main_store(packed: bool) -> crate::Result<(git_ref::file::Store, git_odb::Handle)> {
    let dir = dir(packed)?;
    let git_dir = dir.join("repo").join(".git");
    Ok((
        git_ref::file::Store::at(&git_dir, Default::default(), Default::default()),
        git_odb::at(git_dir.join("objects"))?,
    ))
}

fn worktree_store(packed: bool, worktree_name: &str) -> crate::Result<(git_ref::file::Store, git_odb::Handle)> {
    let dir = dir(packed)?;
    let (git_dir, _work_tree) = git_discover::upwards(dir.join(worktree_name))?
        .0
        .into_repository_and_work_tree_directories();
    let common_dir = git_dir.join("../..");
    Ok((
        git_ref::file::Store::for_linked_worktree(git_dir, &common_dir, Default::default(), Default::default()),
        git_odb::at(common_dir.join("objects"))?,
    ))
}

fn into_peel(
    store: &git_ref::file::Store,
    odb: git_odb::Handle,
) -> impl Fn(git_ref::Reference) -> git_hash::ObjectId + '_ {
    move |mut r: git_ref::Reference| {
        r.peel_to_id_in_place(
            store,
            |id, buf| -> Result<Option<(git_object::Kind, &[u8])>, git_odb::store::find::Error> {
                let data = odb.try_find(id, buf)?;
                Ok(data.map(|d| (d.kind, d.data)))
            },
        )
        .unwrap()
    }
}

#[test]
fn linked() {
    for packed in [false, true] {
        let (store, odb) = worktree_store(packed, "w1").unwrap();
        let peel = into_peel(&store, odb);

        let w1_head_id = peel(store.find("HEAD").unwrap());
        let head_id = peel(store.find("main-worktree/HEAD").unwrap());
        assert_ne!(w1_head_id, head_id, "access to main worktree from linked worktree");
        assert_eq!(
            head_id,
            peel(store.find("main-worktree/refs/bisect/bad").unwrap()),
            "main worktree private branch is accessible and points to its head"
        );
        assert_eq!(
            peel(store.find("refs/bisect/bad").unwrap()),
            w1_head_id,
            "this worktrees bisect branch points to its head"
        );
        assert_eq!(
            peel(store.find("worktrees/w-detached/refs/bisect/bad").unwrap()),
            peel(store.find("worktrees/w-detached/HEAD").unwrap()),
            "the detached worktree's bisect branch points to its head"
        );
        assert_eq!(
            w1_head_id,
            peel(store.find("worktrees/w1/HEAD").unwrap()),
            "access ourselves with worktrees prefix works (HEAD)"
        );

        assert_eq!(
            w1_head_id,
            peel(store.find("worktrees/w1/refs/heads/w1").unwrap()),
            "access ourselves with worktrees prefix works (branch)"
        );

        assert_ne!(
            w1_head_id,
            peel(store.find("worktrees/w-detached/HEAD").unwrap()),
            "both point to different ids"
        );
    }
}

#[test]
fn main() {
    for packed in [false, true] {
        let (store, odb) = main_store(packed).unwrap();
        let peel = into_peel(&store, odb);

        let head_id = peel(store.find("HEAD").unwrap());
        assert_eq!(
            head_id,
            peel(store.find("main-worktree/HEAD").unwrap()),
            "main-worktree prefix in pseudorefs from main worktree just works"
        );
        assert_eq!(
            peel(store.find("main").unwrap()),
            peel(store.find("main-worktree/refs/heads/main").unwrap()),
            "main-worktree prefix in pseudorefs from main worktree just works"
        );
        assert_eq!(
            peel(store.find("refs/bisect/bad").unwrap()),
            head_id,
            "bisect is worktree-private"
        );

        let w1_main_id = peel(store.find("w1").unwrap());
        assert_ne!(w1_main_id, head_id, "w1 is checked out at previous commit");

        let w1_head_id = peel(store.find("worktrees/w1/HEAD").unwrap());
        assert_eq!(w1_head_id, w1_main_id, "worktree head points to the branch");
        assert_eq!(
            peel(store.find("worktrees/w1/refs/bisect/bad").unwrap()),
            w1_main_id,
            "linked worktree bisect points to its head"
        );
        assert_eq!(
            w1_head_id,
            peel(store.find("worktrees/w1/refs/heads/w1").unwrap()),
            "worktree branch can be accessed with refs notation too (git doesnt do this right now, but it's documented)"
        );
        let wd_head_id = peel(store.find("worktrees/w-detached/HEAD").unwrap());
        assert_ne!(wd_head_id, w1_main_id, "both worktrees are in different locations");
        assert_eq!(
            peel(store.find("worktrees/w-detached/refs/bisect/bad").unwrap()),
            wd_head_id,
            "detached worktree bisect is at the same location as its HEAD"
        );
        assert_ne!(
            w1_head_id, head_id,
            "access from main to worktree with respective prefix"
        );
    }
}
