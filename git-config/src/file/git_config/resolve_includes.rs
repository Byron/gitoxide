pub(crate) mod function {
    use crate::file::git_config::from_paths;
    use crate::file::SectionBody;
    use crate::parser::{Key, ParsedSectionHeader};
    use crate::{values, File};
    use bstr::{BString, ByteSlice};
    use git_ref::Category;
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    const DOT: &[u8] = b".";

    pub fn resolve_includes(
        conf: &mut File,
        config_path: Option<&std::path::Path>,
        options: from_paths::Options,
    ) -> Result<(), from_paths::Error> {
        resolve_includes_recursive(conf, config_path, 0, options)
    }

    fn resolve_includes_recursive(
        target_config: &mut File,
        target_config_path: Option<&Path>,
        depth: u8,
        options: from_paths::Options,
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

        let mut include_paths = target_config
            .multi_value::<values::Path>("include", None, "path")
            .unwrap_or_default();

        for (header, body) in get_include_if_sections(target_config) {
            if let Some(condition) = &header.subsection_name {
                if include_condition_match(condition, target_config_path, options).is_some() {
                    let paths = body.values(&Key::from("path"));
                    let paths = paths.iter().map(|path| values::Path::from(path.clone()));
                    include_paths.extend(paths);
                }
            }
        }

        for path in include_paths {
            let path = resolve(path, target_config_path, options)?;

            if path.is_file() {
                paths_to_include.push(path);
            }
        }

        dbg!(&paths_to_include);
        for config_path in paths_to_include {
            let mut include_config = File::open(&config_path)?;
            resolve_includes_recursive(&mut include_config, Some(&config_path), depth + 1, options)?;
            target_config.append(include_config);
        }
        Ok(())
    }

    fn include_condition_match(
        condition: &str,
        target_config_path: Option<&Path>,
        options: from_paths::Options,
    ) -> Option<()> {
        let (prefix, condition) = condition.split_once(':')?;
        match prefix {
            "gitdir" => is_match(&target_config_path, options, options.git_dir?, condition).then(|| ()),
            "gitdir/i" => is_match(
                &target_config_path,
                options,
                options.git_dir?,
                &condition.to_lowercase(),
            )
            .then(|| ()),
            "onbranch" => {
                let branch_name = options.branch_name?;
                let (_, branch_name) = branch_name
                    .category_and_short_name()
                    .filter(|(cat, _)| *cat == Category::LocalBranch)?;

                let mut condition = Cow::Borrowed(condition);
                if condition.starts_with('/') {
                    condition = Cow::Owned(format!("**{}", condition));
                }
                if condition.ends_with('/') {
                    condition = Cow::Owned(format!("{}**", condition));
                }
                let pattern = condition.as_bytes().as_bstr();
                dbg!(&branch_name, &pattern);
                git_glob::wildmatch(pattern, branch_name, git_glob::wildmatch::Mode::empty()).then(|| ())
            }
            _ => None,
        }
    }

    fn is_match(
        target_config_path: &Option<&Path>,
        options: from_paths::Options,
        git_dir: &Path,
        condition: &str,
    ) -> bool {
        if condition.contains('\\') {
            return false;
        }
        let condition_path = values::Path::from(Cow::Borrowed(condition.as_bytes()));
        if let Ok(condition_path) = condition_path.interpolate(options.git_install_dir) {
            let mut condition_path = git_path::into_bstr(condition_path).as_bstr().to_owned();
            condition_path = BString::from(condition_path.replace("\\", "/"));

            dbg!(&target_config_path);
            if condition_path.starts_with(DOT) {
                if let Some(parent_dir_path) = target_config_path {
                    if let Some(parent_path) = parent_dir_path.parent() {
                        let parent_dir = git_path::into_bstr(parent_path);
                        let v = bstr::concat(&[parent_dir.as_bstr(), condition_path[DOT.len()..].as_bstr()]);
                        condition_path = BString::from(v);
                        condition_path = BString::from(condition_path.replace("\\", "/"));
                    }
                }
            }
            if !["~/", "./", "/"]
                .iter()
                .any(|&str| condition_path.starts_with(str.as_bytes()))
            {
                let v = bstr::concat(&["**/".as_bytes().as_bstr(), condition_path.as_bstr()]);
                condition_path = BString::from(v);
            }
            if condition_path.ends_with(b"/") {
                condition_path.push(b'*');
                condition_path.push(b'*');
            }

            let git_dir_value = git_path::into_bstr(git_dir).to_mut().replace("\\", "/");

            println!();
            dbg!(&condition_path.as_bstr(), &git_dir_value.as_bstr());
            let mut result = git_glob::wildmatch(
                condition_path.as_bstr(),
                git_dir_value.as_bstr(),
                git_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL,
            );
            if !result {
                if let Some(target_config_path) = target_config_path {
                    if let Ok(expanded_git_dir_value) =
                        git_path::realpath(git_path::from_byte_slice(&git_dir_value), target_config_path, 32)
                    {
                        let git_dir_value = git_path::into_bstr(expanded_git_dir_value).replace("\\", "/");
                        dbg!(&condition_path.as_bstr(), git_dir_value.as_bstr(),);
                        result = git_glob::wildmatch(
                            condition_path.as_bstr(),
                            git_dir_value.as_bstr(),
                            git_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL,
                        );
                    }
                }
            }
            dbg!(&result);
            return result;
        }
        false
    }

    fn get_include_if_sections<'a>(
        target_config: &'a File<'_>,
    ) -> Vec<(&'a ParsedSectionHeader<'a>, &'a SectionBody<'a>)> {
        // TODO can we have same values in section_headers.values()?
        // ADD test
        let section_headers_to_id: HashMap<_, _> = target_config
            .section_headers
            .values()
            .zip(target_config.section_headers.keys())
            .collect();
        let mut include_if_sections = target_config.sections_by_name_with_header("includeIf");
        include_if_sections.sort_by(|a, b| {
            section_headers_to_id
                .get(a.0)
                .expect("section_id exists")
                .cmp(section_headers_to_id.get(b.0).expect("section_id exists"))
        });
        include_if_sections
    }

    fn resolve(
        path: values::Path,
        target_config_path: Option<&Path>,
        options: from_paths::Options,
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
}
