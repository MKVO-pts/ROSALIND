//Complementing a Strand of DNA
pub fn complementing_a_strand_of_dna(dna:String) -> String{
    //Not good (Needs to create a new Variable, but it only reads the string one time)
    let mut solution: String = String::new();
    for char in dna.chars() {
        match char {
            'A' => solution.insert_str(0, "T"),  
            'C' => solution.insert_str(0, "G"),
            'T' => solution.insert_str(0, "A"),
            'G' => solution.insert_str(0, "C"),
            _ => () //Error/Other case => do nothing
        }
    }
    solution
/*pub fn complementing_a_strand_of_dna(dna:String) -> String {
    let mut rna = String::new();
    for c in dna.chars() {
        match c {
            'A' => rna.push('U'),
            'C' => rna.push('G'),
            'G' => rna.push('C'),
            'T' => rna.push('A'),
            _ => (),
        }
    }
    rna
}*/}

