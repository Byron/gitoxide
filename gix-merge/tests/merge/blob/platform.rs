use gix_merge::blob::{pipeline, ResourceKind};
use gix_object::tree::EntryKind;
use gix_worktree::stack::state::attributes;

use gix_merge::blob::Platform;

#[test]
fn ancestor_and_current_and_other_do_not_exist() -> crate::Result {
    let mut platform = new_platform(None, pipeline::Mode::default());
    platform.set_resource(
        gix_hash::Kind::Sha1.null(),
        EntryKind::Blob,
        "also-missing".into(),
        ResourceKind::CommonAncestorOrBase,
        &gix_object::find::Never,
    )?;

    platform.set_resource(
        gix_hash::Kind::Sha1.null(),
        EntryKind::Blob,
        "can't-be-found-in-odb".into(),
        ResourceKind::CurrentOrOurs,
        &gix_object::find::Never,
    )?;
    platform.set_resource(
        gix_hash::Kind::Sha1.null(),
        EntryKind::BlobExecutable,
        "can't-be-found-in-odb".into(),
        ResourceKind::OtherOrTheirs,
        &gix_object::find::Never,
    )?;

    let state = platform
        .prepare_merge_state(&gix_object::find::Never)
        .expect("no validation is done here, let the caller inspect");
    assert_eq!(state.ancestor.data.as_slice(), None);
    assert_eq!(state.current.data.as_slice(), None);
    assert_eq!(state.other.data.as_slice(), None);
    Ok(())
}

mod set_resource {
    use crate::blob::platform::new_platform;
    use gix_merge::blob::{pipeline, ResourceKind};
    use gix_object::tree::EntryKind;

    #[test]
    fn invalid_resource_types() {
        let mut platform = new_platform(None, pipeline::Mode::ToGit);
        for (mode, name) in [(EntryKind::Commit, "Commit"), (EntryKind::Tree, "Tree")] {
            assert_eq!(
                platform
                    .set_resource(
                        gix_hash::Kind::Sha1.null(),
                        mode,
                        "a".into(),
                        ResourceKind::OtherOrTheirs,
                        &gix_object::find::Never,
                    )
                    .unwrap_err()
                    .to_string(),
                format!("Can only diff blobs, not {name}")
            );
        }
    }
}

fn new_platform(
    drivers: impl IntoIterator<Item = gix_merge::blob::Driver>,
    filter_mode: gix_merge::blob::pipeline::Mode,
) -> Platform {
    let root = gix_testtools::scripted_fixture_read_only("make_blob_repo.sh").expect("valid fixture");
    let attributes = gix_worktree::Stack::new(
        &root,
        gix_worktree::stack::State::AttributesStack(gix_worktree::stack::state::Attributes::new(
            Default::default(),
            None,
            attributes::Source::WorktreeThenIdMapping,
            Default::default(),
        )),
        gix_worktree::glob::pattern::Case::Sensitive,
        Vec::new(),
        Vec::new(),
    );
    let filter = gix_merge::blob::Pipeline::new(
        gix_merge::blob::pipeline::WorktreeRoots {
            common_ancestor_root: Some(root.clone()),
            ..Default::default()
        },
        gix_filter::Pipeline::default(),
        Default::default(),
    );
    Platform::new(
        filter,
        filter_mode,
        attributes,
        drivers.into_iter().collect(),
        Default::default(),
    )
}

//
// #[test]
// fn with_driver() -> crate::Result {
//     let root = gix_testtools::scripted_fixture_read_only("make_blob_repo.sh")?;
//     let print_all = "echo $@ %O %A %B %L %P %S %X %Y";
//     let print_script_args = "echo $@";
//     let mut attributes = gix_worktree::Stack::new(
//         &root,
//         gix_worktree::stack::State::AttributesStack(gix_worktree::stack::state::Attributes::new(
//             Default::default(),
//             None,
//             attributes::Source::WorktreeThenIdMapping,
//             Default::default(),
//         )),
//         gix_worktree::glob::pattern::Case::Sensitive,
//         Vec::new(),
//         Vec::new(),
//     );
//     let mut filter = gix_merge::blob::Pipeline::new(
//         WorktreeRoots {
//             common_ancestor_root: Some(root.clone()),
//             ..Default::default()
//         },
//         gix_filter::Pipeline::default(),
//         vec![
//             gix_merge::blob::Driver {
//                 name: "a".into(),
//                 command: print_all.into(),
//                 ..Default::default()
//             },
//             gix_merge::blob::Driver {
//                 name: "b".into(),
//                 command: print_script_args.into(),
//                 ..Default::default()
//             },
//             gix_merge::blob::Driver {
//                 name: "union".into(),
//                 ..Default::default()
//             },
//             gix_merge::blob::Driver {
//                 name: "missing".into(),
//                 ..Default::default()
//             },
//         ],
//         pipeline::Options {
//             default_driver: Some("binary".into()),
//             ..crate::blob::pipeline::default_options()
//         },
//     );
//
//     let mut buf = Vec::new();
//     let does_not_matter = gix_hash::Kind::Sha1.null();
//     let path = "unspecified";
//     let platform = attributes.at_entry(path, None, &gix_object::find::Never)?;
//     let out = filter.convert_to_mergeable(
//         &does_not_matter,
//         EntryKind::Blob,
//         path.into(),
//         ResourceKind::CommonAncestorOrBase,
//         &mut |_, out| {
//             let _ = platform.matching_attributes(out);
//         },
//         &gix_object::find::Never,
//         pipeline::Mode::ToGit,
//         &mut buf,
//     )?;
//     assert_eq!(
//         out.driver,
//         DriverChoice::BuiltIn(BuiltinDriver::Binary),
//         "fall through to what's set in options"
//     );
//     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     assert_eq!(buf.as_bstr(), "unspecified\n");
//
//     let path = "union";
//     let platform = attributes.at_entry(path, None, &gix_object::find::Never)?;
//     let out = filter.convert_to_mergeable(
//         &does_not_matter,
//         EntryKind::Blob,
//         path.into(),
//         ResourceKind::CommonAncestorOrBase,
//         &mut |_, out| {
//             let _ = platform.matching_attributes(out);
//         },
//         &gix_object::find::Never,
//         pipeline::Mode::ToGit,
//         &mut buf,
//     )?;
//     let driver_idx = 3;
//     assert_eq!(
//         out.driver,
//         DriverChoice::Index(driver_idx),
//         "it finds explicit drivers first before it searches built-in ones"
//     );
//     assert_eq!(
//         filter.drivers()[driver_idx].name,
//         "union",
//         "it has re-sorted the drivers internally, which is why it's read-only"
//     );
//     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     assert_eq!(buf.as_bstr(), "union\n");
//     //
//     // let mut db = ObjectDb::default();
//     // let null = gix_hash::Kind::Sha1.null();
//     // let mut buf = Vec::new();
//     // let platform = attributes.at_entry("a", None, &gix_object::find::Never)?;
//     // let worktree_modes = [
//     //     pipeline::Mode::ToWorktreeAndBinaryToText,
//     //     pipeline::Mode::ToGitUnlessBinaryToTextIsPresent,
//     // ];
//     // let all_modes = [
//     //     pipeline::Mode::ToGit,
//     //     pipeline::Mode::ToWorktreeAndBinaryToText,
//     //     pipeline::Mode::ToGitUnlessBinaryToTextIsPresent,
//     // ];
//     // for mode in worktree_modes {
//     //     let out = filter.convert_to_diffable(
//     //         &null,
//     //         EntryKind::Blob,
//     //         "a".into(),
//     //         ResourceKind::OldOrSource,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &gix_object::find::Never,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(0));
//     //     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     //     assert_eq!(buf.as_bstr(), "to-text\na\n", "filter was applied");
//     // }
//     //
//     // let out = filter.convert_to_diffable(
//     //     &null,
//     //     EntryKind::Blob,
//     //     "a".into(),
//     //     ResourceKind::OldOrSource,
//     //     &mut |_, out| {
//     //         let _ = platform.matching_attributes(out);
//     //     },
//     //     &gix_object::find::Never,
//     //     pipeline::Mode::ToGit,
//     //     &mut buf,
//     // )?;
//     // assert_eq!(out.driver_index, Some(0));
//     // assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     // assert_eq!(buf.as_bstr(), "a\n", "unconditionally use git according to mode");
//     //
//     // let id = db.insert("a\n");
//     // for mode in worktree_modes {
//     //     let out = filter.convert_to_diffable(
//     //         &id,
//     //         EntryKind::Blob,
//     //         "a".into(),
//     //         ResourceKind::NewOrDestination,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &db,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(0));
//     //     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     //     assert_eq!(buf.as_bstr(), "to-text\na\n", "filter was applied");
//     // }
//     //
//     // let out = filter.convert_to_diffable(
//     //     &id,
//     //     EntryKind::Blob,
//     //     "a".into(),
//     //     ResourceKind::NewOrDestination,
//     //     &mut |_, out| {
//     //         let _ = platform.matching_attributes(out);
//     //     },
//     //     &db,
//     //     pipeline::Mode::ToGit,
//     //     &mut buf,
//     // )?;
//     // assert_eq!(out.driver_index, Some(0));
//     // assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     // assert_eq!(
//     //     buf.as_bstr(),
//     //     "a\n",
//     //     "no filter was applied in this mode, also when using the ODB"
//     // );
//     //
//     // let platform = attributes.at_entry("missing", None, &gix_object::find::Never)?;
//     // for mode in all_modes {
//     //     buf.push(1);
//     //     let out = filter.convert_to_diffable(
//     //         &null,
//     //         EntryKind::Link,
//     //         "missing".into(), /* does not actually exist */
//     //         ResourceKind::OldOrSource,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &gix_object::find::Never,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(4), "despite missing, we get driver information");
//     //     assert_eq!(out.data, None);
//     //     assert_eq!(buf.len(), 0, "always cleared");
//     //
//     //     buf.push(1);
//     //     let out = filter.convert_to_diffable(
//     //         &null,
//     //         EntryKind::Link,
//     //         "missing".into(), /* does not actually exist */
//     //         ResourceKind::NewOrDestination,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &gix_object::find::Never,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(4), "despite missing, we get driver information");
//     //     assert_eq!(out.data, None);
//     //     assert_eq!(buf.len(), 0, "always cleared");
//     //
//     //     buf.push(1);
//     //     let id = db.insert("link-target");
//     //     let out = filter.convert_to_diffable(
//     //         &id,
//     //         EntryKind::Link,
//     //         "missing".into(),
//     //         ResourceKind::NewOrDestination,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &db,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(4), "despite missing, we get driver information");
//     //     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     //     assert_eq!(
//     //         buf.as_bstr(),
//     //         "link-target",
//     //         "no matter what, links always look the same."
//     //     );
//     // }
//
//     // let platform = attributes.at_entry("b", None, &gix_object::find::Never)?;
//     // for mode in all_modes {
//     //     buf.push(1);
//     //     let out = filter.convert_to_diffable(
//     //         &null,
//     //         EntryKind::Blob,
//     //         "b".into(),
//     //         ResourceKind::OldOrSource,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &gix_object::find::Never,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //
//     //     assert_eq!(out.driver_index, Some(1));
//     //     assert_eq!(
//     //         out.data,
//     //         Some(pipeline::Data::Binary { size: 2 }),
//     //         "binary value comes from driver, and it's always respected with worktree source"
//     //     );
//     //     assert_eq!(buf.len(), 0, "it's always cleared before any potential use");
//     // }
//     //
//     // let id = db.insert("b\n");
//     // for mode in all_modes {
//     //     buf.push(1);
//     //     let out = filter.convert_to_diffable(
//     //         &id,
//     //         EntryKind::Blob,
//     //         "b".into(),
//     //         ResourceKind::NewOrDestination,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &db,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //
//     //     assert_eq!(out.driver_index, Some(1));
//     //     assert_eq!(
//     //         out.data,
//     //         Some(pipeline::Data::Binary { size: 2 }),
//     //         "binary value comes from driver, and it's always respected with DB source"
//     //     );
//     //     assert_eq!(buf.len(), 0, "it's always cleared before any potential use");
//     // }
//     //
//     // let platform = attributes.at_entry("c", None, &gix_object::find::Never)?;
//     // for mode in worktree_modes {
//     //     let out = filter.convert_to_diffable(
//     //         &null,
//     //         EntryKind::Blob,
//     //         "c".into(),
//     //         ResourceKind::OldOrSource,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &gix_object::find::Never,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(2));
//     //     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     //     assert_eq!(
//     //         buf.as_bstr(),
//     //         "to-text\nc\n",
//     //         "filter was applied, it overrides binary=true"
//     //     );
//     // }
//     //
//     // let id = db.insert("c\n");
//     // for mode in worktree_modes {
//     //     let out = filter.convert_to_diffable(
//     //         &id,
//     //         EntryKind::Blob,
//     //         "c".into(),
//     //         ResourceKind::NewOrDestination,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &db,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(2));
//     //     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     //     assert_eq!(
//     //         buf.as_bstr(),
//     //         "to-text\nc\n",
//     //         "filter was applied, it overrides binary=true"
//     //     );
//     // }
//     //
//     // let platform = attributes.at_entry("unset", None, &gix_object::find::Never)?;
//     // for mode in all_modes {
//     //     let out = filter.convert_to_diffable(
//     //         &null,
//     //         EntryKind::Blob,
//     //         "unset".into(),
//     //         ResourceKind::OldOrSource,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &gix_object::find::Never,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(
//     //         out.driver_index, None,
//     //         "no driver is associated, as `diff` is explicitly unset"
//     //     );
//     //     assert_eq!(
//     //         out.data,
//     //         Some(pipeline::Data::Binary { size: 6 }),
//     //         "unset counts as binary"
//     //     );
//     //     assert_eq!(buf.len(), 0);
//     // }
//     //
//     // let id = db.insert("unset\n");
//     // for mode in all_modes {
//     //     let out = filter.convert_to_diffable(
//     //         &id,
//     //         EntryKind::Blob,
//     //         "unset".into(),
//     //         ResourceKind::NewOrDestination,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &db,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(
//     //         out.driver_index, None,
//     //         "no driver is associated, as `diff` is explicitly unset"
//     //     );
//     //     assert_eq!(
//     //         out.data,
//     //         Some(pipeline::Data::Binary { size: 6 }),
//     //         "unset counts as binary"
//     //     );
//     //     assert_eq!(buf.len(), 0);
//     // }
//     //
//     // let platform = attributes.at_entry("d", None, &gix_object::find::Never)?;
//     // let id = db.insert("d-in-db");
//     // for mode in worktree_modes {
//     //     let out = filter.convert_to_diffable(
//     //         &null,
//     //         EntryKind::Blob,
//     //         "d".into(),
//     //         ResourceKind::OldOrSource,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &gix_object::find::Never,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(3));
//     //     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     //     assert_eq!(
//     //         buf.as_bstr(),
//     //         "to-text\nd\n",
//     //         "the worktree + text conversion was triggered for worktree source"
//     //     );
//     //
//     //     let out = filter.convert_to_diffable(
//     //         &id,
//     //         EntryKind::Blob,
//     //         "d".into(),
//     //         ResourceKind::NewOrDestination,
//     //         &mut |_, out| {
//     //             let _ = platform.matching_attributes(out);
//     //         },
//     //         &db,
//     //         mode,
//     //         &mut buf,
//     //     )?;
//     //     assert_eq!(out.driver_index, Some(3));
//     //     assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     //     assert_eq!(
//     //         buf.as_bstr(),
//     //         "to-text\nd-in-db",
//     //         "the worktree + text conversion was triggered for db source"
//     //     );
//     // }
//     //
//     // let platform = attributes.at_entry("e-no-attr", None, &gix_object::find::Never)?;
//     // let out = filter.convert_to_diffable(
//     //     &null,
//     //     EntryKind::Blob,
//     //     "e-no-attr".into(),
//     //     ResourceKind::OldOrSource,
//     //     &mut |_, out| {
//     //         let _ = platform.matching_attributes(out);
//     //     },
//     //     &gix_object::find::Never,
//     //     pipeline::Mode::ToGitUnlessBinaryToTextIsPresent,
//     //     &mut buf,
//     // )?;
//     // assert_eq!(out.driver_index, None);
//     // assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     // assert_eq!(
//     //     buf.as_bstr(),
//     //     "e\n",
//     //     "no text filter, so git conversion was applied for worktree source"
//     // );
//     //
//     // let id = db.insert("e-in-db");
//     // let out = filter.convert_to_diffable(
//     //     &id,
//     //     EntryKind::Blob,
//     //     "e-no-attr".into(),
//     //     ResourceKind::NewOrDestination,
//     //     &mut |_, out| {
//     //         let _ = platform.matching_attributes(out);
//     //     },
//     //     &db,
//     //     pipeline::Mode::ToGitUnlessBinaryToTextIsPresent,
//     //     &mut buf,
//     // )?;
//     // assert_eq!(out.driver_index, None);
//     // assert_eq!(out.data, Some(pipeline::Data::Buffer));
//     // assert_eq!(
//     //     buf.as_bstr(),
//     //     "e-in-db",
//     //     "no text filter, so git conversion was applied for ODB source"
//     // );
//
//     Ok(())
// }
