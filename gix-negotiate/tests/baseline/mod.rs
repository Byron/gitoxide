use gix_negotiate::Algorithm;
use gix_object::bstr;
use gix_object::bstr::ByteSlice;
use gix_odb::Find;

#[test]
fn run() -> crate::Result {
    let root = gix_testtools::scripted_fixture_read_only("make_repos.sh")?;
    for case in ["no_parents"] {
        let base = root.join(case);

        for (algo_name, algo) in [
            ("noop", Algorithm::Noop),
            // ("consecutive", Algorithm::Consecutive),
            // ("skipping", Algorithm::Skipping),
        ] {
            let buf = std::fs::read(base.join(format!("baseline.{algo_name}")))?;
            let tips = parse::object_ids("", std::fs::read(base.join("tips"))?.lines());
            let store = gix_odb::at(base.join("client").join(".git/objects"))?;

            for use_cache in [false, true] {
                let cache = use_cache
                    .then(|| gix_commitgraph::at(store.store_ref().path().join("info")).ok())
                    .flatten();
                let mut negotiator = algo.into_negotiator(
                    |id, buf| {
                        store
                            .try_find(id, buf)
                            .map(|r| r.and_then(|d| d.try_into_commit_iter()))
                    },
                    cache,
                );
                for tip in &tips {
                    negotiator.add_tip(tip);
                }
                for Round { haves, common } in ParseRounds::new(buf.lines()) {
                    for have in haves {
                        let actual = negotiator.next_have().unwrap_or_else(|| {
                            panic!(
                                "{algo_name}: one have per baseline: {have} missing or in wrong order, left: {:?}",
                                std::iter::from_fn(|| negotiator.next_have()).collect::<Vec<_>>()
                            )
                        });
                        assert_eq!(actual, have, "{algo_name}: order and commit matches exactly");
                    }
                    for common_revision in common {
                        negotiator.in_common_with_remote(&common_revision);
                    }
                }
                assert_eq!(
                    negotiator.next_have(),
                    None,
                    "{algo_name}: negotiator should be depleted after all recorded baseline rounds"
                );
            }
        }
    }
    Ok(())
}

struct ParseRounds<'a> {
    lines: bstr::Lines<'a>,
}

impl<'a> ParseRounds<'a> {
    pub fn new(mut lines: bstr::Lines<'a>) -> Self {
        parse::command(&mut lines, parse::Command::Incoming).expect("handshake");
        Self { lines }
    }
}

impl<'a> Iterator for ParseRounds<'a> {
    type Item = Round;

    fn next(&mut self) -> Option<Self::Item> {
        let haves = parse::object_ids("have", parse::command(&mut self.lines, parse::Command::Outgoing)?);
        let common = parse::object_ids("ACK", parse::command(&mut self.lines, parse::Command::Incoming)?);
        if haves.is_empty() {
            assert!(common.is_empty(), "cannot ack what's not there");
            return None;
        }
        Round { haves, common }.into()
    }
}

struct Round {
    pub haves: Vec<gix_hash::ObjectId>,
    pub common: Vec<gix_hash::ObjectId>,
}

mod parse {
    use gix_object::bstr;
    use gix_object::bstr::{BStr, ByteSlice};

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub enum Command {
        Incoming,
        Outgoing,
    }

    pub fn object_ids(prefix: &str, lines: impl IntoIterator<Item = impl AsRef<[u8]>>) -> Vec<gix_hash::ObjectId> {
        lines
            .into_iter()
            .filter_map(|line| {
                line.as_ref()
                    .strip_prefix(prefix.as_bytes())
                    .map(|id| gix_hash::ObjectId::from_hex(id.trim()).expect("valid hash"))
            })
            .collect()
    }

    pub fn command<'a>(lines: &mut bstr::Lines<'a>, wanted: Command) -> Option<Vec<&'a BStr>> {
        let mut out = Vec::new();
        for line in lines {
            let pos = line.find(b"fetch").expect("fetch token");
            let line_mode = match &line[pos + 5..][..2] {
                b"< " => Command::Incoming,
                b"> " => Command::Outgoing,
                invalid => unreachable!("invalid fetch token: {:?}", invalid.as_bstr()),
            };
            assert_eq!(line_mode, wanted, "command with unexpected mode");
            let line = line[pos + 7..].as_bstr();
            if line == "0000" {
                break;
            }
            out.push(line);
        }
        (!out.is_empty()).then_some(out)
    }
}
