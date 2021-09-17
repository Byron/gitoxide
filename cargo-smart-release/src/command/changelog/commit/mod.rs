pub mod history;

mod message {}

pub struct History {
    pub head: git_repository::refs::Reference,
    pub items: Vec<history::Item>,
}
