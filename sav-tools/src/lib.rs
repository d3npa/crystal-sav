pub mod gen2;
pub mod errors;

/// find all offsets in `data` that match `target`
/// indexes in `target` can be skipped to account for gaps
/// note: index 0 should not be skipped! it will result in skiping.. actually skiping like this always happens - fix the code so when it stops a match chain it rechecks for a new chain with the first byte before continuing
pub fn search_bytes(data: &[u8], target: &[u8], skip: &[usize]) -> Vec<usize> {
    let mut matched_index = 0;
    let mut matched_offsets = vec![];

    for offset in 0..data.len() {        
        if matched_index == target.len() {
            matched_offsets.push(offset - target.len());
            matched_index = 0;
        }

        if skip.contains(&matched_index) {
            matched_index += 1;
        } else if data[offset] == target[matched_index] {
            matched_index += 1;
        } else if data[offset] == target[0] { 
            // restart search at this offset before continuing to next offset
            matched_index = 1;
        } else {
            matched_index = 0;
        }
    }

    matched_offsets
}