mod lookup_ref_delta_objects {
    use git_hash::ObjectId;
    use git_pack::data::{self, entry::Header, input, LookupRefDeltaObjectsIter};

    const D_A: &[u8] = b"a";
    const D_B: &[u8] = b"bb";
    const D_C: &[u8] = b"ccc";

    fn base() -> Header {
        Header::Blob
    }
    fn delta_ofs(offset: u64) -> Header {
        Header::OfsDelta { base_distance: offset }
    }
    fn delta_ref(id: ObjectId) -> Header {
        Header::RefDelta { base_id: id }
    }

    fn extract_delta_offset(header: &Header) -> u64 {
        match header {
            Header::OfsDelta { base_distance } => *base_distance,
            _ => unreachable!("this is supposed to be an offset header, was {:?}", header),
        }
    }

    fn entry(header: Header, data: &'static [u8]) -> input::Entry {
        let obj = data::Object {
            kind: header.as_kind().unwrap_or(git_object::Kind::Blob),
            data,
            pack_location: None,
        };
        let mut entry = input::Entry::from_data_obj(&obj, 0).expect("valid object");
        entry.header = header;
        entry.header_size = header.size(data.len() as u64) as u16;
        entry
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
        let input = compute_offsets(vec![entry(base(), D_A), entry(delta_ofs(100), D_B)]);
        let expected = input.clone();
        assert_eq!(
            LookupRefDeltaObjectsIter::new(into_results_iter(input), |_, _| unreachable!("not going to be called"))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            expected,
            "it won't change the input at all"
        )
    }

    #[test]
    fn ref_deltas_have_their_base_injected_if_not_done_before_and_all_future_entries_are_offset() {
        let first = entry(base(), D_C);
        let mut inserted = entry(base(), D_B);
        let mut last_entry = entry(base(), D_C);
        // todo: let's have an ofs delta point at the altered entry, maybe even last_entry
        let input = compute_offsets(vec![
            first.clone(),
            entry(delta_ref(ObjectId::null_sha1()), D_A),
            last_entry.clone(),
        ]);

        let mut calls = 0;
        let actual = LookupRefDeltaObjectsIter::new(into_results_iter(input), |_oid, buf| {
            calls += 1;
            buf.resize(D_B.len(), 0);
            buf.copy_from_slice(&D_B);
            Some(data::Object {
                kind: git_object::Kind::Blob,
                data: buf.as_slice(),
                pack_location: None,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

        assert_eq!(calls, 1, "there is only one object to replace");
        assert_eq!(actual.len(), 4, "one object was inserted");
        assert_eq!(&actual[0], &first, "first object is unchanged");

        inserted.pack_offset += first.bytes_in_pack();
        assert_eq!(&actual[1], &inserted, "second object is the inserted one");

        let altered = &actual[2];
        assert_eq!(
            extract_delta_offset(&altered.header),
            inserted.bytes_in_pack(),
            "former second entry is now an offset delta pointing at the item before"
        );
        assert_eq!(
            altered.pack_offset,
            first.bytes_in_pack() + inserted.bytes_in_pack(),
            "the pack offset was adjusted to accommodate for the first and inserted object"
        );

        last_entry.pack_offset = first.bytes_in_pack() + inserted.bytes_in_pack() + altered.bytes_in_pack();
        assert_eq!(
            &actual[3], &last_entry,
            "the last entry was offset and is otherwise unchanged"
        );
    }

    #[test]
    #[ignore]
    fn ref_deltas_have_their_existing_base_injected_to_avoid_duplicate_injection() {}

    #[test]
    #[ignore]
    fn offset_deltas_are_extended_by_the_necessary_amount_of_injected_entries_inbetween() {}

    #[test]
    fn lookup_errors_trigger_a_fuse_and_stop_iteration() {
        let input = vec![entry(delta_ref(ObjectId::null_sha1()), D_A), entry(base(), D_B)];
        let mut calls = 0;
        let mut result = LookupRefDeltaObjectsIter::new(into_results_iter(input), |_, _| {
            calls += 1;
            None
        })
        .collect::<Vec<_>>();
        assert_eq!(calls, 1, "it tries to lookup the object");
        assert_eq!(result.len(), 1, "the error stops iteration");
        assert!(matches!(
            result.pop().expect("one"),
            Err(input::Error::NotFound {
                object_id
            }) if object_id == ObjectId::null_sha1()
        ))
    }

    #[test]
    fn inner_errors_are_passed_on() {
        let input = vec![
            Ok(entry(base(), D_A)),
            Err(input::Error::NotFound {
                object_id: ObjectId::null_sha1(),
            }),
            Ok(entry(base(), D_B)),
        ];
        let expected = vec![
            Ok(entry(base(), D_A)),
            Err(input::Error::NotFound {
                object_id: ObjectId::null_sha1(),
            }),
            Ok(entry(base(), D_B)),
        ];
        let actual = LookupRefDeltaObjectsIter::new(input.into_iter(), |_, _| unreachable!("wont be called"))
            .collect::<Vec<_>>();
        for (actual, expected) in actual.into_iter().zip(expected.into_iter()) {
            assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
        }
    }
}
