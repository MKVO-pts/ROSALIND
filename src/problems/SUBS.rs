// Finding a Motif in DNA
pub fn find_motif_in_dna(dna: &str, motif: &str) -> Vec<usize> {

    let mut positions = Vec::new();
    let motif_lenght = motif.len();
    let dna_lenght = dna.len();



    for base in 0..= dna_lenght-motif_lenght {
        print!("{}  ",&dna[base..motif_lenght + base]);
        print!("   {}\n  ", motif);
        
        if motif == &dna[base..motif_lenght + base] {
            positions.push(base+1); //beause the problem uses 1-based numbering instead of 0-base
        }
    }
    positions //returns the vector with the positons
}
