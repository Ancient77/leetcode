/*
 * @lc app=leetcode id=649 lang=rust
 *
 * [649] Dota2 Senate
 */

// @lc code=start
impl Solution {
    pub fn predict_party_victory(mut senate: String) -> String {
        let mut rDisable = 0;
        let mut dDisable = 0;
        
        let mut temp:Vec<char> = senate.chars().collect();
        let mut copy = temp.clone();

        loop {
            
            
            if(!copy.contains(&'D')){
                return "Radiant".to_owned();
            }
            if(!copy.contains(&'R')){
                return "Dire".to_owned();
            }
            temp = copy.clone();
            copy.clear();
            for s in 0..temp.len(){
                match temp[s] {
                    'D' => {
                        if(rDisable == 0){
                            copy.push(temp[s]);
                            dDisable+=1;
                        }else{
                            rDisable-=1;
                        }
                    },
                    'R' => {
                        if(dDisable == 0){
                            copy.push(temp[s]);
                            rDisable+=1;
                        }else{
                            dDisable-=1;
                        }
                    },
                    _ => ()
                }
        }
        

        }
        
    }
}
// @lc code=end

