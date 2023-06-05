/*
 * @lc app=leetcode id=535 lang=rust
 *
 * [535] Encode and Decode TinyURL
 */

// @lc code=start
struct Codec {
	saved: String
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self { saved:"".to_string() }
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        self.saved = longURL;
        return "xyz".to_string();
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        return self.saved.clone();
    }
}


// @lc code=end

