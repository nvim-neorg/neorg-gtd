//! Defines ontologies and operations on those ontologies.
//! Design goals:
//! - Allow users to define ontologies as per the specification.
//! - Allow different ways of retrieving information from ontologies (views)
//! - Allow transferring data between ontologies and even completely changing the view
//!   of a given ontology (with proper type checking).

use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

pub trait View {
    type Structure;

    fn display<D: OpaqueOntology<Self>>(ontology: &Ontology<Self, D>) -> Self::Structure
    where
        Self: Sized;
}

struct ListView {}

impl View for ListView {
    type Structure = Vec<Item>;

    fn display<D: OpaqueOntology<Self>>(ontology: &Ontology<Self, D>) -> Self::Structure {
        ontology.items.clone()
    }
}

pub trait OpaqueOntology<V: View> {
    type Derives: OpaqueOntology<V>;
}

impl<V: View> OpaqueOntology<V> for () {
    type Derives = Self;
}

pub struct Ontology<V: View, D: OpaqueOntology<V>> {
    items: Vec<Item>,
    phantom_view: PhantomData<V>,
    phantom_derives: PhantomData<D>,
}

impl<V: View, D: OpaqueOntology<V>> OpaqueOntology<V> for Ontology<V, D> {
    type Derives = D;
}

#[derive(Clone)]
enum Metadata {
    Date(chrono::NaiveDate),
    Text(String),
    File(PathBuf),
}

/// An item of an ontology
///
/// * `text`: A required field - the content of the note.
/// * `metadata`: A key : value mapping. Any metadata can be bound to any other metadata if
///               necessary.
#[derive(Clone)]
struct Item {
    text: String,
    metadata: HashMap<Metadata, Metadata>,
}
