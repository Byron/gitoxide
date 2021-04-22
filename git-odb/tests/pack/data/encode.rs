mod simple_compression {
    use crate::fixture_path;
    use git_odb::linked;
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
    fn all_input_objects() -> crate::Result {
        let _db = db(DbKind::AbunchOfRandomObjects)?;
        Ok(())
    }
}
