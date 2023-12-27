use crate::{
    ontology::{Item, Ontology, View},
    views::List,
};

#[derive(Default)]
pub struct Inbox {
    items: <Inbox as Ontology<List>>::Data,
}

impl Ontology<List> for Inbox {
    type Data = Vec<Item>;

    fn display(&self) -> <List as View>::Data {
        self.items.clone()
    }
}
