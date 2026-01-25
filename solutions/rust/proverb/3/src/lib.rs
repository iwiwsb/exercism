pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    list.windows(2).fold(String::new(), proverb_constructor)
        + &format!("And all for the want of a {}.", list[0])
}

fn proverb_constructor(acc: String, next: &[&str]) -> String {
    acc + &format!("For want of a {} the {} was lost.\n", next[0], next[1])
}
