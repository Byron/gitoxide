use std::collections::HashMap;

use bstr::BStr;

use crate::{
    file::{self, SectionBodyIds, SectionId, SectionMut},
    lookup,
    parse::section,
    File,
};

/// Private helper functions
impl<'event> File<'event> {
    /// Adds a new section to the config file.
    pub(crate) fn push_section_internal(&mut self, section: file::Section<'event>) -> SectionMut<'_, 'event> {
        let new_section_id = SectionId(self.section_id_counter);
        self.sections.insert(new_section_id, section);
        let header = &self.sections[&new_section_id].header;
        let lookup = self.section_lookup_tree.entry(header.name.clone()).or_default();

        let mut found_node = false;
        if let Some(subsection_name) = header.subsection_name.clone() {
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
        self.sections
            .get_mut(&new_section_id)
            .map(SectionMut::new)
            .expect("previously inserted section")
    }

    /// Returns the mapping between section and subsection name to section ids.
    pub(crate) fn section_ids_by_name_and_subname<'a>(
        &'a self,
        section_name: &'a str,
        subsection_name: Option<&str>,
    ) -> Result<impl Iterator<Item = SectionId> + ExactSizeIterator + DoubleEndedIterator + '_, lookup::existing::Error>
    {
        let section_name = section::Name::from_str_unchecked(section_name);
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
                    maybe_ids = subsection_lookup.get(subsection_name).map(|v| v.iter().copied());
                    break;
                }
            }
        } else {
            for node in section_ids {
                if let SectionBodyIds::Terminal(subsection_lookup) = node {
                    maybe_ids = Some(subsection_lookup.iter().copied());
                    break;
                }
            }
        }
        maybe_ids.ok_or(lookup::existing::Error::SubSectionMissing)
    }

    pub(crate) fn section_ids_by_name<'a>(
        &'a self,
        section_name: &'a str,
    ) -> Result<impl Iterator<Item = SectionId> + '_, lookup::existing::Error> {
        let section_name = section::Name::from_str_unchecked(section_name);
        match self.section_lookup_tree.get(&section_name) {
            Some(lookup) => Ok(lookup.iter().flat_map({
                let section_order = &self.section_order;
                move |node| match node {
                    SectionBodyIds::Terminal(v) => Box::new(v.iter().copied()) as Box<dyn Iterator<Item = _>>,
                    SectionBodyIds::NonTerminal(v) => Box::new({
                        let v: Vec<_> = v.values().flatten().copied().collect();
                        section_order.iter().filter(move |a| v.contains(a)).copied()
                    }),
                }
            })),
            None => Err(lookup::existing::Error::SectionMissing),
        }
    }

    // TODO: add note indicating that probably a lot if not all information about the original files is currently lost,
    //       so can't be written back. This will probably change a lot during refactor, so it's not too important now.
    pub(crate) fn append(&mut self, mut other: Self) {
        // TODO: don't loose the front-matter here. Not doing so means we know after which section it needs to be inserted, complicating things.
        for id in std::mem::take(&mut other.section_order) {
            let section = other.sections.remove(&id).expect("present");
            self.push_section_internal(section);
        }
    }
}
