--
-- @lc app=leetcode id=595 lang=mysql
--
-- [595] Big Countries
--

-- @lc code=start
# Write your MySQL query statement below
SELECT name, population, area 
FROM WORLD 
WHERE area >= 3000000 OR population >= 25000000
-- @lc code=end

