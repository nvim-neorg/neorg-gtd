//! Defines ontologies and operations on those ontologies.
//! Design goals:
//! - Allow users to define ontologies as per the specification.
//! - Allow different ways of retrieving information from ontologies (views)
//! - Allow transferring data between ontologies and even completely changing the view
//!   of a given ontology (with proper type checking).

use std::{collections::HashMap, path::PathBuf};

// An ontology needs some way of defining its structure (and it needs a view over the data)
// The data can be laid out in any way you'd like but a view should transform that data into
// something that can be easily expressable.

pub trait View {
    type Data;
}

pub trait Ontology<V: View> {
    type Data;

    fn display(&self) -> V::Data;
}

/// A unit of data for general metadata.
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
pub struct Item {
    text: String,
    metadata: HashMap<Metadata, Metadata>,
}
