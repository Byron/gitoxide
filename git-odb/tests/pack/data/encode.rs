mod simple_compression {
    use crate::fixture_path;
    use git_features::progress;
    use git_odb::{linked, pack};
    use std::{path::PathBuf, sync::Arc};

    enum DbKind {
        AbunchOfRandomObjects,
    }

    fn db(kind: DbKind) -> crate::Result<Arc<linked::Db>> {
        use DbKind::*;
        let path: PathBuf = match kind {
            AbunchOfRandomObjects => fixture_path("objects"),
        };
        linked::Db::at(path).map_err(Into::into).map(Into::into)
    }

    #[test]
    #[should_panic]
    fn all_input_objects() {
        (|| -> Result<(), Box<dyn std::error::Error>> {
            let db = db(DbKind::AbunchOfRandomObjects)?;
            let obj_count = db.iter().count();
            assert_eq!(obj_count, 146);
            let all_objects = db.arc_iter().flat_map(Result::ok);
            let entries: Vec<_> = pack::data::encode::entries(
                db.clone(),
                all_objects,
                progress::Discard,
                pack::data::encode::Options::default(),
            )
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
            assert_eq!(entries.len(), obj_count, "each object gets one entry");
            Ok(())
        })()
        .unwrap();
    }
}
