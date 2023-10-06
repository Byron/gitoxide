use std::path::Path;

#[test]
fn directories() -> crate::Result {
    baseline::run("directory", true, baseline::directories)
}

#[test]
fn no_pathspecs_match_everything() -> crate::Result {
    let mut search = gix_pathspec::Search::from_specs([], None, Path::new(""))?;
    assert_eq!(search.patterns().count(), 0, "nothing artificial is added");
    let m = search
        .pattern_matching_relative_path("hello".into(), None, &mut |_, _, _, _| {
            unreachable!("must not be called")
        })
        .expect("matches");
    assert_eq!(m.pattern.prefix_directory(), "", "there is no prefix as none was given");

    Ok(())
}

#[test]
fn init_with_exclude() -> crate::Result {
    let search = gix_pathspec::Search::from_specs(pathspecs(&["tests/", ":!*.sh"]), None, Path::new(""))?;
    assert_eq!(search.patterns().count(), 2, "nothing artificial is added");
    assert!(
        search.patterns().next().expect("first of two").is_excluded(),
        "re-orded so that excluded are first"
    );
    assert_eq!(search.common_prefix(), "tests");
    Ok(())
}

#[test]
fn no_pathspecs_respect_prefix() -> crate::Result {
    let mut search = gix_pathspec::Search::from_specs([], Some(Path::new("a")), Path::new(""))?;
    assert_eq!(
        search.patterns().count(),
        1,
        "we get an artificial pattern to get the prefix"
    );
    assert!(
        search
            .pattern_matching_relative_path("hello".into(), None, &mut |_, _, _, _| unreachable!(
                "must not be called"
            ))
            .is_none(),
        "not the right prefix"
    );
    let m = search
        .pattern_matching_relative_path("a/b".into(), None, &mut |_, _, _, _| unreachable!("must not be called"))
        .expect("match");
    assert_eq!(
        m.pattern.prefix_directory(),
        "a",
        "the prefix directory matched verbatim"
    );

    Ok(())
}

#[test]
fn prefixes_are_always_case_insensitive() -> crate::Result {
    let path = gix_testtools::scripted_fixture_read_only("match_baseline_files.sh")?.join("paths");
    let items = baseline::parse_paths(path)?;

    for (spec, prefix, common_prefix, expected) in [
        (":(icase)bar", "FOO", "FOO", &["FOO/BAR", "FOO/bAr", "FOO/bar"] as &[_]),
        (":(icase)bar", "F", "F", &[]),
        (":(icase)bar", "FO", "FO", &[]),
        (":(icase)../bar", "fOo", "", &["BAR", "bAr", "bar"]),
        ("../bar", "fOo", "bar", &["bar"]),
        ("    ", "", "    ", &["    "]),    // whitespace can match verbatim
        ("  hi*", "", "  hi", &["  hi  "]), // whitespace can match with globs as well
        (":(icase)../bar", "fO", "", &["BAR", "bAr", "bar"]), // prefixes are virtual, and don't have to exist at all.
        (
            ":(icase)../foo/bar",
            "FOO",
            "",
            &[
                "FOO/BAR", "FOO/bAr", "FOO/bar", "fOo/BAR", "fOo/bAr", "fOo/bar", "foo/BAR", "foo/bAr", "foo/bar",
            ],
        ),
        ("../foo/bar", "FOO", "foo/bar", &["foo/bar"]),
        (
            ":(icase)../foo/../fOo/bar",
            "FOO",
            "",
            &[
                "FOO/BAR", "FOO/bAr", "FOO/bar", "fOo/BAR", "fOo/bAr", "fOo/bar", "foo/BAR", "foo/bAr", "foo/bar",
            ],
        ),
        ("../foo/../fOo/BAR", "FOO", "fOo/BAR", &["fOo/BAR"]),
    ] {
        let mut search = gix_pathspec::Search::from_specs(
            gix_pathspec::parse(spec.as_bytes(), Default::default()),
            Some(Path::new(prefix)),
            Path::new(""),
        )?;
        assert_eq!(search.common_prefix(), common_prefix, "{spec} {prefix}");
        let actual: Vec<_> = items
            .iter()
            .filter(|relative_path| {
                search
                    .pattern_matching_relative_path(relative_path.as_str().into(), Some(false), &mut |_, _, _, _| false)
                    .is_some()
            })
            .collect();
        assert_eq!(actual, expected, "{spec} {prefix}");
    }
    Ok(())
}

#[test]
fn common_prefix() -> crate::Result {
    for (specs, prefix, expected) in [
        (&["foo/bar", ":(icase)foo/bar"] as &[_], None, ""),
        (&["foo/bar", "foo"], None, "foo"),
        (&["foo/bar/baz", "foo/bar/"], None, "foo/bar"), // directory trailing slashes are ignored, but that prefix shouldn't care anyway
        (&[":(icase)bar", ":(icase)bart"], Some("foo"), "foo"), // only case-sensitive portions count
        (&["bar", "bart"], Some("foo"), "foo/bar"),      // otherwise everything that matches counts
        (&["bar", "bart", "ba"], Some("foo"), "foo/ba"),
    ] {
        let search = gix_pathspec::Search::from_specs(
            specs
                .iter()
                .map(|s| gix_pathspec::parse(s.as_bytes(), Default::default()).expect("valid")),
            prefix.map(Path::new),
            Path::new(""),
        )?;
        assert_eq!(search.common_prefix(), expected, "{specs:?} {prefix:?}");
    }
    Ok(())
}

#[test]
fn files() -> crate::Result {
    baseline::run("file", false, baseline::files)
}

fn pathspecs(input: &[&str]) -> Vec<gix_pathspec::Pattern> {
    input
        .iter()
        .map(|pattern| gix_pathspec::parse(pattern.as_bytes(), Default::default()).expect("known to be valid"))
        .collect()
}

mod baseline {
    use std::path::{Path, PathBuf};

    use bstr::{BString, ByteSlice};

    pub fn run(
        name: &str,
        items_are_dirs: bool,
        init: impl FnOnce() -> crate::Result<(PathBuf, Vec<String>, Vec<Expected>)>,
    ) -> crate::Result {
        let (root, items, expected) = init()?;
        let mut collection = Default::default();
        let attrs =
            gix_attributes::Search::new_globals(Some(root.join(".gitattributes")), &mut Vec::new(), &mut collection)?;
        let tests = expected.len();
        for expected in expected {
            let mut search = gix_pathspec::Search::from_specs(expected.pathspecs, None, Path::new(""))?;
            let actual: Vec<_> = items
                .iter()
                .filter(|path| {
                    search
                        .pattern_matching_relative_path(
                            path.as_str().into(),
                            Some(items_are_dirs),
                            &mut |rela_path, case, is_dir, out| {
                                out.initialize(&collection);
                                attrs.pattern_matching_relative_path(rela_path, case, Some(is_dir), out)
                            },
                        )
                        .map_or(false, |m| !m.is_excluded())
                })
                .cloned()
                .collect();
            let matches_expectation = actual == expected.matches;
            assert_eq!(
                matches_expectation,
                expected.is_consistent,
                "{} - {actual:?} == {:?}",
                search.patterns().map(|p| format!("{p}")).collect::<Vec<_>>().join(", "),
                expected.matches
            );
        }
        eprintln!("{tests} {name} matches OK");
        Ok(())
    }

    #[derive(Debug)]
    pub struct Expected {
        pub pathspecs: Vec<gix_pathspec::Pattern>,
        pub matches: Vec<String>,
        /// If true, this means that the baseline is different from what we get, and that our solution is consistent with the rules.
        pub is_consistent: bool,
    }

    pub fn parse_paths(path: PathBuf) -> std::io::Result<Vec<String>> {
        let buf = std::fs::read(path)?;
        Ok(buf.lines().map(BString::from).map(|s| s.to_string()).collect())
    }

    fn parse_blocks(input: &[u8], parse_matches: impl Fn(&[u8]) -> Vec<String>) -> Vec<Expected> {
        input
            .split(|b| *b == b';')
            .filter(|b| !b.is_empty())
            .map(move |block| {
                let mut lines = block.lines();
                let mut is_inconsistent = false;
                let pathspecs = lines
                    .next()
                    .expect("pathspec")
                    .split(|b| *b == b'+')
                    .filter(|spec| {
                        is_inconsistent = spec.as_bstr() == "git-inconsistency";
                        !is_inconsistent
                    })
                    .filter(|s| !s.trim().is_empty())
                    .map(|pathspec| gix_pathspec::parse(pathspec.trim(), Default::default()).expect("valid pathspec"))
                    .collect();
                Expected {
                    pathspecs,
                    matches: parse_matches(lines.as_bytes()),
                    is_consistent: !is_inconsistent,
                }
            })
            .collect()
    }

    mod submodule {
        use bstr::ByteSlice;

        pub fn matches_from_status(input: &[u8]) -> impl Iterator<Item = (bool, String)> + '_ {
            input.lines().map(|line| {
                let matches = line[0] == b' ';
                assert_eq!(!matches, line[0] == b'-');
                let mut tokens = line[1..].split(|b| *b == b' ').skip(1);
                let path = tokens.next().expect("path").to_str().expect("valid UTF-8");
                (matches, path.to_owned())
            })
        }

        pub fn parse_expected(input: &[u8]) -> Vec<super::Expected> {
            super::parse_blocks(input, |block| {
                matches_from_status(block)
                    .filter_map(|(matches, module_path)| matches.then_some(module_path))
                    .collect()
            })
        }
    }

    mod files {
        use bstr::{BString, ByteSlice};
        pub fn parse_expected(input: &[u8]) -> Vec<super::Expected> {
            super::parse_blocks(input, |block| {
                block.lines().map(BString::from).map(|s| s.to_string()).collect()
            })
        }
    }

    pub fn directories() -> crate::Result<(PathBuf, Vec<String>, Vec<Expected>)> {
        let root = gix_testtools::scripted_fixture_read_only("match_baseline_dirs.sh")?.join("parent");
        let buf = std::fs::read(root.join("paths"))?;
        let items = submodule::matches_from_status(&buf)
            .map(|(_matches, path)| path)
            .collect();
        let expected = submodule::parse_expected(&std::fs::read(root.join("baseline.git"))?);
        Ok((root, items, expected))
    }

    pub fn files() -> crate::Result<(PathBuf, Vec<String>, Vec<Expected>)> {
        let root = gix_testtools::scripted_fixture_read_only("match_baseline_files.sh")?;
        let items = parse_paths(root.join("paths"))?;
        let expected = files::parse_expected(&std::fs::read(root.join("baseline.git"))?);
        Ok((root, items, expected))
    }
}
