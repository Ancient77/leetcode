--
-- @lc app=leetcode id=197 lang=mysql
--
-- [197] Rising Temperature
--

-- @lc code=start
# Write your MySQL query statement below
SELECT weather.id AS "Id"
FROM 
    weather
      JOIN
    weather w ON DATEDIFF(weather.recordDate, w.recordDate) = 1
      AND weather.Temperature > w.Temperature
-- @lc code=end

