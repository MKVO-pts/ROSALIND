
//Count Nucleotides in a DNA string
pub fn count_dna_nucleotides(dna: &str) -> (i64, i64, i64, i64) {
    let mut count = (0, 0, 0, 0); //tuple for A, C, G, T

    for c in dna.chars() {
        match c {
            'A' => count.0 += 1,
            'C' => count.1 += 1,
            'G' => count.2 += 1,
            'T' => count.3 += 1,
            _ => (),
        }
    }
    count //return the tuple (A,C,G,T)
}