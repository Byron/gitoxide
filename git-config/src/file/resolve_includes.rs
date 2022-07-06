use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use bstr::{BStr, BString, ByteSlice, ByteVec};
use git_ref::Category;

use crate::file::from_paths::Options;
use crate::{
    file::{from_paths, SectionId},
    parser::Key,
    values, File,
};

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
            if header.name.0.as_ref() == "include" && header.subsection_name.is_none() {
                extract_include_path(target_config, &mut include_paths, id)
            } else if header.name.0.as_ref() == "includeIf" {
                if let Some(condition) = &header.subsection_name {
                    if include_condition_match(condition.as_ref(), target_config_path, options)? {
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
    condition: &BStr,
    target_config_path: Option<&Path>,
    options: from_paths::Options<'_>,
) -> Result<bool, from_paths::Error> {
    let mut tokens = condition.splitn(2, |b| *b == b':');
    let (prefix, condition) = match (tokens.next(), tokens.next()) {
        (Some(a), Some(b)) => (a, b),
        _ => return Ok(false),
    };
    let condition = condition.as_bstr();
    match prefix {
        b"gitdir" => gitdir_matches(
            condition,
            target_config_path,
            options,
            git_glob::wildmatch::Mode::empty(),
        ),
        b"gitdir/i" => gitdir_matches(
            condition,
            target_config_path,
            options,
            git_glob::wildmatch::Mode::IGNORE_CASE,
        ),
        b"onbranch" => Ok(onbranch_matches(condition, options).is_some()),
        _ => Ok(false),
    }
}

fn onbranch_matches(condition: &BStr, options: Options<'_>) -> Option<()> {
    let branch_name = options.branch_name?;
    let (_, branch_name) = branch_name
        .category_and_short_name()
        .filter(|(cat, _)| *cat == Category::LocalBranch)?;

    let condition = if condition.ends_with(b"/") {
        let mut condition: BString = condition.into();
        condition.push_str("**");
        Cow::Owned(condition)
    } else {
        condition.into()
    };

    git_glob::wildmatch(
        condition.as_ref(),
        branch_name,
        git_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL,
    )
    .then(|| ())
}

fn gitdir_matches(
    condition_path: &BStr,
    target_config_path: Option<&Path>,
    from_paths::Options {
        git_install_dir,
        git_dir,
        home_dir,
        ..
    }: from_paths::Options<'_>,
    wildmatch_mode: git_glob::wildmatch::Mode,
) -> Result<bool, from_paths::Error> {
    let git_dir =
        git_path::to_unix_separators_on_windows(git_path::into_bstr(git_dir.ok_or(from_paths::Error::MissingGitDir)?));

    let mut pattern_path: Cow<'_, _> = {
        let path = values::Path::from(Cow::Borrowed(condition_path)).interpolate(git_install_dir, home_dir)?;
        git_path::into_bstr(path).into_owned().into()
    };
    // NOTE: yes, only if we do path interpolation will the slashes be forced to unix separators on windows
    if pattern_path != condition_path {
        pattern_path = git_path::to_unix_separators_on_windows(pattern_path);
    }

    if let Some(relative_pattern_path) = pattern_path.strip_prefix(b"./") {
        let parent_dir = target_config_path
            .ok_or(from_paths::Error::MissingConfigPath)?
            .parent()
            .expect("config path can never be /");
        let mut joined_path = git_path::to_unix_separators_on_windows(git_path::into_bstr(parent_dir)).into_owned();
        joined_path.push(b'/');
        joined_path.extend_from_slice(relative_pattern_path);
        pattern_path = joined_path.into();
    }

    // NOTE: this special handling of leading backslash is needed to do it like git does
    if pattern_path.iter().next() != Some(&(std::path::MAIN_SEPARATOR as u8))
        && !git_path::from_bstr(pattern_path.clone()).is_absolute()
    {
        let mut prefixed = pattern_path.into_owned();
        prefixed.insert_str(0, "**/");
        pattern_path = prefixed.into()
    }
    if pattern_path.ends_with(b"/") {
        let mut suffixed = pattern_path.into_owned();
        suffixed.push_str("**");
        pattern_path = suffixed.into();
    }

    let match_mode = git_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL | wildmatch_mode;
    let is_match = git_glob::wildmatch(pattern_path.as_bstr(), git_dir.as_bstr(), match_mode);
    if is_match {
        return Ok(true);
    }

    let expanded_git_dir = git_path::into_bstr(git_path::realpath(git_path::from_byte_slice(&git_dir))?);
    Ok(git_glob::wildmatch(
        pattern_path.as_bstr(),
        expanded_git_dir.as_bstr(),
        match_mode,
    ))
}

fn resolve(
    path: values::Path<'_>,
    target_config_path: Option<&Path>,
    from_paths::Options {
        git_install_dir,
        home_dir,
        ..
    }: from_paths::Options<'_>,
) -> Result<PathBuf, from_paths::Error> {
    let path = path.interpolate(git_install_dir, home_dir)?;
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
