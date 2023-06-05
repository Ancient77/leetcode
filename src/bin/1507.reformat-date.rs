/*
 * @lc app=leetcode id=1507 lang=rust
 *
 * [1507] Reformat Date
 */

// @lc code=start



impl Solution {
    pub fn reformat_date(date: String) -> String {
        let mut stringDate:Vec<&str> = date.split(' ').collect();
        let mut resDate = String::new();
        resDate.push_str(stringDate[2]);
        resDate.push('-');
        match stringDate[1] {
            "Jan" => resDate.push_str("01"),
            "Feb" => resDate.push_str("02"),
            "Mar" => resDate.push_str("03"),
            "Apr" => resDate.push_str("04"),
            "May" => resDate.push_str("05"),
            "Jun" => resDate.push_str("06"),
            "Jul" => resDate.push_str("07"),
            "Aug" => resDate.push_str("08"),
            "Sep" => resDate.push_str("09"),
            "Oct" => resDate.push_str("10"),
            "Nov" => resDate.push_str("11"),
            "Dec" => resDate.push_str("12"),
            _ => {}
        }
        resDate.push('-');
        let mut chars = stringDate[0].chars();
        chars.next_back();
        chars.next_back();
        let mut charStr = chars.as_str();
        if(charStr.len() == 1){
            let formatted = format!("0{}", charStr);
            charStr = formatted.as_str();
            resDate.push_str(charStr);
        }else{
            resDate.push_str(charStr);
        }
        
        resDate
    }
}
// @lc code=end

