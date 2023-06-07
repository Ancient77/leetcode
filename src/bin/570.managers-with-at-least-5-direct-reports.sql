--
-- @lc app=leetcode id=570 lang=mysql
--
-- [570] Managers with at Least 5 Direct Reports
--

-- @lc code=start
# Write your MySQL query statement below
SELECT distinct m.name as name
FROM Employee e JOIN employee m
on m.id = e.managerId
group by m.id
HAVING COUNT(*) >= 5
-- @lc code=end

