use std::collections::HashMap;

pub fn solution(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::<[i32; 26], Vec<String>>::new();
    for s in strs {
        let mut occur = [0; 26];
        let cloned_s = s.clone();
        for c in s.chars() {
            let index = c as u8 - 'a' as u8;
            occur[index as usize] += 1;
        }
        map.entry(occur)
            .and_modify(|v| v.push(s))
            .or_insert(vec![cloned_s]);
    }
    map.into_values().collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("{}", b'a');
    }
}
