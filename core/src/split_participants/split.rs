/*
Because the search for Hamiltonian cycles is a NP-complete problem,
for graphs over 10 participants, it is wise to avoid trying to look for complete graphs,
but instead divide the graph into N/4 chunks of 4 participants (if N % 4 = 0).

When N % 4 is not zero, we want the last chunk, named the complement, to hold
not 1, 2 or 3 participants, but rather 5, 6 or 7.
 */

const CHUNK_SIZE: usize = 4;

/// Calculate the number of participants to put in each subgraph
fn calculate_split_number_of_participants(n: usize) -> (usize, usize) {
    let num_quatuors = (n - CHUNK_SIZE) / CHUNK_SIZE;
    let size_complement = (n - CHUNK_SIZE) % CHUNK_SIZE + CHUNK_SIZE;

    (num_quatuors, size_complement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_chunks() {
        assert_eq!((1, 7), calculate_split_number_of_participants(11));
        assert_eq!((1, 4), calculate_split_number_of_participants(8));
    }
}
