use git_object::borrowed;

pub struct Object<'a> {
    pub kind: git_object::Kind,
    pub data: &'a [u8],
}

impl<'a> Object<'a> {
    pub fn decode(&self) -> Result<borrowed::Object, borrowed::Error> {
        Ok(match self.kind {
            git_object::Kind::Tag => borrowed::Object::Tag(borrowed::Tag::from_bytes(self.data)?),
            git_object::Kind::Tree => borrowed::Object::Tree(borrowed::Tree::from_bytes(self.data)?),
            git_object::Kind::Commit => borrowed::Object::Commit(borrowed::Commit::from_bytes(self.data)?),
            git_object::Kind::Blob => borrowed::Object::Blob(borrowed::Blob { data: self.data }),
        })
    }
}
