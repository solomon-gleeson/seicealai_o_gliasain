// Types of initial consonant mutations
pub enum Mutation {
    Lenition,
    Eclipsis,
    Prothesis,
}

// Apply a mutation to a single word
pub fn apply_mutation(word: &str, mutation: Mutation) -> String {
    if word.is_empty() {
        return "".to_string();
    }

    match mutation {
        Mutation::Lenition => lenite(word),
        Mutation::Eclipsis => eclipse(word),
        Mutation::Prothesis => prothesise(word),
    }
}

// Lenition: add 'h' after initial consonant if applicable
fn lenite(word: &str) -> String {
    let first = word.chars().next().unwrap();
    let rest: String = word.chars().skip(1).collect();       // Unused variable warning, _rest is only for unused variables, but here rest is used in format!, so it cannot have the underscore.

    match first {
        'b' | 'c' | 'd' | 'f' | 'g' | 'm' | 'p' | 's' | 't' => format!("{}h{}", first, rest),
        _ => word.to_string(),
    }
}

// Eclipsis: prefix certain letters with another consonant
fn eclipse(word: &str) -> String {
    let first = word.chars().next().unwrap();
    let rest: String = word.chars().skip(1).collect();

    let prefix = match first {
        'b' => "m",
        'c' => "g",
        'd' => "n",
        'f' => "bh",
        'g' => "n",
        'p' => "b",
        't' => "d",
        _ => "",
    };

    if prefix.is_empty() {
        word.to_string()
    } else {
        format!("{}{}", prefix, word)
    }
}

// Prothesis: prefix vowel with h
fn prothesise(word: &str) -> String {
    let first = word.chars().next().unwrap();
    if "aeiouáéíóú".contains(first) {
        format!("h{}", word)
    } else {
        word.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lenition() {
        assert_eq!(apply_mutation("bád", Mutation::Lenition), "bhád");
        assert_eq!(apply_mutation("cat", Mutation::Lenition), "chat");
        assert_eq!(apply_mutation("muir", Mutation::Lenition), "mhuir");
        assert_eq!(apply_mutation("eolas", Mutation::Lenition), "eolas");
    }

    #[test]
    fn test_eclipsis() {
        assert_eq!(apply_mutation("bád", Mutation::Eclipsis), "mbád");
        assert_eq!(apply_mutation("cat", Mutation::Eclipsis), "gcat");
        assert_eq!(apply_mutation("fáilte", Mutation::Eclipsis), "bhfáilte");
        assert_eq!(apply_mutation("eolas", Mutation::Eclipsis), "eolas");
    }

    #[test]
    fn test_prothesis() {
        assert_eq!(apply_mutation("athair", Mutation::Prothesis), "hathair");
        assert_eq!(apply_mutation("bád", Mutation::Prothesis), "bád");
    }
}