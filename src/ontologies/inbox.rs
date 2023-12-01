use crate::ontology::*;

use std::marker::PhantomData;

pub struct Inbox<V: View, D: OpaqueOntology<V>> {
    items: <Self as OpaqueOntology<V>>::Structure,
    view: PhantomData<V>,
    derives: PhantomData<D>,
}

impl<V: View, D: OpaqueOntology<V>> OpaqueOntology<V> for Inbox<V, D> {
    type Structure = Vec<Item>;
    type Derives = D;
}

impl<V: View, D: OpaqueOntology<V>> View for Inbox<V, D> {}

impl<V: View, D: OpaqueOntology<V, Derives = D>> ListView<V, D, Self> for Inbox<V, D> {
    fn display(&self) -> <Self as OpaqueOntology<V>>::Structure {
        self.items.clone()
    }
}
