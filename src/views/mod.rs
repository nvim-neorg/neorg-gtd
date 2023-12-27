use crate::ontology::{Item, View};

pub struct List {}

impl View for List {
    type Data = Vec<Item>;
}
