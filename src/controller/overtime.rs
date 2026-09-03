use chrono::{Duration, Local};
use crate::data::activity;
use crate::view::list::group_activities_by_date;
use crate::view::report::sum_duration;

/// Test overtime in report using
///
/// `./target/debug/bartib.exe -f C:\Users\linus\bartib\misc\TEST_TIMETRACKING_FILE report -o`

// TODO replace with system variable
// Also I am too lazy to do this nicely, this is 39.5h * 0.8 / 5 in seconds
const BARTIB_NOMINAL_WORKDAY: Duration  = Duration::seconds(12 + 19 * 60 + 6 * 60 * 60);


pub fn add_transfer_overtime() {}

pub fn subtract_transfer_overtime() {}

pub fn gleitzeit() {}

pub fn get_total_overtime(activities: &[&activity::Activity]) -> Duration {
    // Get activities grouped by date
    let grouped_activities = group_activities_by_date(activities);

    // Sum up total overtime
    let mut total_overtime = Duration::seconds(0);
    for daily_activities in grouped_activities {
        if daily_activities.0 == Local::now().naive_local().date() {
            continue
        }
        total_overtime += get_overtime_from_activity_vec(daily_activities.1)
    }

    // TODO include "Übertrag"

    total_overtime
}

pub fn get_overtime_from_activity_vec(activities: Vec<&activity::Activity>) -> Duration {
    // Sum up duration
    let daily_hours = sum_duration(&activities);

    // Subtract BARTIB_NOMINAL_WORKDAY
    daily_hours - BARTIB_NOMINAL_WORKDAY
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nominal_workday() {
        assert_eq!(BARTIB_NOMINAL_WORKDAY.num_hours(), 6);
        assert_eq!(BARTIB_NOMINAL_WORKDAY.num_minutes(), 6 * 60 + 19);
        assert_eq!(BARTIB_NOMINAL_WORKDAY.num_seconds(), 12 + 19 * 60 + 6 * 60 * 60);
    }
}