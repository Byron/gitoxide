use git_hash::ObjectId;
use git_hash::Prefix;
use git_object::bstr::BStr;
use git_revision::parser::Database;
use git_revision::parser::Error as DbError;

use std::convert::TryFrom;

use crate::prelude::ObjectIdExt;

impl Database for crate::Repository {
    fn rev_resolve_head(&self) -> Result<ObjectId, DbError> {
        let id = self.head_id().map_err(|_| DbError::Head)?;
        Ok(id.into())
    }

    fn rev_nth_ancestor(&self, id: ObjectId, n: usize) -> Result<ObjectId, DbError> {
        let mut ancestors_it = id.attach(&self).ancestors().first_parent_only().all().unwrap();
        ancestors_it.next();
        let ids: Result<Vec<_>, _> = ancestors_it.take(n).into_iter().collect();
        let f = ids.map_err(|e| DbError::InvalidNavigation(format!("{}", e)))?;
        if let Some(last) = f.last() {
            Ok(ObjectId::from(*last))
        } else {
            Err(DbError::InvalidNavigation(format!("History to short for ")))
        }
    }

    fn rev_nth_parent(&self, id: ObjectId, n: usize) -> Result<ObjectId, DbError> {
        let commit = id.attach(&self).object().unwrap().try_into_commit().unwrap();
        Ok(commit.parent_ids().take(n).last().unwrap().into())
    }

    fn rev_resolve_ref(&self, input: &BStr) -> Result<Option<ObjectId>, DbError> {
        let r = self
            .try_find_reference(input)
            .map_err(|_| DbError::RefLookUp(input.to_string()))?;
        if let Some(reference) = r {
            let id: ObjectId = reference
                .into_fully_peeled_id()
                .map_err(|_| DbError::RefLookUp(input.to_string()))?
                .into();
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }

    fn rev_find_id(&self, input: &BStr) -> Result<ObjectId, DbError> {
        let obj_id = ObjectId::from_hex(input).map_err(|_| DbError::InvalidHex(input.to_string()))?;
        let _ = self.find_object(obj_id).map_err(|_| DbError::IdLookUp)?;
        Ok(obj_id)
    }

    fn rev_find_by_prefix(&self, input: &BStr) -> Result<ObjectId, DbError> {
        let prefix = Prefix::try_from(input.to_string().as_str())?;
        let r = self
            .objects
            .lookup_prefix(prefix)
            .map_err(|e| DbError::Other(format!("{}", e)))?;
        match r {
            Some(Ok(m)) => Ok(m),
            Some(_) => Err(DbError::IdMultiMatch),
            None => Err(DbError::IdLookUp),
        }
    }
}
