//Calculating Protein Mass
use std::collections::HashMap;

pub fn calculating_protein_mass(protein:String) -> f64 {
    let mut solution: f64 = 0.0;
    let mut monoisotopic_mass_table:HashMap<char,f64> = HashMap::new();
    
    //Monoisotopic Mass Table
    monoisotopic_mass_table.insert('A', 71.03711);
    monoisotopic_mass_table.insert('C', 103.00919);
    monoisotopic_mass_table.insert('D', 115.02694);
    monoisotopic_mass_table.insert('E', 129.04259);
    monoisotopic_mass_table.insert('F', 147.06841);
    monoisotopic_mass_table.insert('G', 57.02146);
    monoisotopic_mass_table.insert('H', 137.05891);
    monoisotopic_mass_table.insert('I', 113.08406);
    monoisotopic_mass_table.insert('K', 128.09496);
    monoisotopic_mass_table.insert('L', 113.08406);
    monoisotopic_mass_table.insert('M', 131.04049);
    monoisotopic_mass_table.insert('N', 114.04293);
    monoisotopic_mass_table.insert('P', 97.05276);
    monoisotopic_mass_table.insert('Q', 128.05858);
    monoisotopic_mass_table.insert('R', 156.10111);
    monoisotopic_mass_table.insert('S', 87.03203);
    monoisotopic_mass_table.insert('T', 101.04768);
    monoisotopic_mass_table.insert('V', 99.06841);
    monoisotopic_mass_table.insert('W', 186.07931);
    monoisotopic_mass_table.insert('Y', 163.06333 );

    for char in protein.chars() {   
        match monoisotopic_mass_table.get(&char) { //get the value for the monoisotops
            Some(value) => solution += value, //sum the mass value
            None => continue,            
        }
    }
    solution //return the sum 
}