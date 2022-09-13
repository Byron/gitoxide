use git_refspec::instruction::Fetch;

#[test]
fn fetch_only() {
    baseline::parse_input().unwrap();
    baseline::parse().unwrap();
    let _spec = Fetch::Only {
        src: "refs/heads/main".into(),
    };
}

mod baseline {
    use bstr::{BString, ByteSlice};
    use git_hash::ObjectId;

    #[derive(Debug)]
    pub struct Ref {
        pub name: BString,
        pub target: ObjectId,
        /// Set if this is a tag, pointing to the tag object itself
        pub tag: Option<ObjectId>,
    }

    pub fn parse_input() -> crate::Result<Vec<Ref>> {
        let dir = git_testtools::scripted_fixture_repo_read_only("match_baseline.sh")?;
        let refs_buf = std::fs::read(dir.join("clone").join("remote-refs.list"))?;
        let mut out = Vec::new();
        for line in refs_buf.lines() {
            let mut tokens = line.splitn(2, |b| *b == b'\t');
            let target = ObjectId::from_hex(tokens.next().expect("hex-sha"))?;
            let name = tokens.next().expect("name");
            if !name.ends_with(b"^{}") {
                out.push(Ref {
                    name: name.into(),
                    target,
                    tag: None,
                })
            } else {
                let last = out.last_mut().unwrap();
                let tag = last.target;
                last.target = target;
                last.tag = Some(tag);
            }
        }
        Ok(out)
    }
    pub fn parse() -> crate::Result {
        let _ = git_testtools::scripted_fixture_repo_read_only("match_baseline.sh")?;
        Ok(())
    }
}
