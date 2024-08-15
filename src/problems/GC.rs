//Computing GC Content

pub fn computing_gc_content(file: String) -> (String, f32) {
    let mut results: Vec<(String, f32)> = Vec::new();
    let cases: Vec<&str> = file.trim().split(">").collect();


    for case in &cases[1..cases.len()] {
        //print!("\n\n{}",case);
        let mut count: i32 = 0;
        let id: &str = &case[..13];
        let sequence: String = (&case[15..]).to_string();

        //sequence.chars().filter(|c| c.is_alphabetic()).count()  *No idea why this works, but it does (Copilot help)
        let sequence_size: i32 = sequence.chars().filter(|c| c.is_alphabetic()).count().try_into().unwrap();

        for c in sequence.chars() {
            match c {
                'C'|'G' => count += 1,
                _ => (),
            }
        }
        let percentage: f32 = count as f32 / sequence_size as f32;
        results.push((id.to_string(), percentage*100.0));
        //println!(" ID:{}    Count:{},    Total:{},    Percentage:{}", id, count, sequence_size, percentage);
    }

    //Return maximum percentage and corresponding ID (returns a tuple)
    results.into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or((String::new(), 0.0))
}