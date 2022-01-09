pub struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        // 1970/12/31是周四
        let mut days_elapsed = 3;
        let days_of_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let weekday_desc = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        for i in 1971..year {
            days_elapsed += if Self::is_leap_year(i) { 366 } else { 365 };
        }
        days_elapsed += days_of_month.iter().take(month as usize - 1).sum::<i32>();
        if Self::is_leap_year(year) && month > 2 {
            days_elapsed += 1;
        }
        days_elapsed += day;
        weekday_desc[(days_elapsed % 7) as usize].to_string()
    }

    pub fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && year % 100 != 0 || year % 400 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_of_the_week_works() {
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday")
    }
}
