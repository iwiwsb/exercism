#[cfg(feature = "grapheme")]
use unicode_reverse::reverse_grapheme_clusters_in_place;

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    let mut grapheme_cluster = input.to_string();
    reverse_grapheme_clusters_in_place(&mut grapheme_cluster);
    grapheme_cluster.to_string()
}

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
