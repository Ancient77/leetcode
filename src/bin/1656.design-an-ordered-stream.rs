/*
 * @lc app=leetcode id=1656 lang=rust
 *
 * [1656] Design an Ordered Stream
 */

// @lc code=start
struct OrderedStream {
    waitingForIndex: i32,
    savedState: Vec<Option<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            waitingForIndex: 0,
            savedState: vec![None; n as usize],
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.savedState[id_key as usize - 1] = Some(value);
        let mut resVec = Vec::new();
        while (self.waitingForIndex as usize)< self.savedState.len() {
            match &self.savedState[self.waitingForIndex as usize] {
                Some(x) => {
                    resVec.push(x.clone());
                    self.waitingForIndex += 1;
                }
                None => break,
            }
        }
        resVec
    }
}

// @lc code=end
