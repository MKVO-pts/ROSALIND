use std::{borrow::Borrow, collections::HashMap};

//Translating RNA into Protein
pub fn translating_rna_to_rotein(rna: String) -> String {
    
    let mut solution:String = String::new();
    let mut rna_codon_table: HashMap<_, _> = HashMap::new();

    rna_codon_table.insert("UUU","F");
    rna_codon_table.insert("UUC","F");
    rna_codon_table.insert("UUA","L");
    rna_codon_table.insert("UUG","L");
    rna_codon_table.insert("UCU","S");
    rna_codon_table.insert("UCC","S");
    rna_codon_table.insert("UCA","S");
    rna_codon_table.insert("UCG","S");
    rna_codon_table.insert("UAU","Y");
    rna_codon_table.insert("UAC","Y");
    rna_codon_table.insert("UAA","    ");
    rna_codon_table.insert("UAG","    ");
    rna_codon_table.insert("UGU","C");
    rna_codon_table.insert("UGC","C");
    rna_codon_table.insert("UGA","    ");
    rna_codon_table.insert("UGG","W");

    rna_codon_table.insert("CUU","L");
    rna_codon_table.insert("CUC","L");
    rna_codon_table.insert("CUA","L");
    rna_codon_table.insert("CUG","L");
    rna_codon_table.insert("CCU","P");
    rna_codon_table.insert("CCC","P");
    rna_codon_table.insert("CCA","P");
    rna_codon_table.insert("CCG","P");
    rna_codon_table.insert("CAU","H");
    rna_codon_table.insert("CAC","H");
    rna_codon_table.insert("CAA","Q");
    rna_codon_table.insert("CAG","Q");
    rna_codon_table.insert("CGU","R");
    rna_codon_table.insert("CGC","R");
    rna_codon_table.insert("CGA","R");
    rna_codon_table.insert("CGG","R");

    rna_codon_table.insert("AUU","I");
    rna_codon_table.insert("AUC","I");
    rna_codon_table.insert("AUA","I");
    rna_codon_table.insert("AUG","M");
    rna_codon_table.insert("ACU","T");
    rna_codon_table.insert("ACC","T");
    rna_codon_table.insert("ACA","T");
    rna_codon_table.insert("ACG","T");
    rna_codon_table.insert("AAU","N");
    rna_codon_table.insert("AAC","N");
    rna_codon_table.insert("AAA","K");
    rna_codon_table.insert("AAG","K");
    rna_codon_table.insert("AGU","S");
    rna_codon_table.insert("AGC","S");
    rna_codon_table.insert("AGA","R");
    rna_codon_table.insert("AGG","R");

    rna_codon_table.insert("GUU","V");
    rna_codon_table.insert("GUC","V");
    rna_codon_table.insert("GUA","V");
    rna_codon_table.insert("GUG","V");
    rna_codon_table.insert("GCU","A");
    rna_codon_table.insert("GCC","A");
    rna_codon_table.insert("GCA","A");
    rna_codon_table.insert("GCG","A");
    rna_codon_table.insert("GAU","D");
    rna_codon_table.insert("GAC","D");
    rna_codon_table.insert("GAA","E");
    rna_codon_table.insert("GAG","E");
    rna_codon_table.insert("GGU","G");
    rna_codon_table.insert("GGC","G");
    rna_codon_table.insert("GGA","G");
    rna_codon_table.insert("GGG","G");

    let mut codons_list: Vec<String> = rna.chars() // iterate over the characters
        .collect::<Vec<_>>() //  collect the characters into a Vec<char>
        .chunks(3) //split the characters into chunks of 3
        .map(|chunk| chunk.iter().collect()) //convert chunk of characters into String
        .collect(); //collect all substrings


    //Unexplained, 
    let mut index = 0;
    while index < codons_list.len() {
        if let Some(codon) = codons_list.get(index).map(|s| s.to_string()) {
            if let Some(value) = rna_codon_table.get(&codon.borrow()) {
                solution.push_str(value);
                codons_list.remove(index);
            } else {
                index += 1;
            }
        }
    }

    solution // return final answer
    
}

