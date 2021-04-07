use git_commitgraph::{graph::Position as GraphPosition, Graph};
use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    hash::BuildHasher,
    io::{BufRead, Cursor},
    path::{Path, PathBuf},
    process::Command,
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod access;

pub fn check_common(cg: &Graph, expected: &HashMap<String, RefInfo, impl BuildHasher>) {
    assert_eq!(
        usize::try_from(cg.num_commits()).expect("an architecture able to hold 32 bits of integer"),
        expected.len()
    );
    for ref_info in expected.values() {
        assert_eq!(cg.id_at(ref_info.pos()), ref_info.id(), "id_at({})", ref_info.pos());
        assert_eq!(
            cg.lookup(ref_info.id()),
            Some(ref_info.pos()),
            "lookup({})",
            ref_info.id()
        );

        let expected_parents: Vec<_> = ref_info
            .parent_ids()
            .into_iter()
            .map(|id| {
                expected
                    .values()
                    .find(|item| item.id() == id)
                    .expect("find RefInfo by id")
            })
            .collect();

        let commit = cg.commit_at(ref_info.pos());
        assert_eq!(commit.id(), ref_info.id());
        assert_eq!(commit.root_tree_id(), ref_info.root_tree_id());
        assert_eq!(
            commit.parent1().expect("failed to access commit's parent1"),
            expected_parents.iter().map(|x| x.pos()).next()
        );
        assert_eq!(
            commit
                .iter_parents()
                .collect::<std::result::Result<Vec<_>, _>>()
                .expect("failed to access commit's parents"),
            expected_parents.iter().map(|x| x.pos()).collect::<Vec<_>>()
        );
    }

    assert_eq!(
        cg.iter_ids().collect::<HashSet<_>>(),
        expected.values().map(|x| x.id()).collect::<HashSet<_>>()
    );
}

pub fn create_repo(script_path: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let status = Command::new("bash")
        .arg(fixture_path(script_path))
        .arg(dir.path())
        .env_remove("GIT_DIR")
        .status()
        .expect("failed to run repo script");
    assert!(status.success(), "repo script failed");
    dir
}

pub fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path)
}

pub fn hex_to_id(hex: &[u8]) -> git_hash::Id {
    git_hash::Id::from_40_bytes_in_hex(hex).expect("40 bytes hex")
}

pub struct RefInfo {
    id: git_hash::Id,
    parent_ids: Vec<git_hash::Id>,
    pos: GraphPosition,
    root_tree_id: git_hash::Id,
}

impl RefInfo {
    pub fn id(&self) -> git_hash::borrowed::Id {
        self.id.to_borrowed()
    }

    pub fn pos(&self) -> GraphPosition {
        self.pos
    }

    pub fn parent_ids(&self) -> impl IntoIterator<Item = git_hash::borrowed::Id> {
        self.parent_ids.iter().map(|x| x.to_borrowed())
    }

    pub fn root_tree_id(&self) -> git_hash::borrowed::Id {
        self.root_tree_id.to_borrowed()
    }
}

pub fn inspect_refs(repo_dir: impl AsRef<Path>, refs: &[&'static str]) -> HashMap<String, RefInfo> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir.as_ref())
        .arg("show")
        .arg("--no-patch")
        .arg("--pretty=format:%S %H %T %P")
        .args(refs)
        .arg("--")
        .env_remove("GIT_DIR")
        .output()
        .expect("failed to execute `git show`");
    // Output format: <refname> <id> <tree_id> <parent_ids>
    let mut infos: Vec<_> = Cursor::new(output.stdout)
        .lines()
        .map(|x| x.expect("failed to read `git show` output"))
        .map(|x| {
            let parts = x.trim_end().split(' ').collect::<Vec<_>>();
            (
                parts[0].to_string(),
                git_hash::Id::from_40_bytes_in_hex(parts[1].as_bytes()).expect("40 bytes hex"),
                git_hash::Id::from_40_bytes_in_hex(parts[2].as_bytes()).expect("40 bytes hex"),
                parts[3..]
                    .iter()
                    .map(|x| git_hash::Id::from_40_bytes_in_hex(x.as_bytes()).expect("40 bytes hex"))
                    .collect(),
            )
        })
        .collect();
    infos.sort_by_key(|x| x.1);

    let get_pos = |id: git_hash::borrowed::Id| -> GraphPosition {
        let pos: u32 = infos
            .binary_search_by_key(&id, |x| x.1.to_borrowed())
            .expect("sorted_ids to contain id")
            .try_into()
            .expect("graph position to fit in u32");
        GraphPosition(pos)
    };

    infos
        .iter()
        .cloned()
        .map(|(name, id, root_tree_id, parent_ids)| {
            (
                name,
                RefInfo {
                    id,
                    parent_ids,
                    root_tree_id,
                    pos: get_pos(id.to_borrowed()),
                },
            )
        })
        .collect()
}
