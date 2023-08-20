/// Convert a hexadecimal hash into its corresponding `ObjectId` or _panic_.
fn hex_to_id(hex: &str) -> gix_hash::ObjectId {
    gix_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

mod from_tree {
    use std::{
        convert::Infallible,
        io::{Error, ErrorKind, Read, Write},
        path::PathBuf,
        sync::Arc,
    };

    use gix_attributes::glob::pattern::Case;
    use gix_object::{bstr::ByteSlice, tree::EntryMode};
    use gix_odb::FindExt;
    use gix_testtools::once_cell::sync::Lazy;
    use gix_worktree::stack::state::attributes::Source;

    use crate::hex_to_id;

    #[test]
    fn can_receive_err_if_root_is_not_found() {
        let mut stream = gix_worktree_stream::from_tree(
            gix_hash::Kind::Sha1.null(),
            |_, _| Err(Error::new(ErrorKind::Other, "object retrieval failed")),
            mutating_pipeline(false),
            |_, _, _| -> Result<_, Infallible> { unreachable!("must not be called") },
        );
        let err = stream.next_entry().unwrap_err();
        assert_eq!(err.to_string(), "Could not find a blob or tree for archival");
    }

    #[test]
    fn can_receive_err_if_attribute_not_found() -> gix_testtools::Result {
        let (_dir, head_tree, odb, _cache) = basic()?;
        let mut stream = gix_worktree_stream::from_tree(
            head_tree,
            move |id, buf| odb.find(id, buf),
            mutating_pipeline(false),
            |_, _, _| Err(Error::new(ErrorKind::Other, "attribute retrieval failed")),
        );
        let err = stream.next_entry().unwrap_err();
        assert_eq!(
            err.to_string(),
            "Could not query attributes for path \".gitattributes\""
        );
        Ok(())
    }

    #[test]
    fn will_provide_all_information_and_respect_export_ignore() -> gix_testtools::Result {
        let (dir, head_tree, odb, mut cache) = basic()?;
        let mut stream = gix_worktree_stream::from_tree(
            head_tree,
            {
                let odb = odb.clone();
                move |id, buf| odb.find(id, buf)
            },
            mutating_pipeline(true),
            move |rela_path, mode, attrs| {
                cache
                    .at_entry(rela_path, mode.is_tree().into(), |id, buf| odb.find_blob(id, buf))
                    .map(|entry| entry.matching_attributes(attrs))
                    .map(|_| ())
            },
        );
        stream
            .add_entry_from_path(&dir, &dir.join("extra-file"))?
            .add_entry_from_path(&dir, &dir.join("extra-bigfile"))?
            .add_entry_from_path(&dir, &dir.join("extra-exe"))?
            .add_entry_from_path(&dir, &dir.join("extra-dir-empty"))?
            .add_entry_from_path(&dir, &dir.join("extra-dir").join("symlink-to-extra"))?;

        let tee_read = TeeToMemory {
            read: stream.into_read(),
            write: Default::default(),
        };
        let copy = tee_read.write.clone();
        let mut paths_and_modes = Vec::new();
        let mut stream = gix_worktree_stream::Stream::from_read(tee_read);

        while let Some(mut entry) = stream.next_entry().expect("entry retrieval does not fail") {
            paths_and_modes.push((entry.relative_path().to_owned(), entry.mode, entry.id));
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf).expect("stream can always be read");
            if !buf.is_empty() && entry.mode.is_blob() {
                if entry.relative_path().contains_str("extra") {
                    assert!(
                        buf.find_byte(b'\r').is_none(),
                        "extra-files are not processed in any way"
                    );
                } else if !entry.relative_path().contains_str("big") {
                    assert!(
                        buf.find_byte(b'\r').is_some(),
                        "'{}' did not contain a carriage return as sign of having been filtered",
                        buf.as_bstr()
                    );
                    if entry.relative_path().ends_with_str(b"streamed") {
                        assert_eq!(buf.as_bstr(), "➡streamed-by-driver\r\n");
                    }
                }
            }
        }

        let expected_exe_mode = if cfg!(windows) {
            EntryMode::Blob
        } else {
            EntryMode::BlobExecutable
        };
        let expected_link_mode = if cfg!(windows) {
            EntryMode::Blob
        } else {
            EntryMode::Link
        };
        assert_eq!(
            paths_and_modes,
            &[
                (
                    ".gitattributes".into(),
                    EntryMode::Blob,
                    hex_to_id("45c160c35c17ad264b96431cceb9793160396e99")
                ),
                (
                    "a".into(),
                    EntryMode::Blob,
                    hex_to_id("45b983be36b73c0788dc9cbcb76cbb80fc7bb057")
                ),
                (
                    "bigfile".into(),
                    EntryMode::Blob,
                    hex_to_id("4995fde49ed64e043977e22539f66a0d372dd129")
                ),
                (
                    "symlink-to-a".into(),
                    expected_link_mode,
                    hex_to_id(if cfg!(windows) {
                        "45b983be36b73c0788dc9cbcb76cbb80fc7bb057"
                    } else {
                        "2e65efe2a145dda7ee51d1741299f848e5bf752e"
                    })
                ),
                (
                    "dir/.gitattributes".into(),
                    EntryMode::Blob,
                    hex_to_id("81b9a375276405703e05be6cecf0fc1c8b8eed64")
                ),
                (
                    "dir/b".into(),
                    EntryMode::Blob,
                    hex_to_id("ab4a98190cf776b43cb0fe57cef231fb93fd07e6")
                ),
                (
                    "dir/subdir/exe".into(),
                    expected_exe_mode,
                    hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")
                ),
                (
                    "dir/subdir/streamed".into(),
                    EntryMode::Blob,
                    hex_to_id("08991f58f4de5d85b61c0f87f3ac053c79d0e739")
                ),
                (
                    "extra-file".into(),
                    EntryMode::Blob,
                    hex_to_id("0000000000000000000000000000000000000000")
                ),
                (
                    "extra-bigfile".into(),
                    EntryMode::Blob,
                    hex_to_id("0000000000000000000000000000000000000000")
                ),
                (
                    "extra-exe".into(),
                    expected_exe_mode,
                    hex_to_id("0000000000000000000000000000000000000000")
                ),
                (
                    "extra-dir-empty".into(),
                    EntryMode::Tree,
                    hex_to_id("0000000000000000000000000000000000000000")
                ),
                (
                    "extra-dir/symlink-to-extra".into(),
                    expected_link_mode,
                    hex_to_id("0000000000000000000000000000000000000000")
                )
            ]
        );
        assert_eq!(
            copy.lock().len(),
            320302,
            "keep track of file size changes of the streaming format"
        );

        let mut copied_stream =
            gix_worktree_stream::Stream::from_read(std::io::Cursor::new(copy.lock().as_bytes().to_owned()));
        let mut copied_paths_and_modes = Vec::new();
        let mut buf = Vec::new();
        while let Some(mut entry) = copied_stream.next_entry().expect("entry retrieval does not fail") {
            copied_paths_and_modes.push((entry.relative_path().to_owned(), entry.mode, entry.id));
            buf.clear();
            entry.read_to_end(&mut buf).expect("stream can always be read");
        }
        assert_eq!(
            copied_paths_and_modes, paths_and_modes,
            "a stream copy yields exactly the same result"
        );
        Ok(())
    }

    #[test]
    fn can_drop_entry_without_reading_it() -> gix_testtools::Result {
        let (_dir, head_tree, odb, mut cache) = basic()?;
        let mut stream = gix_worktree_stream::from_tree(
            head_tree,
            {
                let odb = odb.clone();
                move |id, buf| odb.find(id, buf)
            },
            mutating_pipeline(false),
            move |rela_path, mode, attrs| {
                cache
                    .at_entry(rela_path, mode.is_tree().into(), |id, buf| odb.find_blob(id, buf))
                    .map(|entry| entry.matching_attributes(attrs))
                    .map(|_| ())
            },
        );

        drop(stream.next_entry().expect("entry retrieval does not fail"));
        Ok(())
    }

    fn basic() -> gix_testtools::Result<(PathBuf, gix_hash::ObjectId, gix_odb::HandleArc, gix_worktree::Stack)> {
        let dir = gix_testtools::scripted_fixture_read_only("basic.sh")?;

        let head = {
            let hex = std::fs::read(dir.join("head.hex"))?;
            gix_hash::ObjectId::from_hex(hex.trim())?
        };
        let odb = gix_odb::at(dir.join(".git").join("objects"))?;

        let mut collection = Default::default();
        let mut buf = Default::default();
        let attributes = gix_worktree::stack::state::Attributes::new(
            gix_attributes::Search::new_globals(None::<PathBuf>, &mut buf, &mut collection)?,
            None,
            Source::WorktreeThenIdMapping,
            collection,
        );
        let state = gix_worktree::stack::State::AttributesStack(attributes);
        let cache = gix_worktree::Stack::new(&dir, state, Case::Sensitive, Default::default(), Default::default());
        Ok((dir, head, odb.into_arc()?, cache))
    }

    fn mutating_pipeline(driver: bool) -> gix_filter::Pipeline {
        gix_filter::Pipeline::new(
            &Default::default(),
            gix_filter::pipeline::Options {
                drivers: if driver { vec![driver_with_process()] } else { vec![] },
                eol_config: gix_filter::eol::Configuration {
                    auto_crlf: gix_filter::eol::AutoCrlf::Enabled,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    }

    pub(crate) fn driver_with_process() -> gix_filter::Driver {
        let mut exe = DRIVER.to_string_lossy().into_owned();
        if cfg!(windows) {
            exe = exe.replace('\\', "/");
        }
        gix_filter::Driver {
            name: "arrow".into(),
            clean: None,
            smudge: None,
            process: Some((exe + " process").into()),
            required: true,
        }
    }

    static DRIVER: Lazy<PathBuf> = Lazy::new(|| {
        let mut cargo = std::process::Command::new(env!("CARGO"));
        let res = cargo
            .args(["build", "-p=gix-filter", "--example", "arrow"])
            .status()
            .expect("cargo should run fine");
        assert!(res.success(), "cargo invocation should be successful");

        let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
            .ancestors()
            .nth(1)
            .expect("first parent in target dir")
            .join("debug")
            .join("examples")
            .join(if cfg!(windows) { "arrow.exe" } else { "arrow" });
        assert!(path.is_file(), "Expecting driver to be located at {path:?}");
        path
    });

    struct TeeToMemory<R> {
        read: R,
        write: Arc<parking_lot::Mutex<Vec<u8>>>,
    }

    impl<R> std::io::Read for TeeToMemory<R>
    where
        R: std::io::Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let nb = self.read.read(buf)?;
            self.write.lock().write_all(&buf[..nb])?;
            Ok(nb)
        }
    }
}
