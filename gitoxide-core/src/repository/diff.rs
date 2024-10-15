use gix::bstr::{BString, ByteSlice};

pub fn tree(
    repo: gix::Repository,
    out: &mut dyn std::io::Write,
    old_treeish: BString,
    new_treeish: BString,
) -> anyhow::Result<()> {
    let old_tree_id = repo.rev_parse_single(old_treeish.as_bstr())?;
    let new_tree_id = repo.rev_parse_single(new_treeish.as_bstr())?;

    let old_tree = old_tree_id.object()?.peel_to_kind(gix::object::Kind::Tree)?.into_tree();
    let new_tree = new_tree_id.object()?.peel_to_kind(gix::object::Kind::Tree)?.into_tree();

    let changes = repo.diff_tree_to_tree(&old_tree, &new_tree, None)?;

    writeln!(
        out,
        "Diffing trees `{old_treeish}` ({old_tree_id}) -> `{new_treeish}` ({new_tree_id})"
    )?;
    writeln!(out)?;

    write_changes(out, changes)?;

    Ok(())
}

fn write_changes(
    mut out: impl std::io::Write,
    changes: Vec<gix::diff::tree_with_rewrites::Change>,
) -> Result<(), std::io::Error> {
    for change in changes {
        match change {
            gix::diff::tree_with_rewrites::Change::Addition {
                location,
                id,
                entry_mode,
                ..
            } => {
                writeln!(out, "A: {location}")?;
                writeln!(out, "  {id}")?;
                writeln!(out, "  -> {:o}", entry_mode.0)?;
            }
            gix::diff::tree_with_rewrites::Change::Deletion {
                location,
                id,
                entry_mode,
                ..
            } => {
                writeln!(out, "D: {location}")?;
                writeln!(out, "  {id}")?;
                writeln!(out, "  {:o} ->", entry_mode.0)?;
            }
            gix::diff::tree_with_rewrites::Change::Modification {
                location,
                previous_id,
                id,
                previous_entry_mode,
                entry_mode,
            } => {
                writeln!(out, "M: {location}")?;
                writeln!(out, "  {previous_id} -> {id}")?;
                writeln!(out, "  {:o} -> {:o}", previous_entry_mode.0, entry_mode.0)?;
            }
            gix::diff::tree_with_rewrites::Change::Rewrite {
                source_location,
                source_id,
                id,
                location,
                source_entry_mode,
                entry_mode,
                ..
            } => {
                writeln!(out, "R: {source_location} -> {location}")?;
                writeln!(out, "  {source_id} -> {id}")?;
                writeln!(out, "  {:o} -> {:o}", source_entry_mode.0, entry_mode.0)?;
            }
        };
    }

    Ok(())
}
