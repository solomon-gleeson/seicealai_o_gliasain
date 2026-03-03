// src/lib.rs

/// Core library for Seiceálaí Ó Gliasáin
/// Provides Irish grammar checking functionality

// Morphology module handles mutations and will later include nouns, verbs, lexicon, etc.
pub mod morphology {
    pub mod mutations;
    pub mod nouns;
    // pub mod verbs;    // future
    // pub mod lexicon;  // future
}

// Parsing module (future) for tokenization, POS tagging, syntax trees
// pub mod parsing {
//     pub mod tokenizer;
//     pub mod pos_tagging;
//     pub mod syntax_tree;
//     pub mod dependency_graph;
// }

// Rules module (future) for grammar rules checking
// pub mod rules {
//     pub mod base;
//     pub mod agreement;
//     pub mod mutation_rules;
//     pub mod verb_rules;
//     pub mod sentence_rules;
// }

// Report module (future) for generating analysis output
// pub mod report {
//     pub mod formatter;
//     pub mod reporter;
// }