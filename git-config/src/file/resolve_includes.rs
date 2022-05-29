use crate::file::{from_paths, SectionId};
use crate::parser::Key;
use crate::{values, File};
use bstr::{BString, ByteSlice, ByteVec};
use git_ref::Category;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub(crate) fn resolve_includes(
    conf: &mut File<'_>,
    config_path: Option<&std::path::Path>,
    options: from_paths::Options<'_>,
) -> Result<(), from_paths::Error> {
    resolve_includes_recursive(conf, config_path, 0, options)
}

fn resolve_includes_recursive(
    target_config: &mut File<'_>,
    target_config_path: Option<&Path>,
    depth: u8,
    options: from_paths::Options<'_>,
) -> Result<(), from_paths::Error> {
    if depth == options.max_depth {
        return if options.error_on_max_depth_exceeded {
            Err(from_paths::Error::IncludeDepthExceeded {
                max_depth: options.max_depth,
            })
        } else {
            Ok(())
        };
    }

    let mut paths_to_include = Vec::new();

    let mut incl_section_ids = Vec::new();
    for name in ["include", "includeIf"] {
        for id in target_config.section_ids_by_name(name).unwrap_or_default() {
            incl_section_ids.push((
                id,
                target_config
                    .section_order
                    .iter()
                    .position(|&e| e == id)
                    .expect("section id is from config"),
            ));
        }
    }
    incl_section_ids.sort_by(|a, b| a.1.cmp(&b.1));

    let mut include_paths = Vec::new();
    for (id, _) in incl_section_ids {
        if let Some(header) = target_config.section_headers.get(&id) {
            if header.name.0 == "include" && header.subsection_name.is_none() {
                extract_include_path(target_config, &mut include_paths, id)
            } else if header.name.0 == "includeIf" {
                if let Some(condition) = &header.subsection_name {
                    if include_condition_match(condition, target_config_path, options).is_some() {
                        extract_include_path(target_config, &mut include_paths, id)
                    }
                }
            }
        }
    }

    for path in include_paths {
        let path = resolve(path, target_config_path, options)?;

        if path.is_file() {
            paths_to_include.push(path);
        }
    }

    for config_path in paths_to_include {
        let mut include_config = File::at(&config_path)?;
        resolve_includes_recursive(&mut include_config, Some(&config_path), depth + 1, options)?;
        target_config.append(include_config);
    }
    Ok(())
}

fn extract_include_path<'a>(target_config: &mut File<'a>, include_paths: &mut Vec<values::Path<'a>>, id: SectionId) {
    if let Some(body) = target_config.sections.get(&id) {
        let paths = body.values(&Key::from("path"));
        let paths = paths.iter().map(|path| values::Path::from(path.clone()));
        include_paths.extend(paths);
    }
}

fn include_condition_match(
    condition: &str,
    target_config_path: Option<&Path>,
    options: from_paths::Options<'_>,
) -> Option<()> {
    let (prefix, condition) = condition.split_once(':')?;
    match prefix {
        "gitdir" => gitdir_matches(
            condition,
            target_config_path,
            options,
            git_glob::wildmatch::Mode::empty(),
        ),
        "gitdir/i" => gitdir_matches(
            condition,
            target_config_path,
            options,
            git_glob::wildmatch::Mode::IGNORE_CASE,
        ),
        "onbranch" => {
            let branch_name = options.branch_name?;
            let (_, branch_name) = branch_name
                .category_and_short_name()
                .filter(|(cat, _)| *cat == Category::LocalBranch)?;

            let mut condition = Cow::Borrowed(condition);
            if condition.ends_with('/') {
                condition = Cow::Owned(format!("{}**", condition));
            }
            git_glob::wildmatch(
                condition.as_ref().into(),
                branch_name,
                git_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL,
            )
            .then(|| ())
        }
        _ => None,
    }
}

fn gitdir_matches(
    condition_path: &str,
    target_config_path: Option<&Path>,
    from_paths::Options {
        git_install_dir,
        git_dir,
        ..
    }: from_paths::Options<'_>,
    wildmatch_mode: git_glob::wildmatch::Mode,
) -> Option<()> {
    const DOT: &[u8] = b".";
    const DOT_DOT: &[u8] = b"..";

    let git_dir = git_path::to_unix_separators(git_path::into_bstr(git_dir?));
    if condition_path.contains('\\') {
        return None;
    }
    let mut pattern_path = {
        let cow = Cow::Borrowed(condition_path.as_bytes());
        let path = values::Path::from(cow).interpolate(git_install_dir).ok()?;
        git_path::to_unix_separators(git_path::into_bstr(path)).into_owned()
    };

    if pattern_path.starts_with(DOT) {
        if let Some(parent_path) = target_config_path.and_then(|p| p.parent()) {
            let parent_dir = git_path::to_unix_separators(git_path::into_bstr(parent_path));
            let skip = if pattern_path.starts_with(DOT_DOT) {
                DOT_DOT
            } else {
                DOT
            };
            pattern_path = bstr::concat(&[&parent_dir, &pattern_path[skip.len()..]]).into();
        }
    }

    if ["~/", "./", "/"]
        .iter()
        .all(|prefix| !pattern_path.starts_with(prefix.as_bytes()))
    {
        let v = bstr::concat(&[&b"**/"[..], &pattern_path]);
        pattern_path = BString::from(v);
    }
    if pattern_path.ends_with(b"/") {
        pattern_path.push_str("**");
    }

    let match_mode = git_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL | wildmatch_mode;
    let is_match = git_glob::wildmatch(pattern_path.as_bstr(), git_dir.as_bstr(), match_mode);
    if is_match {
        return Some(());
    }

    let expanded_git_dir = git_path::realpath(git_path::from_byte_slice(&git_dir), target_config_path?).ok()?;
    let expanded_git_dir = git_path::to_unix_separators(git_path::into_bstr(expanded_git_dir));
    git_glob::wildmatch(pattern_path.as_bstr(), expanded_git_dir.as_bstr(), match_mode).then(|| ())
}

fn resolve(
    path: values::Path<'_>,
    target_config_path: Option<&Path>,
    options: from_paths::Options<'_>,
) -> Result<PathBuf, from_paths::Error> {
    let path = path.interpolate(options.git_install_dir)?;
    let path: PathBuf = if path.is_relative() {
        target_config_path
            .ok_or(from_paths::Error::MissingConfigPath)?
            .parent()
            .expect("path is a config file which naturally lives in a directory")
            .join(path)
    } else {
        path.into()
    };
    Ok(path)
}
