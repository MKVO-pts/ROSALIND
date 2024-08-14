//Counting Point Mutations

pub fn counting_point_mutations(sequence1:String, sequence2:String) -> i32 {
    let mut hamming_distance: i32 = 0;
    let seq1:String = String::from(sequence1);
    let seq2:String = String::from(sequence2);
    let size:i32 = seq1.len().try_into().unwrap();


    for i in 0..size {
        if seq1.chars().nth(i.try_into().unwrap()).unwrap() != seq2.chars().nth(i.try_into().unwrap()).unwrap() { //loop though both sequences at the same time
            hamming_distance += 1;
        }else { //they are the same
            continue;
        };
    }
    hamming_distance //return number point mutations
}
