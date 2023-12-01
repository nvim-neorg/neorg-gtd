//! Defines ontologies and operations on those ontologies.
//! Design goals:
//! - Allow users to define ontologies as per the specification.
//! - Allow different ways of retrieving information from ontologies (views)
//! - Allow transferring data between ontologies and even completely changing the view
//!   of a given ontology (with proper type checking).

use std::{collections::HashMap, path::PathBuf};

/// A very generic trait of a View.
/// Views allow an [OpaqueOntology]'s contents to be examined and retrieved in a structured manner.
pub trait View {}

/// A trait to display the contents of an Ontology in the form of a list of items.
pub trait ListView<V: View, D: OpaqueOntology<V, Derives = D>, S: OpaqueOntology<V, Derives = D>>:
    View
{
    fn display(&self) -> S::Structure;
}

/// A generic trait of an ontology. The trait is "opaque" because it contains no
/// physical data, only the general structure of an ontology.
///
/// Ontologies are collections of information about a given topic. Examples of ontologies include
/// but are not limited to:
/// - Lists of upcoming tasks
/// - Graphs of connections between notes
/// - Time-sensitive calendars
///
/// All of these are representable via this trait.
///
/// Ontologies may derive from other ontologies if both have the same view (i.e. the same data layout).
pub trait OpaqueOntology<V: View + ?Sized> {
    type Derives: OpaqueOntology<V>;
    type Structure;
}

// Since every ontology must derive from something, here we define an identity ontology. Such an
// ontology has no structure and simply derives from itself.
impl<V: View> OpaqueOntology<V> for () {
    type Structure = ();
    type Derives = Self;
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
