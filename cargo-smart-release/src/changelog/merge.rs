use crate::changelog::{Section, Version};
use crate::ChangeLog;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::FromIterator;

impl ChangeLog {
    /// Bring `generated` into `self` in such a way that `self` preserves everything while enriching itself from `generated`.
    /// Thus we clearly assume that `self` is parsed and `generated` is generated.
    pub fn merge_generated(mut self, rhs: Self) -> Self {
        if self.sections.is_empty() {
            return rhs;
        }

        let mut sections_to_merge = VecDeque::from_iter(rhs.sections);
        let sections = &mut self.sections;

        merge_generated_verbatim_section_if_there_is_only_releases_on_lhs(&mut sections_to_merge, sections);

        let _first_release_indentation = match sections.iter().find_map(|s| match s {
            Section::Release { heading_level, .. } => Some(heading_level),
            _ => None,
        }) {
            Some(level) => level,
            None => {
                sections.extend(sections_to_merge);
                return self;
            }
        };

        for section_to_merge in sections_to_merge {
            match section_to_merge {
                Section::Verbatim { .. } => {
                    unreachable!("BUG: generated logs may only have verbatim sections at the beginning")
                }
                Section::Release { ref name, .. } => match find_target_section(name, sections) {
                    (pos, Ordering::Equal) => merge_section(&mut sections[pos], section_to_merge),
                    (pos, Ordering::Greater) => sections.insert(pos, section_to_merge),
                    (pos, Ordering::Less) => sections.insert(pos + 1, section_to_merge),
                },
            }
        }

        self
    }
}

fn find_target_section(_name: &Version, _sections: &[Section]) -> (usize, Ordering) {
    todo!("find insertion point")
}

fn merge_section(_dest: &mut Section, _src: Section) {
    todo!("actual merge")
}

fn merge_generated_verbatim_section_if_there_is_only_releases_on_lhs(
    sections_to_merge: &mut VecDeque<Section>,
    sections: &mut Vec<Section>,
) {
    while let Some(section_to_merge) = sections_to_merge.pop_front() {
        match section_to_merge {
            Section::Verbatim { generated, .. } => {
                assert!(generated, "BUG: rhs must always be generated");
                let first_section = &sections[0];
                if matches!(first_section, Section::Release { .. })
                    || matches!(first_section, Section::Verbatim {generated, ..} if *generated )
                {
                    sections.insert(0, section_to_merge)
                }
            }
            Section::Release { .. } => {
                sections_to_merge.push_front(section_to_merge);
                break;
            }
        }
    }
}
