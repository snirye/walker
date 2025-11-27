/// Keyboard layout variant support for fuzzy matching.
///
/// This module provides functionality to generate layout variants of a query string,
/// allowing fuzzy matching to work across different keyboard layouts. Currently
/// supports Hebrew → English QWERTY mapping. The implementation is extendable for
/// other layout pairs in the future.
use std::collections::HashMap;

/// Returns a list of layout variants for the given query.
///
/// Always includes the original query as the first variant. For each known layout mapping,
/// generates additional variants by translating characters according to their physical
/// key positions.
///
/// # Arguments
/// * `query` - The original query string typed by the user
///
/// # Returns
/// A vector of unique string variants, with the original query first
pub fn layout_variants(query: &str) -> Vec<String> {
    let mut variants = Vec::new();

    // Always include the original query first
    variants.push(query.to_string());

    // Generate Hebrew → English variant
    let hebrew_to_english = generate_hebrew_to_english_variant(query);
    if hebrew_to_english != query && !variants.contains(&hebrew_to_english) {
        variants.push(hebrew_to_english);
    }

    variants
}

/// Generates an English QWERTY variant from Hebrew keyboard input.
///
/// Maps Hebrew characters to their corresponding English characters based on
/// physical key positions on a standard keyboard layout.
fn generate_hebrew_to_english_variant(query: &str) -> String {
    let mapping = hebrew_to_english_map();

    query
        .chars()
        .map(|c| *mapping.get(&c).unwrap_or(&c))
        .collect()
}

/// Returns a mapping from Hebrew characters to English QWERTY characters.
///
/// The mapping is based on physical key positions - typing in Hebrew mode on a
/// QWERTY keyboard produces Hebrew characters at certain positions, and this map
/// reverses that to find what English character would be produced at the same
/// physical key.
fn hebrew_to_english_map() -> HashMap<char, char> {
    // Hebrew keyboard layout mapped to QWERTY positions
    // This maps Hebrew characters to the English characters on the same physical keys
    let pairs: &[(char, char)] = &[
        // Top row (q w e r t y u i o p)
        ('/', 'q'),  // Hebrew / on q key
        ('\'', 'w'), // Hebrew ' on w key
        ('ק', 'e'),  // Qof
        ('ר', 'r'),  // Resh
        ('א', 't'),  // Alef
        ('ט', 'y'),  // Tet
        ('ו', 'u'),  // Vav
        ('ן', 'i'),  // Final Nun
        ('ם', 'o'),  // Final Mem
        ('פ', 'p'),  // Pe
        // Home row (a s d f g h j k l ;)
        ('ש', 'a'), // Shin
        ('ד', 's'), // Dalet
        ('ג', 'd'), // Gimel
        ('כ', 'f'), // Kaf
        ('ע', 'g'), // Ayin
        ('י', 'h'), // Yod
        ('ח', 'j'), // Chet
        ('ל', 'k'), // Lamed
        ('ך', 'l'), // Final Kaf
        ('ף', ';'), // Final Pe
        // Bottom row (z x c v b n m , . /)
        ('ז', 'z'), // Zayin
        ('ס', 'x'), // Samekh
        ('ב', 'c'), // Bet
        ('ה', 'v'), // He
        ('נ', 'b'), // Nun
        ('מ', 'n'), // Mem
        ('צ', 'm'), // Tsadi
        ('ת', ','), // Tav
        ('ץ', '.'), // Final Tsadi
        ('.', '/'), // Period maps to /
    ];

    pairs.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_variants_hebrew_to_english() {
        // 'ביר' in Hebrew maps to 'chr' in English (on QWERTY keyboard)
        // ב -> c, י -> h, ר -> r
        let variants = layout_variants("ביר");
        assert!(
            variants.contains(&"ביר".to_string()),
            "should contain original"
        );
        assert!(
            variants.contains(&"chr".to_string()),
            "should contain English variant 'chr'"
        );
    }

    #[test]
    fn test_layout_variants_english_unchanged() {
        // English text should remain unchanged
        let variants = layout_variants("chrome");
        assert_eq!(variants.len(), 1);
        assert_eq!(variants[0], "chrome");
    }

    #[test]
    fn test_layout_variants_empty() {
        let variants = layout_variants("");
        assert_eq!(variants.len(), 1);
        assert_eq!(variants[0], "");
    }

    #[test]
    fn test_layout_variants_mixed() {
        // Mixed Hebrew and English should partially translate
        let variants = layout_variants("aבר");
        assert!(
            variants.contains(&"aבר".to_string()),
            "should contain original"
        );
        // ב -> c, ר -> r
        assert!(
            variants.contains(&"acr".to_string()),
            "should contain partial translation"
        );
    }

    #[test]
    fn test_hebrew_to_english_complete_word() {
        // Test 'שלום' (shalom) - ש=a, ל=k, ו=u, ם=o
        let variants = layout_variants("שלום");
        assert!(variants.contains(&"שלום".to_string()));
        assert!(variants.contains(&"akuo".to_string()));
    }

    #[test]
    fn test_deduplication() {
        // If the variant is the same as original, don't add duplicate
        let variants = layout_variants("abc");
        assert_eq!(variants.len(), 1);
    }
}
