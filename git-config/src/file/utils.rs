use bstr::BStr;
use std::collections::HashMap;

use crate::{
    file::{MutableSection, SectionBody, SectionBodyId, SectionBodyIds},
    lookup,
    parse::section,
    File,
};

/// Private helper functions
impl<'event> File<'event> {
    /// Adds a new section to the config file.
    pub(crate) fn push_section_internal(
        &mut self,
        header: section::Header<'event>,
        section: SectionBody<'event>,
    ) -> MutableSection<'_, 'event> {
        let new_section_id = SectionBodyId(self.section_id_counter);
        self.section_headers.insert(new_section_id, header.clone());
        self.sections.insert(new_section_id, section);
        let lookup = self.section_lookup_tree.entry(header.name).or_default();

        let mut found_node = false;
        if let Some(subsection_name) = header.subsection_name {
            for node in lookup.iter_mut() {
                if let SectionBodyIds::NonTerminal(subsections) = node {
                    found_node = true;
                    subsections
                        .entry(subsection_name.clone())
                        .or_default()
                        .push(new_section_id);
                    break;
                }
            }
            if !found_node {
                let mut map = HashMap::new();
                map.insert(subsection_name, vec![new_section_id]);
                lookup.push(SectionBodyIds::NonTerminal(map));
            }
        } else {
            for node in lookup.iter_mut() {
                if let SectionBodyIds::Terminal(vec) = node {
                    found_node = true;
                    vec.push(new_section_id);
                    break;
                }
            }
            if !found_node {
                lookup.push(SectionBodyIds::Terminal(vec![new_section_id]));
            }
        }
        self.section_order.push_back(new_section_id);
        self.section_id_counter += 1;
        self.sections.get_mut(&new_section_id).map(MutableSection::new).unwrap()
    }

    /// Returns the mapping between section and subsection name to section ids.
    pub(crate) fn section_ids_by_name_and_subname<'a>(
        &self,
        section_name: impl Into<section::Name<'a>>,
        subsection_name: Option<&str>,
    ) -> Result<Vec<SectionBodyId>, lookup::existing::Error> {
        let section_name = section_name.into();
        let section_ids = self
            .section_lookup_tree
            .get(&section_name)
            .ok_or(lookup::existing::Error::SectionMissing)?;
        let mut maybe_ids = None;
        // Don't simplify if and matches here -- the for loop currently needs
        // `n + 1` checks, while the if and matches will result in the for loop
        // needing `2n` checks.
        if let Some(subsection_name) = subsection_name {
            let subsection_name: &BStr = subsection_name.into();
            for node in section_ids {
                if let SectionBodyIds::NonTerminal(subsection_lookup) = node {
                    maybe_ids = subsection_lookup.get(subsection_name);
                    break;
                }
            }
        } else {
            for node in section_ids {
                if let SectionBodyIds::Terminal(subsection_lookup) = node {
                    maybe_ids = Some(subsection_lookup);
                    break;
                }
            }
        }
        maybe_ids
            .map(Vec::to_owned)
            .ok_or(lookup::existing::Error::SubSectionMissing)
    }

    pub(crate) fn section_ids_by_name<'a>(
        &self,
        section_name: impl Into<section::Name<'a>>,
    ) -> Result<Vec<SectionBodyId>, lookup::existing::Error> {
        let section_name = section_name.into();
        self.section_lookup_tree
            .get(&section_name)
            .map(|lookup| {
                lookup
                    .iter()
                    .flat_map(|node| match node {
                        SectionBodyIds::Terminal(v) => v.clone(),
                        SectionBodyIds::NonTerminal(v) => v.values().flatten().copied().collect(),
                    })
                    .collect()
            })
            .ok_or(lookup::existing::Error::SectionMissing)
    }

    // TODO: add note indicating that probably a lot if not all information about the original files is currently lost,
    //       so can't be written back. This will probably change a lot during refactor, so it's not too important now.
    pub(crate) fn append(&mut self, mut other: Self) {
        for id in std::mem::take(&mut other.section_order) {
            let header = other.section_headers.remove(&id).expect("present");
            let body = other.sections.remove(&id).expect("present");
            self.push_section_internal(header, body);
        }
    }
}
