fn solution(s: String) -> bool {
    let s = s.to_lowercase();
    let (mut i, mut j) = (0 as usize, s.len() - 1);
    let chars = s.chars().collect::<Vec<char>>();
    while i < j {
        while i < s.len()
            && !chars
                .get(i)
                .expect("i should be inbound: {}")
                .is_alphanumeric()
        {
            i += 1;
        }
        while j > 0 && !chars.get(j).expect("j should be inbound").is_alphanumeric() {
            j -= 1;
        }
        if i >= j {
            return true;
        }

        if chars.get(i).unwrap() != chars.get(j).unwrap() {
            return false;
        }

        i += 1;
        j -= 1;
    }
    true
}
