pub fn is_valid(s: String) -> bool {
    // whenever seeing a open one: (, [, {, insert its opposing one
    // whenever seeing a clones one, pop from the stack and compare the two
    // if match, keep going
    // if not match, return false
    // if stack has something remaining at the end of iteration, return false.
    let mut vec = Vec::new();
    for c in s.chars() {
        if c == '(' {
            vec.push(')');
        } else if c == '[' {
            vec.push(']');
        } else if c == '{' {
            vec.push('}');
        }

        if c == ')' || c == ']' || c == '}' {
            match vec.pop() {
                None => {
                    return false;
                }
                Some(v) if v != c => {
                    return false;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    vec.is_empty()
}
