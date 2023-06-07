--
-- @lc app=leetcode id=1280 lang=mysql
--
-- [1280] Students and Examinations
--

-- @lc code=start
# Write your MySQL query statement below
SELECT st.student_id, st.student_name,su.subject_name,COUNT(e.subject_name) as attended_exams
FROM Students st 
JOIN Subjects su
LEFT JOIN Examinations as e
ON st.student_id=e.student_id AND su.subject_name=e.subject_name
GROUP BY st.student_id,su.subject_name
ORDER BY student_id,subject_name
-- @lc code=end

