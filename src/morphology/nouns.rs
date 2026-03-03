use crate::morphology::mutations::{apply_mutation, Mutation};

// Grammatical cases
#[derive(Debug, Clone, Copy)]
pub enum Case {
    Nominative,
    Genitive,
    Vocative,
    Dative, // Expansion later on
}

// Noun struct
#[derive(Debug, Clone)]
pub struct Noun {
    pub lemma: String,        // base form
    pub gender: Gender,       // masculine or feminine
}

// Gender of the noun
#[derive(Debug, Clone, Copy)]
pub enum Gender {
    Masculine,
    Feminine,
}

// Apply mutation to noun based on grammatical case
pub fn mutate_noun(noun: &Noun, grammatical_case: Case) -> String {
    match grammatical_case {
        Case::Nominative => noun.lemma.clone(),
        Case::Genitive => apply_mutation(&noun.lemma, Mutation::Lenition),
        Case::Vocative => apply_mutation(&noun.lemma, Mutation::Eclipsis),
        Case::Dative => noun.lemma.clone(), // placeholder for future rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noun_mutations() {
        let noun1 = Noun { lemma: "bád".to_string(), gender: Gender::Masculine };
        let noun2 = Noun { lemma: "cailín".to_string(), gender: Gender::Feminine };

        assert_eq!(mutate_noun(&noun1, Case::Nominative), "bád");
        assert_eq!(mutate_noun(&noun1, Case::Genitive), "bhád");
        assert_eq!(mutate_noun(&noun1, Case::Vocative), "mbád");

        assert_eq!(mutate_noun(&noun2, Case::Nominative), "cailín");
        assert_eq!(mutate_noun(&noun2, Case::Genitive), "chailín");
        assert_eq!(mutate_noun(&noun2, Case::Vocative), "gcailín");
    }
}