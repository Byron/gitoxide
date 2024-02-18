use std::path::Path;

#[test]
fn pipeline_in_nonbare_repo_without_index() -> crate::Result {
    let repo = named_subrepo_opts("make_basic_repo.sh", "all-untracked", Default::default())?;
    let _ = repo.filter_pipeline(None).expect("does not fail due to missing index");
    Ok(())
}

use gix::bstr::ByteSlice;
use gix_filter::driver::apply::Delay;

use crate::util::{named_repo, named_subrepo_opts};

#[test]
fn pipeline_in_repo_without_special_options() -> crate::Result {
    let repo = named_repo("make_basic_repo.sh")?;
    let (mut pipe, index) = repo.filter_pipeline(None)?;

    let input = "hi\n";
    {
        let out = pipe.convert_to_git(input.as_bytes(), Path::new("file"), &index)?;
        assert!(!out.is_changed(), "no filtering is configured, nothing changes");
    }

    {
        let out = pipe.convert_to_worktree(input.as_bytes(), "file".into(), Delay::Forbid)?;
        assert!(!out.is_changed(), "no filtering is configured, nothing changes");
    }

    Ok(())
}

#[test]
fn pipeline_with_autocrlf() -> crate::Result {
    let repo = named_repo("make_config_repo.sh")?;
    let (mut pipe, index) = repo.filter_pipeline(None)?;

    let input = "hi\r\n";
    {
        let out = pipe.convert_to_git(input.as_bytes(), Path::new("file"), &index)?;
        assert!(out.is_changed(), "filtering is configured so a change should happen");
        assert_eq!(
            out.as_bytes()
                .expect("a buffer is needed for eol conversions")
                .as_bstr(),
            "hi\n"
        );
    }

    {
        let out = pipe.convert_to_worktree("hi\n".as_bytes(), "file".into(), Delay::Forbid)?;
        assert_eq!(
            out.as_bytes()
                .expect("a buffer is needed for eol conversions")
                .as_bstr(),
            input,
            "autocrlf converts text LF to CRLF in the worktree"
        );
    }
    Ok(())
}
