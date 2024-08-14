//Transcribing DNA to RNA
pub fn transcribing_dna_to_rna(dna:String) -> String {
    let rna: String = dna.replace("T", "U");
    rna //return rna with U instead of T
    
    //another way
    /* does not create a new string, ?speed test?
    dna = dna.chars().map(|c| match c {
        'T' => 'U',
        _ => c,
    }).collect();*/
}