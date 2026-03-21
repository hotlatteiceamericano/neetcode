pub struct EncodeDecode {}
impl EncodeDecode {
    fn encode(strs: Vec<String>) -> String {
        let mut encode_list = Vec::new();
        for str in strs {
            encode_list.push(format!("{:0>3}", str.len().to_string()));
            encode_list.push(str);
        }
        encode_list.join("")
    }

    fn decode(str: String) -> Vec<String> {
        let mut i = 0 as usize;
        let mut decode = Vec::new();
        while i < str.len() {
            let length = str[i..i + 3]
                .parse::<usize>()
                .expect("header is not integer");
            decode.push(str[i + 3..i + 3 + length].to_string());
            i += 3 + length;
        }
        println!("{}", str.len());
        decode
    }
}

