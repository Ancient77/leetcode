/*
 * @lc app=leetcode id=1309 lang=rust
 *
 * [1309] Decrypt String from Alphabet to Integer Mapping
 */

// @lc code=start
impl Solution {
    pub fn freq_alphabets(mut s: String) -> String {
            let mut buffer = vec![];
            let mut res = vec![];
            for n in s.chars(){
                if(n != '#'){
                    buffer.push(n);
                }else {
                    for i in 0..buffer.len()-2 {
                        res.push((buffer[i].to_digit(10).unwrap() as u8 + 96) as char);
                    }
                    let y = buffer[buffer.len() - 2].to_digit(10).unwrap()*10 + buffer[buffer.len() - 1].to_digit(10).unwrap(); 
                    println!("{}",(y as u8 + 96) as char);
                    res.push((y as u8 + 96) as char);
                    buffer.clear();
                }
            }
            if(!buffer.is_empty()){
                for y in buffer{
                    res.push((y.to_digit(10).unwrap() as u8 + 96) as char);
                }
            }

            res.into_iter().collect()
    }
}
// @lc code=end

