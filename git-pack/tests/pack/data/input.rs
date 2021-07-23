mod lookup_ref_delta_objects {
    use git_odb::data::LookupRefDeltaObjectsIter;
    use git_pack::data;
    use git_pack::data::entry::Header;
    use git_pack::data::input;

    const D_A: &[u8] = b"a";
    const D_B: &[u8] = b"bb";

    fn base() -> Header {
        Header::Blob
    }

    fn entry(header: Header, data: &'static [u8]) -> input::Entry {
        let obj = data::Object {
            kind: header.as_kind().unwrap_or(git_object::Kind::Blob),
            data,
            pack_location: None,
        };
        input::Entry::from_data_obj(&obj, 0).expect("valid object")
    }

    fn compute_offsets(mut entries: Vec<input::Entry>) -> Vec<input::Entry> {
        let mut offset = 0;
        for entry in &mut entries {
            entry.pack_offset = offset;
            offset += entry.bytes_in_pack();
        }
        entries
    }

    fn into_results_iter(entries: Vec<input::Entry>) -> impl Iterator<Item = Result<input::Entry, input::Error>> {
        entries.into_iter().map(Ok)
    }

    #[test]
    fn only_ref_deltas_are_handled() {
        let input = compute_offsets(vec![entry(base(), D_A), entry(base(), D_B)]);
        let expected = input.clone();
        assert_eq!(
            LookupRefDeltaObjectsIter::new(into_results_iter(input), |_, _| unreachable!("not going to be called"))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            expected,
            "it won't change the input at all"
        )
    }
}
