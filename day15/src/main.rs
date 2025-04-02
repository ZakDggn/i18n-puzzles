use jiff::{
    civil::{Date, Time},
    tz::TimeZone,
};
use std::{fs, str::FromStr};

const START_TIME: Time = Time::constant(8, 30, 0, 0);
const END_TIME: Time = Time::constant(17, 0, 0, 0);

#[derive(Debug)]
struct Location {
    timezone: TimeZone,
    holidays: Vec<Date>,
}

#[derive(Debug, PartialEq, Eq)]
struct Interval {
    start: Time,
    end: Time,
}

impl Interval {
    fn duration(&self) -> u64 {
        self.start
            .duration_until(self.end)
            .round(jiff::Unit::Minute)
            .unwrap()
            .as_mins()
            .try_into()
            .unwrap()
    }
}

impl FromStr for Location {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split('\t');
        if fields.next().is_none() {
            return Err("failed to parse name");
        };
        let timezone = match fields.next() {
            Some(timezone) => match TimeZone::get(timezone) {
                Ok(timezone) => timezone,
                Err(_) => return Err("invalid timezone"),
            },
            None => return Err("failed to parse timezone"),
        };
        let Some(holidays) = fields.next() else {
            return Err("failed to parse holidays");
        };
        let Ok(holidays): Result<Vec<Date>, _> = holidays
            .split(';')
            .map(|date| Date::strptime("%d %B %Y", date))
            .collect()
        else {
            return Err("invalid date in holidays");
        };
        Ok(Self { timezone, holidays })
    }
}

fn parse(input: &str) -> (Vec<Location>, Vec<Location>) {
    let (offices, customers) = input.split_once("\n\n").unwrap();
    (
        offices
            .lines()
            .map(|location| location.parse().unwrap())
            .collect(),
        customers
            .lines()
            .map(|location| location.parse().unwrap())
            .collect(),
    )
}

fn is_work_day(date: Date, location: &Location) -> bool {
    !location.holidays.contains(&date) && (1..=5).contains(&date.weekday().to_monday_one_offset())
}

/// returns the time intervals worked between 00:00 and 24:00 UTC on `date` at `location`
fn location_work_intervals(date: Date, location: &Location) -> Vec<Interval> {
    let mut intervals = Vec::new();
    let utc_day_start = date.to_datetime(Time::MIN).in_tz("UTC").unwrap();
    let utc_day_end = date.to_datetime(Time::MAX).in_tz("UTC").unwrap();
    let local_today_start = date
        .to_datetime(START_TIME)
        .to_zoned(location.timezone.clone())
        .unwrap();
    let local_today_end = date
        .to_datetime(END_TIME)
        .to_zoned(location.timezone.clone())
        .unwrap();
    let yesterday = date.yesterday().unwrap();
    let local_yesterday_end = yesterday
        .to_datetime(END_TIME)
        .to_zoned(location.timezone.clone())
        .unwrap();
    // large negative UTC offset
    if local_yesterday_end > utc_day_start {
        if is_work_day(yesterday, location) {
            intervals.push(Interval {
                start: utc_day_start.time(),
                end: local_yesterday_end.in_tz("UTC").unwrap().time(),
            });
        }
        if is_work_day(date, location) {
            intervals.push(Interval {
                start: local_today_start.in_tz("UTC").unwrap().time(),
                end: utc_day_end.time(),
            });
        }
    }
    // large positive UTC offset
    else if local_today_start < utc_day_start {
        if is_work_day(date, location) {
            intervals.push(Interval {
                start: utc_day_start.time(),
                end: local_today_end.in_tz("UTC").unwrap().time(),
            });
        }
        let tomorrow = date.tomorrow().unwrap();
        if is_work_day(tomorrow, location) {
            let local_tomorrow_start = tomorrow
                .to_datetime(START_TIME)
                .to_zoned(location.timezone.clone())
                .unwrap();
            intervals.push(Interval {
                start: local_tomorrow_start.in_tz("UTC").unwrap().time(),
                end: utc_day_end.time(),
            });
        }
    // small UTC offset
    } else if is_work_day(date, location) {
        let end = if local_today_end < utc_day_end {
            local_today_end.in_tz("UTC").unwrap()
        } else {
            utc_day_end
        };
        intervals.push(Interval {
            start: local_today_start.in_tz("UTC").unwrap().time(),
            end: end.time(),
        });
    }
    intervals
}

/// add `other` interval into sorted `intervals` and join any overlaps
fn add_interval(intervals: &mut Vec<Interval>, other: Interval) {
    let Some(i) = intervals
        .iter()
        .position(|interval| other.start <= interval.end)
    else {
        intervals.push(other);
        return;
    };
    intervals.insert(i, other);
    while i != intervals.len() - 1 && intervals[i].end >= intervals[i + 1].start {
        let start = std::cmp::min(intervals[i].start, intervals[i + 1].start);
        let end = std::cmp::max(intervals[i].end, intervals[i + 1].end);
        intervals[i] = Interval { start, end };
        intervals.remove(i + 1);
    }
}

/// returns the time intervals between 00:00 and 24:00 UTC on `date` when at least one office is
/// working
fn combined_work_intervals(date: Date, offices: &[Location]) -> Vec<Interval> {
    let mut intervals = Vec::new();
    for office in offices {
        for interval in location_work_intervals(date, office) {
            add_interval(&mut intervals, interval);
        }
    }
    intervals
}

fn invert_intervals(intervals: &[Interval]) -> Vec<Interval> {
    if intervals.is_empty() {
        return vec![Interval {
            start: Time::MIN,
            end: Time::MAX,
        }];
    }
    let mut inverted = Vec::new();
    if intervals[0].start > Time::MIN {
        inverted.push(Interval {
            start: Time::MIN,
            end: intervals[0].start,
        });
    }
    for pair in intervals.windows(2) {
        inverted.push(Interval {
            start: pair[0].end,
            end: pair[1].start,
        });
    }
    if intervals.last().unwrap().end < Time::MAX {
        inverted.push(Interval {
            start: intervals.last().unwrap().end,
            end: Time::MAX,
        });
    }
    inverted
}

fn find_overlaps(xs: &[Interval], ys: &[Interval]) -> Vec<Interval> {
    let mut overlaps = Vec::new();
    let mut ix = 0;
    let mut iy = 0;
    while ix < xs.len() && iy < ys.len() {
        let start = std::cmp::max(xs[ix].start, ys[iy].start);
        let end = std::cmp::min(xs[ix].end, ys[iy].end);
        if end > start {
            overlaps.push(Interval { start, end });
        }
        if xs[ix].end < ys[iy].end {
            ix += 1;
        } else {
            iy += 1;
        }
    }
    overlaps
}

fn customer_support_intervals(date: Date, customer: &Location) -> Vec<Interval> {
    let utc_day_start = date.to_datetime(Time::MIN).in_tz("UTC").unwrap();
    let utc_day_end = date.to_datetime(Time::MAX).in_tz("UTC").unwrap();
    let local_today_start = date
        .to_datetime(Time::MIN)
        .to_zoned(customer.timezone.clone())
        .unwrap();
    let local_today_end = date
        .to_datetime(Time::MAX)
        .to_zoned(customer.timezone.clone())
        .unwrap();
    let yesterday = date.yesterday().unwrap();
    let local_yesterday_end = yesterday
        .to_datetime(Time::MAX)
        .to_zoned(customer.timezone.clone())
        .unwrap();
    let tomorrow = date.tomorrow().unwrap();
    let local_tomorrow_start = tomorrow
        .to_datetime(Time::MIN)
        .to_zoned(customer.timezone.clone())
        .unwrap();
    let mut intervals = Vec::new();
    if is_work_day(yesterday, customer) && local_yesterday_end > utc_day_start {
        intervals.push(Interval {
            start: utc_day_start.time(),
            end: local_yesterday_end.in_tz("UTC").unwrap().time(),
        });
    }
    if is_work_day(date, customer) && local_today_start < utc_day_end {
        let start = if local_today_start > utc_day_start {
            local_today_start.in_tz("UTC").unwrap()
        } else {
            utc_day_start
        };
        let end = if local_today_end < utc_day_end {
            local_today_end.in_tz("UTC").unwrap()
        } else {
            utc_day_end.clone()
        };
        intervals.push(Interval {
            start: start.time(),
            end: end.time(),
        });
    }
    if is_work_day(tomorrow, customer) && local_tomorrow_start < utc_day_end {
        intervals.push(Interval {
            start: local_tomorrow_start.in_tz("UTC").unwrap().time(),
            end: utc_day_end.time(),
        });
    }
    intervals
}

fn overtime_minutes(date: Date, work_gaps: &[Interval], customer: &Location) -> u64 {
    let customer_intervals = customer_support_intervals(date, customer);
    let overtime_intervals = find_overlaps(work_gaps, &customer_intervals);
    overtime_intervals.iter().map(Interval::duration).sum()
}

fn solution(input: &str) -> u64 {
    let (offices, customers) = parse(input);
    let mut customer_overtimes = vec![0; customers.len()];
    let mut date = Date::new(2022, 1, 1).unwrap();
    for _ in 0..365 {
        let work_intervals = combined_work_intervals(date, &offices);
        let work_gaps = invert_intervals(&work_intervals);
        for (i, customer) in customers.iter().enumerate() {
            let overtime = overtime_minutes(date, &work_gaps, customer);
            customer_overtimes[i] += overtime;
        }
        date = date.tomorrow().unwrap();
    }

    customer_overtimes.iter().max().unwrap() - customer_overtimes.iter().min().unwrap()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = solution(&input);
    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_interval((start, end): &(i8, i8)) -> Interval {
        Interval {
            start: Time::new(*start, 0, 0, 0).unwrap(),
            end: if *end < 24 {
                Time::new(*end, 0, 0, 0).unwrap()
            } else {
                Time::MAX
            },
        }
    }

    fn assert_intervals_become(intervals: &[(i8, i8)], other: (i8, i8), expected: &[(i8, i8)]) {
        let mut intervals: Vec<_> = intervals.iter().map(create_interval).collect();
        let other = create_interval(&other);
        let expected: Vec<_> = expected.iter().map(create_interval).collect();
        add_interval(&mut intervals, other);
        assert_eq!(intervals, expected);
    }

    #[test]
    fn test_add_interval() {
        let intervals = &[(3, 5), (9, 15), (16, 20)];
        assert_intervals_become(intervals, (1, 2), &[(1, 2), (3, 5), (9, 15), (16, 20)]);
        assert_intervals_become(intervals, (6, 8), &[(3, 5), (6, 8), (9, 15), (16, 20)]);
        assert_intervals_become(intervals, (22, 23), &[(3, 5), (9, 15), (16, 20), (22, 23)]);
        assert_intervals_become(intervals, (1, 4), &[(1, 5), (9, 15), (16, 20)]);
        assert_intervals_become(intervals, (20, 22), &[(3, 5), (9, 15), (16, 22)]);
        assert_intervals_become(intervals, (14, 18), &[(3, 5), (9, 20)]);
        assert_intervals_become(intervals, (1, 23), &[(1, 23)]);
    }

    #[test]
    fn test_invert_intervals() {
        let intervals: Vec<_> = [(1, 3), (12, 17), (20, 22)]
            .iter()
            .map(create_interval)
            .collect();
        let inverted: Vec<_> = [(0, 1), (3, 12), (17, 20), (22, 24)]
            .iter()
            .map(create_interval)
            .collect();
        assert_eq!(invert_intervals(&intervals), inverted);
    }

    #[test]
    fn test_find_overlaps() {
        let xs: Vec<_> = [(2, 19), (20, 21)].iter().map(create_interval).collect();
        let ys: Vec<_> = [(3, 4), (7, 10), (18, 23)]
            .iter()
            .map(create_interval)
            .collect();
        let expected: Vec<_> = [(3, 4), (7, 10), (18, 19), (20, 21)]
            .iter()
            .map(create_interval)
            .collect();
        assert_eq!(find_overlaps(&xs, &ys), expected);

        let xs: Vec<_> = [(1, 5), (11, 16), (19, 21)]
            .iter()
            .map(create_interval)
            .collect();
        let ys: Vec<_> = [(2, 4), (9, 17), (20, 23)]
            .iter()
            .map(create_interval)
            .collect();
        let expected: Vec<_> = [(2, 4), (11, 16), (20, 21)]
            .iter()
            .map(create_interval)
            .collect();
        assert_eq!(find_overlaps(&xs, &ys), expected);
    }

    #[test]
    fn test_interval_duration() {
        let intervals = [
            Interval {
                start: Time::new(3, 0, 0, 0).unwrap(),
                end: Time::new(7, 30, 0, 0).unwrap(),
            },
            Interval {
                start: Time::new(8, 0, 0, 0).unwrap(),
                end: Time::new(9, 45, 0, 0).unwrap(),
            },
            Interval {
                start: Time::new(16, 0, 0, 0).unwrap(),
                end: Time::new(19, 0, 0, 0).unwrap(),
            },
            Interval {
                start: Time::new(20, 0, 0, 0).unwrap(),
                end: Time::new(22, 15, 0, 0).unwrap(),
            },
        ];
        assert_eq!(intervals.iter().map(Interval::duration).sum::<u64>(), 690);
    }

    #[test]
    fn test_overtime_minutes() {
        let date = Date::new(2022, 1, 4).unwrap();
        let customer = Location {
            timezone: TimeZone::get("UTC").unwrap(),
            holidays: vec![],
        };
        let work_gaps = vec![
            Interval {
                start: Time::new(3, 0, 0, 0).unwrap(),
                end: Time::new(7, 30, 0, 0).unwrap(),
            },
            Interval {
                start: Time::new(8, 0, 0, 0).unwrap(),
                end: Time::new(9, 45, 0, 0).unwrap(),
            },
            Interval {
                start: Time::new(20, 0, 0, 0).unwrap(),
                end: Time::new(22, 15, 0, 0).unwrap(),
            },
        ];
        assert_eq!(overtime_minutes(date, &work_gaps, &customer), 510);
    }

    #[test]
    fn example() {
        let input = fs::read_to_string("test-input").unwrap();
        assert_eq!(solution(&input), 3030);
    }
}
