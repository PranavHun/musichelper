pub fn get_highlight(notes: &Vec<String>, key: &String, formula: &Vec<u64>) -> Vec<String> {
    let mut local_notes = notes.clone();
    let mut highlight: Vec<String> = Vec::new();
    let idx = notes.iter().position(|x| x == key).unwrap_or(0);
    if idx > 0 {
        local_notes.rotate_left(idx);
    }

    for i in formula {
        highlight.push(local_notes[(*i as usize) % notes.len()].clone());
    }
    highlight
}
