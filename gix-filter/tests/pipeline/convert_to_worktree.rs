use std::io::Read;

use bstr::ByteSlice;
use gix_filter::pipeline::CrlfRoundTripCheck;

use crate::{driver::apply::driver_with_process, pipeline::pipeline};

#[test]
fn all_stages() -> gix_testtools::Result {
    let (mut cache, mut pipe) = pipeline("all-filters", || {
        (
            vec![driver_with_process()],
            Vec::new(),
            CrlfRoundTripCheck::Skip,
            Default::default(),
        )
    })?;

    let mut out = pipe.convert_to_worktree(
        b"a\nb\n$Id$",
        "any.txt".into(),
        &mut |path, attrs| {
            cache
                .at_entry(path, Some(false), &gix_object::find::Never)
                .expect("cannot fail")
                .matching_attributes(attrs);
        },
        gix_filter::driver::apply::Delay::Forbid,
    )?;
    assert!(out.is_changed(), "filters were applied");
    assert!(
        out.as_bytes().is_none(),
        "the last filter is a driver which is applied, yielding a stream"
    );
    assert!(out.as_read().is_some(), "process filter is last");
    let mut buf = Vec::new();
    out.read_to_end(&mut buf)?;
    assert_eq!(
        buf.as_bstr(),
        "➡a\r\n➡b\r\n➡$Id: 2188d1cdee2b93a80084b61af431a49d21bc7cc0$",
        "the buffer shows that a lot of transformations were applied"
    );
    Ok(())
}

#[test]
fn all_stages_no_filter() -> gix_testtools::Result {
    let (mut cache, mut pipe) = pipeline("all-filters", || {
        (vec![], Vec::new(), CrlfRoundTripCheck::Skip, Default::default())
    })?;

    let mut out = pipe.convert_to_worktree(
        b"$Id$a\nb\n",
        "other.txt".into(),
        &mut |path, attrs| {
            cache
                .at_entry(path, Some(false), &gix_object::find::Never)
                .expect("cannot fail")
                .matching_attributes(attrs);
        },
        gix_filter::driver::apply::Delay::Forbid,
    )?;
    assert!(out.is_changed(), "filters were applied");
    assert!(
        out.as_read().is_none(),
        "there is no filter process, so no chance for getting a stream"
    );
    let buf = out.as_bytes().expect("no filter process");
    assert_eq!(
        buf.as_bstr(),
        "$Id: a77d7acbc809ac8df987a769221c83137ba1b9f9$a\r\nb\r\n",
        "the buffer shows that a lot of transformations were applied"
    );
    Ok(())
}

#[test]
fn no_filter() -> gix_testtools::Result {
    let (mut cache, mut pipe) = pipeline("no-filters", || {
        (vec![], Vec::new(), CrlfRoundTripCheck::Skip, Default::default())
    })?;

    let input = b"$Id$a\nb\n";
    let out = pipe.convert_to_worktree(
        input,
        "other.txt".into(),
        &mut |path, attrs| {
            cache
                .at_entry(path, Some(false), &gix_object::find::Never)
                .expect("cannot fail")
                .matching_attributes(attrs);
        },
        gix_filter::driver::apply::Delay::Forbid,
    )?;
    assert!(!out.is_changed(), "no filter was applied");
    let actual = out.as_bytes().expect("input is unchanged");
    assert_eq!(actual, input, "so the input is unchanged…");
    assert_eq!(actual.as_ptr(), input.as_ptr(), "…which means it's exactly the same");
    Ok(())
}
