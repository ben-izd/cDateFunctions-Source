//noinspection RsMainFunctionNotFound
use std::time::{SystemTime, UNIX_EPOCH};

type YearType = i64;
type MonthType = u8;
type DayType = u8;
type HourType = u8;
type MinuteType = u8;

// used for receiving second in second to datelist conversion
type AbsoluteType = i64;

// used for refining datelist or converting datelist arguments to second
type DateItemMaxType = i64;

type Instant = f64;

const DAY_SECOND: AbsoluteType = 86400;
const HOUR_SECOND: AbsoluteType = 3600;
const MINUTE_SECOND: AbsoluteType = 60;


#[derive(Copy, Clone)]
enum WeekDay {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}


#[repr(C)]
pub struct DateListResultWithoutSecond {
    pub year: YearType,
    pub month: MonthType,
    pub day: DayType,
    pub hour: HourType,
    pub minute: MinuteType,
}


pub mod gregorian {
    use super::{WeekDay, LeapFunctions, DateItemMaxType, AbsoluteType, CalendarOptions, YearType, MonthType, DayType, CalendarTemplate, DateListResultWithoutSecond};

    pub const ORIGIN_OFFSET_TO_1900: i64 = 59926608000;


    fn is_leap_year(mut year: YearType) -> bool {
        if year.is_negative() {
            year += 1;
        }
        ((year % 4) == 0 && (year % 100) != 0 || (year % 400) == 0) || year == 0
    }

    fn count_leap_years(mut year: YearType) -> YearType {
        // println!("--input year {}",year);
        if year.is_negative() {
            year += 1;
        }
        // println!("-- [CO-Old]:{}" , year / 4 - year / 100 + year / 400 - if year.is_negative() { 1 } else { 0 });
        year / 4 - year / 100 + year / 400 - if !year.is_positive() { 1 } else { 0 }
    }

    pub static CALENDAR: CalendarTemplate<12> = CalendarTemplate::new([31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], 2, WeekDay::Monday, 4, LeapFunctions { is_leap_year, count_leap_years }, CalendarOptions { has_year_0: false, mean_year_length_second: None });


    #[no_mangle]
    pub extern "C" fn gregorian_is_leap_year(year: YearType) -> bool {
        is_leap_year(year)
    }

    #[no_mangle]
    pub extern "C" fn second_to_date_list(absolute_value: AbsoluteType) -> DateListResultWithoutSecond {
        CALENDAR.second_to_date_list(absolute_value + ORIGIN_OFFSET_TO_1900)
    }

    #[no_mangle]
    pub extern "C" fn date_list_to_second(year: DateItemMaxType, month: DateItemMaxType, day: DateItemMaxType, hour: DateItemMaxType, minute: DateItemMaxType) -> AbsoluteType {
        CALENDAR.date_list_to_second(if year == 0 { -1 } else { year }, month, day, hour, minute) - ORIGIN_OFFSET_TO_1900
    }

    #[no_mangle]
    pub extern "C" fn date_list_to_date_list(year: DateItemMaxType, month: DateItemMaxType, day: DateItemMaxType, hour: DateItemMaxType, minute: DateItemMaxType) -> DateListResultWithoutSecond {
        CALENDAR.second_to_date_list(CALENDAR.date_list_to_second(if year == 0 { -1 } else { year }, month, day, hour, minute))
    }

    fn day_name_index(date: AbsoluteType) -> usize {
        (((date as DateItemMaxType).div_euclid(86400) % 7) + 1) as usize
    }


    pub fn is_holiday_no_correction(year: YearType, month: MonthType, day: DayType) -> bool {
        (match month {
            1 => (year >= 1871 && day == 1) || (year >= 1986 && day == [15, 21, 20, 19, 18, 17, 16][day_name_index(CALENDAR.date_list_to_second(year, 1, 1, 0, 0))]),
            2 => (1885 < year && year < 1971 && day == 22) || (year >= 1971 && day == [15, 21, 20, 19, 18, 17, 16][day_name_index(CALENDAR.date_list_to_second(year, 2, 1, 0, 0))]),
            5 => (year >= 1971) && day == [29, 28, 27, 26, 25, 31, 30][day_name_index(CALENDAR.date_list_to_second(year, 5, 1, 0, 0))],
            6 => year >= 2021 && day == 19,
            7 => year >= 1941 && day == 4,
            9 => year >= 1894 && day == [1, 7, 6, 5, 4, 3, 2][day_name_index(CALENDAR.date_list_to_second(year, 9, 1, 0, 0))],

            10 => ((1971..=1977).contains(&year)
                && day == [22, 28, 27, 26, 25, 24, 23][day_name_index(CALENDAR.date_list_to_second(year, 10, 1, 0, 0))]
                || (1936 < year && year < 1971 && day == 12)
                || (year >= 1971 && day == [8, 14, 13, 12, 11, 10, 9][day_name_index(CALENDAR.date_list_to_second(year, 10, 1, 0, 0))])),

            11 => (year >= 1938 && !(1971..=1977).contains(&year) && day == 11) || (year >= 1863 && day == [25, 24, 23, 22, 28, 27, 26][day_name_index(CALENDAR.date_list_to_second(year, 11, 1, 0, 0))]),
            12 => year >= 1871 && day == 25,
            _ => false
        } ||
            /* is monday and sunday was a official holiday */ day_name_index(CALENDAR.date_list_to_second(year, month as DateItemMaxType, day as DateItemMaxType, 0, 0)) == 1 && {
            let temp = date_list_to_date_list(year, month as DateItemMaxType, (day - 1) as DateItemMaxType, 0, 0);
            is_holiday_no_correction(temp.year, temp.month, temp.day)
        } ||
            /* is friday and saturday is a official holiday */ day_name_index(CALENDAR.date_list_to_second(year, month as DateItemMaxType, day as DateItemMaxType, 0, 0)) == 5 && {
            let temp = date_list_to_date_list(year, month as DateItemMaxType, (day + 1) as DateItemMaxType, 0, 0);
            is_holiday_no_correction(temp.year, temp.month, temp.day)
        })
    }

    #[no_mangle]
    pub extern "C" fn is_holiday_with_correction(year: DateItemMaxType, month: DateItemMaxType, day: DateItemMaxType) -> bool {
        let temp = date_list_to_date_list(if year == 0 { -1 } else { year }, month, day, 0, 0);
        is_holiday_no_correction(temp.year, temp.month, temp.day)
    }

    #[no_mangle]
    pub extern "C" fn date_list_now() -> DateListResultWithoutSecond {
        CALENDAR.second_to_date_list(super::absolute_time_now() as AbsoluteType)
    }


    #[no_mangle]
    pub extern "C" fn gergorian_month_view(date: AbsoluteType) -> super::MonthView {
        // date should be offset from 1900
        CALENDAR.generate_month_view(ORIGIN_OFFSET_TO_1900 + date)
    }

    #[no_mangle]
    pub extern "C" fn gergorian_year_view(start: AbsoluteType, end: AbsoluteType) -> super::YearView {
        // start,end should be offset from 1900
        CALENDAR.generate_year_view(ORIGIN_OFFSET_TO_1900 + start, ORIGIN_OFFSET_TO_1900 + end)
    }
}


#[no_mangle]
pub extern "C" fn unix_time_now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64()
}

#[no_mangle]
pub extern "C" fn unix_date_list_to_second(year: DateItemMaxType, month: DateItemMaxType, day: DateItemMaxType, hour: DateItemMaxType, minute: DateItemMaxType) -> AbsoluteType {
    gregorian::date_list_to_second(year, month, day, hour, minute) - 2208988800
}

#[no_mangle]
pub extern "C" fn julian_time_now() -> Instant {
    (absolute_time_now() + 208657771200.) / DAY_SECOND as Instant
}

#[no_mangle]
pub extern "C" fn julian_date_list_to_second(year: DateItemMaxType, month: DateItemMaxType, day: DateItemMaxType, hour: DateItemMaxType, minute: DateItemMaxType) -> AbsoluteType {
    gregorian::date_list_to_second(year, month, day, hour, minute) + 208657771200
}


#[no_mangle]
pub extern "C" fn absolute_time_now() -> Instant {
    unix_time_now() + 2208988800.
}


#[no_mangle]
pub extern "C" fn unix_second_to_date_list(absolute_value: AbsoluteType) -> DateListResultWithoutSecond {
    if absolute_value == 0 {
        return DateListResultWithoutSecond {
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
        };
    }

    gregorian::second_to_date_list(absolute_value + 2208988800)
}


#[no_mangle]
pub extern "C" fn julian_second_to_date_list(absolute_value: AbsoluteType) -> DateListResultWithoutSecond {
    if absolute_value == 0 {
        return DateListResultWithoutSecond {
            year: -4714,
            month: 11,
            day: 24,
            hour: 15,
            minute: 30,
        };
    }

    // 208657771200.0 is difference between julian date base (-4714,...) and absolute time base (1900)
    gregorian::second_to_date_list(absolute_value * DAY_SECOND - 208657771200)
}


pub mod arithmetic_persian {
    use super::{MonthView, YearView, WeekDay, YearType, LeapCycle, LeapSubCycle};
    use super::{LeapFunctions, CalendarOptions, CalendarTemplate, DateItemMaxType, AbsoluteType, DateListResultWithoutSecond};

    pub const ORIGIN_OFFSET_TO_1900: i64 = 40322880000;

    const fn persian_base_calendar_template(leap_functions: LeapFunctions) -> CalendarTemplate<12> {
        CalendarTemplate::new([31, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 29], 12, WeekDay::Friday, 4, leap_functions, CalendarOptions { has_year_0: false, mean_year_length_second: None })
    }
    // IDs are sorted by date the calendars introduced

    // Cycle 33 - by Omar Khayyam - ID:0

    pub static LEAP_CYCLE_33: LeapCycle = LeapCycle {
        cycle_generator: |x| match x {
            4 => LeapSubCycle::new([0, 0, 0, 1], None),
            5 => LeapSubCycle::new([0, 0, 0, 0, 1], None),
            33 => LeapSubCycle::new([5, 4, 4, 4, 4, 4, 4, 4], Some([1; 8])),
            _ => unreachable!()
        },
        length: 33,
        number_of_leap_years_per_cycle: 8,
        start_year_offset: 16,
        number_of_leap_years_in_offset: 3,
    };

    pub static CALENDAR_CYCLE_33: CalendarTemplate<12> = persian_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_33.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_33.count_leap_years(x) });

    // End of Cycle 33

    // Cycle 128 - by Hassan Taghi Zade - ID:1

    static LEAP_CYCLE_128: LeapCycle = LeapCycle {
        cycle_generator: |x| match x {
            4 => LeapSubCycle::new([0, 0, 0, 1], None),
            5 => LeapSubCycle::new([0, 0, 0, 0, 1], None),
            29 => LeapSubCycle::new([5, 4, 4, 4, 4, 4, 4], Some([1; 7])),
            33 => LeapSubCycle::new([5, 4, 4, 4, 4, 4, 4, 4], Some([1; 8])),
            128 => LeapSubCycle::new([29, 33, 33, 33], Some([7, 8, 8, 8])),
            _ => unreachable!()
        },
        length: 128,
        number_of_leap_years_per_cycle: 31,
        start_year_offset: 0,
        number_of_leap_years_in_offset: 0,
    };

    pub static CALENDAR_CYCLE_128: CalendarTemplate<12> = persian_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_128.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_128.count_leap_years(x) });

    // End of Cycle 128


    // Cycle 2820-1 - by Zabih Allah Behrouz - ID:2

    pub static LEAP_CYCLE_2820_1: LeapCycle = LeapCycle {
        cycle_generator: |x| match x {
            4 => LeapSubCycle::new([0, 0, 0, 1], None),
            5 => LeapSubCycle::new([0, 0, 0, 0, 1], None),
            29 => LeapSubCycle::new([5, 4, 4, 4, 4, 4, 4], Some([1; 7])),
            33 => LeapSubCycle::new([5, 4, 4, 4, 4, 4, 4, 4], Some([1; 8])),
            128 => LeapSubCycle::new([29, 33, 33, 33], Some([7, 8, 8, 8])),
            2820 => LeapSubCycle::new([128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 4], Some([31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 1])),
            _ => unreachable!()
        },
        length: 2820,
        number_of_leap_years_per_cycle: 683,
        start_year_offset: 2346,
        number_of_leap_years_in_offset: 568,
    };

    pub static CALENDAR_CYCLE_2820_1: CalendarTemplate<12> = persian_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_2820_1.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_2820_1.count_leap_years(x) });

    // End of Cycle-1

    // Cycle 2820-2 - by Musa Akrami - ID:3

    static LEAP_CYCLE_2820_2: LeapCycle = LeapCycle {
        cycle_generator: |x| match x {
            4 => LeapSubCycle::new([0, 0, 0, 1], None),
            5 => LeapSubCycle::new([0, 0, 0, 0, 1], None),
            29 => LeapSubCycle::new([5, 4, 4, 4, 4, 4, 4], Some([1; 7])),
            33 => LeapSubCycle::new([5, 4, 4, 4, 4, 4, 4, 4], Some([1; 8])),
            128 => LeapSubCycle::new([29, 33, 33, 33], Some([7, 8, 8, 8])),
            673 => LeapSubCycle::new([128, 128, 128, 128, 128, 33], Some([31, 31, 31, 31, 31, 8])),
            2820 => LeapSubCycle::new([128, 673, 673, 673, 673], Some([31, 163, 163, 163, 163])),
            _ => unreachable!()
        },
        length: 2820,
        number_of_leap_years_per_cycle: 683,
        start_year_offset: 2346,
        number_of_leap_years_in_offset: 568,
    };

    pub static CALENDAR_CYCLE_2820_2: CalendarTemplate<12> = persian_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_2820_2.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_2820_2.count_leap_years(x) });

    // End of Cycle-2 2820

    // Cycle 4166 - by Ahmad Farmad - ID:4

    static LEAP_CYCLE_4166: LeapCycle = LeapCycle {
        cycle_generator: |x| match x {
            4 => LeapSubCycle::new([0, 0, 0, 1], None),
            5 => LeapSubCycle::new([0, 0, 0, 0, 1], None),
            29 => LeapSubCycle::new([4, 4, 4, 4, 4, 4, 5], Some([1; 7])),
            33 => LeapSubCycle::new([4, 4, 4, 4, 4, 4, 5, 4], Some([1; 8])),
            128 => LeapSubCycle::new([29, 33, 33, 33], Some([7, 8, 8, 8])),
            673 => LeapSubCycle::new([128, 128, 128, 128, 128, 33], Some([31, 31, 31, 31, 31, 8])),
            4166 => LeapSubCycle::new([673, 673, 673, 673, 673, 673, 128], Some([163, 163, 163, 163, 163, 163, 31])),
            _ => unreachable!()
        },
        length: 4166,
        number_of_leap_years_per_cycle: 1009,
        start_year_offset: 0,
        number_of_leap_years_in_offset: 0,
    };

    pub static CALENDAR_CYCLE_4166: CalendarTemplate<12> = persian_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_4166.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_4166.count_leap_years(x) });

    // End of Cycle 4166

    #[no_mangle]
    pub extern "C" fn arithmetic_persian_is_leap_year(cycle_id: u8, year: YearType) -> bool {
        match cycle_id {
            0 => LEAP_CYCLE_33.is_leap_year(year),
            1 => LEAP_CYCLE_128.is_leap_year(year),
            2 => LEAP_CYCLE_2820_1.is_leap_year(year),
            3 => LEAP_CYCLE_2820_2.is_leap_year(year),
            4 => LEAP_CYCLE_4166.is_leap_year(year),
            _ => unreachable!()
        }
    }

    #[no_mangle]
    pub extern "C" fn arithmetic_persian_second_to_date_list(cycle_id: u8, absolute_value: AbsoluteType) -> DateListResultWithoutSecond {
        match cycle_id {
            0 => CALENDAR_CYCLE_33.second_to_date_list(absolute_value),
            1 => CALENDAR_CYCLE_128.second_to_date_list(absolute_value),
            2 => CALENDAR_CYCLE_2820_1.second_to_date_list(absolute_value),
            3 => CALENDAR_CYCLE_2820_2.second_to_date_list(absolute_value),
            4 => CALENDAR_CYCLE_4166.second_to_date_list(absolute_value),
            _ => unreachable!(),
        }
    }

    #[no_mangle]
    pub extern "C" fn arithmetic_persian_date_list_to_second(cycle_id: u8, mut year: DateItemMaxType, month: DateItemMaxType, day: DateItemMaxType, hour: DateItemMaxType, minute: DateItemMaxType) -> AbsoluteType {
        if year == 0 {
            year = -1;
        };

        match cycle_id {
            0 => CALENDAR_CYCLE_33.date_list_to_second(year, month, day, hour, minute),
            1 => CALENDAR_CYCLE_128.date_list_to_second(year, month, day, hour, minute),
            2 => CALENDAR_CYCLE_2820_1.date_list_to_second(year, month, day, hour, minute),
            3 => CALENDAR_CYCLE_2820_2.date_list_to_second(year, month, day, hour, minute),
            4 => CALENDAR_CYCLE_4166.date_list_to_second(year, month, day, hour, minute),
            _ => unreachable!()
        }
    }

    #[no_mangle]
    pub extern "C" fn arithmetic_persian_month_view(cycle_id: u8, date: AbsoluteType) -> MonthView {
        match cycle_id {
            0 => CALENDAR_CYCLE_33.generate_month_view(date),
            1 => CALENDAR_CYCLE_128.generate_month_view(date),
            2 => CALENDAR_CYCLE_2820_1.generate_month_view(date),
            3 => CALENDAR_CYCLE_2820_2.generate_month_view(date),
            4 => CALENDAR_CYCLE_4166.generate_month_view(date),
            _ => unreachable!()
        }
    }

    #[no_mangle]
    pub extern "C" fn arithmetic_persian_year_view(cycle_id: u8, start: AbsoluteType, end: AbsoluteType) -> YearView {
        match cycle_id {
            0 => CALENDAR_CYCLE_33.generate_year_view(start, end),
            1 => CALENDAR_CYCLE_128.generate_year_view(start, end),
            2 => CALENDAR_CYCLE_2820_1.generate_year_view(start, end),
            3 => CALENDAR_CYCLE_2820_2.generate_year_view(start, end),
            4 => CALENDAR_CYCLE_4166.generate_year_view(start, end),
            _ => unreachable!()
        }
    }
}


pub mod islamic {
    // Uses Microsoft implementation leap cycle : 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29
    use super::{YearView, MonthView, WeekDay, LeapFunctions, CalendarOptions, CalendarTemplate, DateItemMaxType, YearType, AbsoluteType, DateListResultWithoutSecond};

    pub const ORIGIN_OFFSET_TO_1900: i64 = 40312598400;

    pub struct IslamicLeapCycle {
        pub leap_cycle: [bool; 30],
    }

    impl IslamicLeapCycle {
        pub const fn new(leap_years: [u8; 11]) -> Self {
            let mut leap_cycle = [false; 30];

            let mut index: usize = 0;
            while index < 11 {
                leap_cycle[leap_years[index] as usize - 1] = true;
                index += 1;
            }


            IslamicLeapCycle { leap_cycle }
        }

        pub fn is_leap_year(&self, year: YearType) -> bool {
            self.leap_cycle[((year - 1).rem_euclid(30)) as usize]
        }

        pub fn count_leap_years(&self, mut year: YearType) -> YearType {
            if year.is_negative() { year -= 1; };
            (year.div_euclid(30)) * 11 +
                {
                    let mut count = 0;
                    self.leap_cycle.iter().take(year.rem_euclid(30) as usize).for_each(|&x| if x { count += 1; });
                    count
                }
        }
    }


    const fn islamic_base_calendar_template(leap_functions: LeapFunctions) -> CalendarTemplate<12> {
        CalendarTemplate::new([30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 29], 12, WeekDay::Friday, 2, leap_functions, CalendarOptions { has_year_0: true, mean_year_length_second: Some(31570560) })
    }

    // Source:  https://www.joda.org/joda-time/cal_islamic.html
    // 15-based 	    2, 5, 7, 10, 13, 15, 18, 21, 24, 26, 29
    // 16-based 	    2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29
    // Indian 	        2, 5, 8, 10, 13, 16, 19, 21, 24, 27, 29
    // Habash al-Hasib 	2, 5, 8, 11, 13, 16, 19, 21, 24, 27, 30

    static LEAP_CYCLE_BASE_15: IslamicLeapCycle = IslamicLeapCycle::new([2, 5, 7, 10, 13, 15, 18, 21, 24, 26, 29]);
    pub static CALENDAR_CYCLE_15: CalendarTemplate<12> = islamic_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_BASE_15.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_BASE_15.count_leap_years(x) });

    static LEAP_CYCLE_BASE_16: IslamicLeapCycle = IslamicLeapCycle::new([2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29]);
    pub static CALENDAR_CYCLE_16: CalendarTemplate<12> = islamic_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_BASE_16.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_BASE_16.count_leap_years(x) });

    static LEAP_CYCLE_INDIAN: IslamicLeapCycle = IslamicLeapCycle::new([2, 5, 8, 10, 13, 16, 19, 21, 24, 27, 29]);
    pub static CALENDAR_CYCLE_INDIAN: CalendarTemplate<12> = islamic_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_INDIAN.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_INDIAN.count_leap_years(x) });

    static LEAP_CYCLE_HABASH_AL_HASIB: IslamicLeapCycle = IslamicLeapCycle::new([2, 5, 8, 11, 13, 16, 19, 21, 24, 27, 30]);
    pub static CALENDAR_CYCLE_HABASH_AL_HASIB: CalendarTemplate<12> = islamic_base_calendar_template(LeapFunctions { is_leap_year: |x| LEAP_CYCLE_HABASH_AL_HASIB.is_leap_year(x), count_leap_years: |x| LEAP_CYCLE_HABASH_AL_HASIB.count_leap_years(x) });

    #[no_mangle]
    pub extern "C" fn islamic_is_leap_year(cycle_id: u8, year: YearType) -> bool {
        match cycle_id {
            0 => LEAP_CYCLE_BASE_15.is_leap_year(year),
            1 => LEAP_CYCLE_BASE_16.is_leap_year(year),
            2 => LEAP_CYCLE_INDIAN.is_leap_year(year),
            3 => LEAP_CYCLE_HABASH_AL_HASIB.is_leap_year(year),
            _ => unreachable!()
        }
    }

    #[no_mangle]
    pub extern "C" fn islamic_second_to_date_list(cycle_id: u8, absolute_value: AbsoluteType) -> DateListResultWithoutSecond {
        match cycle_id {
            0 => CALENDAR_CYCLE_15.second_to_date_list(absolute_value),
            1 => CALENDAR_CYCLE_16.second_to_date_list(absolute_value),
            2 => CALENDAR_CYCLE_INDIAN.second_to_date_list(absolute_value),
            3 => CALENDAR_CYCLE_HABASH_AL_HASIB.second_to_date_list(absolute_value),
            _ => unreachable!()
        }
    }

    #[no_mangle]
    pub extern "C" fn islamic_date_list_to_second(cycle_id: u8, mut year: DateItemMaxType, month: DateItemMaxType, day: DateItemMaxType, hour: DateItemMaxType, minute: DateItemMaxType) -> AbsoluteType {
        if year == 0 {
            year = -1;
        };

        match cycle_id {
            0 => CALENDAR_CYCLE_15.date_list_to_second(year, month, day, hour, minute),
            1 => CALENDAR_CYCLE_16.date_list_to_second(year, month, day, hour, minute),
            2 => CALENDAR_CYCLE_INDIAN.date_list_to_second(year, month, day, hour, minute),
            3 => CALENDAR_CYCLE_HABASH_AL_HASIB.date_list_to_second(year, month, day, hour, minute),
            _ => unreachable!()
        }
    }

    #[no_mangle]
    pub extern "C" fn islamic_month_view(cycle_id: u8, mut date: AbsoluteType) -> MonthView {

        match cycle_id {
            0 => CALENDAR_CYCLE_15.generate_month_view(date),
            1 => CALENDAR_CYCLE_16.generate_month_view(date),
            2 => CALENDAR_CYCLE_INDIAN.generate_month_view(date),
            3 => CALENDAR_CYCLE_HABASH_AL_HASIB.generate_month_view(date),
            _ => unreachable!()
        }
    }

    #[no_mangle]
    pub extern "C" fn islamic_year_view(cycle_id: u8, mut start: AbsoluteType, mut end: AbsoluteType) -> YearView {

        match cycle_id {
            0 => CALENDAR_CYCLE_15.generate_year_view(start, end),
            1 => CALENDAR_CYCLE_16.generate_year_view(start, end),
            2 => CALENDAR_CYCLE_INDIAN.generate_year_view(start, end),
            3 => CALENDAR_CYCLE_HABASH_AL_HASIB.generate_year_view(start, end),
            _ => unreachable!()
        }
    }
}



// use this template to model calendars which :
// - have 24-hour day
// - have a fix list of days for months except leap year and fix leap year
// - leap day is always 1 day and is additive
// - leap day always will be added to 1 month only
pub struct CalendarTemplate<const SIZE: usize> {
    // used to give is_leap_year and count_leap_years functions (because as of rust version 1.57, pointer is not allowed in const fn)
    leap_functions: LeapFunctions,

    // will be calculated automatically using ::new
    // number of total second for a non-leap-year
    year_in_second: AbsoluteType,

    // number of days for each month without considering leap day:
    // gregorian : [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    months: [u8; SIZE],

    // which month will have the leap day (1-base indexing)
    // 1,2,..,11,12 for a 12 month calendar
    leap_month: usize,

    // used for year approximation by dividing second elapsed from origin to this number
    mean_year_length_second: AbsoluteType,

    has_year_0: bool,

    first_day: WeekDay,

    leap_cycle_safe_guard: u8,

    // will be calculated automatically by using ::new
    // accumulated months
    // first element should be always zero
    // is accumulated month of : [0,1,1..2,...,1..12]
    accumulated_months_in_second: [AbsoluteType; SIZE],

    // will be calculated automatically by using ::new
    // accumulated months from the end of list
    // numbers are negative
    // last element is always zero (actually is extra, probably will be fixed in future version of rust)
    // is accumulated month of : [12,12..11,12..10,...,12..1,0]
    accumulated_months_reverse_in_second: [AbsoluteType; SIZE],
}


impl<const SIZE: usize> CalendarTemplate<SIZE> {
    const fn new(months: [u8; SIZE], leap_month: usize, first_day: WeekDay, min_leap_year_cycle: u8, leap_functions: LeapFunctions, calendar_options: CalendarOptions) -> CalendarTemplate<SIZE> {


        // calculate year in second
        let mut year_in_second = 0;
        let mut index = 0;
        while index < SIZE {
            year_in_second += months[index] as AbsoluteType * DAY_SECOND;
            index += 1;
        }

        // calcuate accumulated_months
        let mut accumulated_months: [AbsoluteType; SIZE] = [0; SIZE];
        {
            let mut index = 1;
            while index < SIZE {
                let mut inner_index = 0;
                let mut sum = 0;
                while inner_index < index {
                    sum += months[inner_index] as u16;
                    inner_index += 1;
                }
                accumulated_months[index] = sum as AbsoluteType * DAY_SECOND;
                index += 1;
            }
        }


        // calculate accumulated_months_reverse
        let mut accumulated_months_reverse: [AbsoluteType; SIZE] = [0; SIZE];

        {
            let temp_size = SIZE;
            let mut index = 1;
            while index < SIZE {
                let mut inner_index = index;
                let mut sum = 0;
                while inner_index < SIZE {
                    sum += months[inner_index] as u16;
                    inner_index += 1;
                }
                accumulated_months_reverse[temp_size - index - 1] = sum as AbsoluteType * -DAY_SECOND;
                index += 1;
            }
        }


        CalendarTemplate {
            leap_functions,
            year_in_second,
            months,
            leap_month,
            first_day,
            has_year_0: calendar_options.has_year_0,
            leap_cycle_safe_guard: min_leap_year_cycle - 1,
            mean_year_length_second: if let Some(value) = calendar_options.mean_year_length_second { value } else { year_in_second + DAY_SECOND / min_leap_year_cycle as AbsoluteType },
            accumulated_months_in_second: accumulated_months,
            accumulated_months_reverse_in_second: accumulated_months_reverse,
        }
    }


    fn calculate_days(&self, year: YearType, month: u8) -> DayType {
        self.months[(month - 1) as usize] + if month == self.leap_month as u8 { (self.leap_functions.is_leap_year)(year) as DayType } else { 0 }
    }

    pub fn second_to_date_list(&self, mut absolute_value: AbsoluteType) -> DateListResultWithoutSecond {
        let mut year: YearType = 1;
        let mut month: MonthType = 1;
        let mut day: DayType = 1;
        let mut hour: HourType = 0;
        let mut minute: MinuteType = 0;

        // println!("-- absolute value : {} , mean year : {}",absolute_value,self.mean_year_length_second);

        // approximate year
        let temp = ((absolute_value.abs() / self.mean_year_length_second) - self.leap_cycle_safe_guard as AbsoluteType).max(0) as YearType;

        // println!("-- found approximately {} years",temp);

        if absolute_value.is_positive() {
            year += temp;

            if temp.is_positive() {
                absolute_value -= temp as AbsoluteType * self.year_in_second + ((self.leap_functions.count_leap_years)(year - 1) as AbsoluteType * DAY_SECOND);
            }

            // println!("-- before: year is {} , abs {}",year,absolute_value);

            {
                let mut temp = self.year_in_second + if (self.leap_functions.is_leap_year)(year) { DAY_SECOND } else { 0 };

                // println!("-- next year in second : {}",temp);

                while absolute_value >= temp {
                    absolute_value -= temp;
                    year += 1;
                    temp = self.year_in_second + if (self.leap_functions.is_leap_year)(year) { DAY_SECOND } else { 0 };
                }
            }

            // println!("-- after: year is {} ,abs {}",year,absolute_value);

            // calculate month
            {
                let mut index = SIZE - 1;
                let temp_leap_year_value = if (self.leap_functions.is_leap_year)(year) { DAY_SECOND } else { 0 };

                // println!("-- is leap year {} ",temp_leap_year_value==86400.);

                while index > 0 {
                    let temp = self.accumulated_months_in_second[index]
                        + if index >= self.leap_month { temp_leap_year_value } else { 0 };


                    if absolute_value >= temp {

                        // println!("-- entered {},temp: {}, abs : {}",index,temp,absolute_value);

                        absolute_value -= temp;
                        month = index as MonthType + 1;
                        break;
                    }

                    index -= 1;
                }

                // println!("-- month is {}, index is {}",month,index);

            }

            // println!("-- abs {} / day {} = number of days founded {} ",absolute_value,DAY_SECOND,(absolute_value as i32) / DAY_SECOND);

            day += (absolute_value / DAY_SECOND) as DayType;


            hour = ((absolute_value % DAY_SECOND) / HOUR_SECOND) as HourType;

            minute = ((absolute_value % HOUR_SECOND) / MINUTE_SECOND) as MinuteType;
        } else if absolute_value.is_negative() {
            if self.has_year_0 {
                year = 0;
            } else {
                year = -1;
            }

            // println!("-- year is {}",year);

            if temp.is_positive() {
                year -= temp;

                // println!("-- found {} leap days in {} years backward",(self.leap_functions.count_leap_years)(year+1),temp);
                // println!("-- value to subtract year {} + {} (leap days)",temp as f64 * self.year_in_second ,- (self.leap_functions.count_leap_years)(year+1));

                absolute_value += temp as AbsoluteType * self.year_in_second - (self.leap_functions.count_leap_years)(year + 1) as AbsoluteType * DAY_SECOND;
            }

            // println!("-- found {} , year is {} ",temp,year);
            // println!("-- abs is {}",absolute_value);

            let mut temp_leap_year_value = if (self.leap_functions.is_leap_year)(year) { DAY_SECOND } else { 0 };
            {
                let mut temp = -(self.year_in_second + temp_leap_year_value);
                while absolute_value <= temp {

                    // println!("-- value to subtract year {} + {} (leap days)",temp ,temp_leap_year_value);

                    absolute_value -= temp;
                    year -= 1;

                    // println!("-- year {} - leap year : {} , abs is {}",year,(self.leap_functions.is_leap_year)(year),absolute_value);

                    temp_leap_year_value = if (self.leap_functions.is_leap_year)(year) { DAY_SECOND } else { 0 };
                    temp = -(self.year_in_second + temp_leap_year_value);
                }
            }

            // println!("-- after: year is {} ",year);
            // calculate month
            {
                let mut index = SIZE - 1;

                // println!("-- is leap year {} ",temp_leap_year_value==86400.);

                while index > 0 {
                    let temp = self.accumulated_months_reverse_in_second[index - 1]
                        - if SIZE - index < self.leap_month { temp_leap_year_value } else { 0 };

                    if absolute_value <= temp {
                        absolute_value -= temp;
                        break;
                    }

                    index -= 1;
                }

                month = SIZE as MonthType - index as MonthType;

                // println!("-- month is {}",month);

            }

            // println!("-- abs is {} , number of days founded {} ",absolute_value,(absolute_value as i32) / DAY_SECOND);

            day = (self.calculate_days(year, month) as i32 + (absolute_value / DAY_SECOND) as i32) as DayType;

            // println!("-- before day {} {}:{}",day,hour,minute);

            {
                let use_second = (absolute_value % MINUTE_SECOND) != 0;
                minute += (if use_second { 59 } else { 60 } + (((absolute_value % HOUR_SECOND) as i16) / MINUTE_SECOND as i16)) as MinuteType;

                // println!("-- use second : {} , minute : {}",use_second,minute);
                // println!("-- hour in abs {}",((absolute_value % DAY_SECOND) as i32) / HOUR_SECOND);

                hour += (if minute != 60 { 23 } else {
                    minute = 0;
                    24
                } + (((absolute_value % DAY_SECOND) as i32) / HOUR_SECOND as i32)) as HourType;

                // println!("-- hour is {}",hour);

            }

            // println!("-- after {}:{}",hour,minute);

            if hour == 24 {

                // is last day
                if absolute_value / DAY_SECOND == 0 {
                    if month == 12 {
                        month = 1;
                        year += 1;
                    } else {
                        month += 1;
                    }
                    day = 1;
                } else {
                    day += 1;
                }
                hour = 0;
            }

        }

        // println!("-- month is {} , with max day {}!",month,self.months[month as usize -1]);
        // println!("-- day is {}!",day);
        // assert!(0 < day && day <= self.months[month as usize - 1]);

        DateListResultWithoutSecond { year, month, day, hour, minute }
    }
    pub fn date_list_to_second(&self, mut year: DateItemMaxType, mut month: DateItemMaxType, mut day: DateItemMaxType, hour: DateItemMaxType, minute: DateItemMaxType) -> AbsoluteType {
        let old_year = year;
        year += (month - 1).div_euclid(SIZE as YearType);

        month = (month - 1).rem_euclid(SIZE as DateItemMaxType) + 1;
        day -= 1;

        // println!("[ABS] {}-{}-{}",year,month,day);

        {
            let original_year = year;

            if self.has_year_0 {
                if !matches!(year,0|1) {
                    day += (self.leap_functions.count_leap_years)(year - year.signum());
                }
                if !year.is_positive() {
                    year -= 1;
                }
            } else {

                // if you pass the origin and don't have year zero
                if (old_year.is_negative() && !year.is_negative()) || (old_year.is_positive() && !year.is_positive()) {
                    // if year.signum() != old_year.signum() {
                    // println!("[ABS] year.signum()");
                    year += if year > old_year { 1 } else { -1 };
                } else if year == 0 {
                    // old_year==year==0
                    year = -1;
                }


                if !matches!(year, -1|0|1) {
                    day += (self.leap_functions.count_leap_years)(year - year.signum());
                }
            }

            // add/minus leap day for positive/negative years
            day += if (year.is_positive() && month as usize > self.leap_month) // for gregorian: YYYY-3..12 (y>0)
                || (year.is_negative() && ((month as usize) <= self.leap_month  // for gregorian YYYY-1 (y<0)
                // || (month as usize)==self.leap_month && day <= (self.months[self.leap_month-1]) as DateItemMaxType
            )) // for gregorian YYYY-2-dd (y<0 - d<=28)
            { if (self.leap_functions.is_leap_year)(if self.has_year_0 { original_year } else { year }) { year.signum() } else { 0 } } else { 0 };
        }

        // println!("-- year {} month {} day {}",year,month,day);
        // println!("-- new day {} !",day);

        // year
        (if year.is_positive() { year - 1 } else { year }) as AbsoluteType * self.year_in_second

            // month
            + (self.accumulated_months_in_second[(month - 1) as usize])

            + (day as AbsoluteType * DAY_SECOND)
            + (hour as AbsoluteType * HOUR_SECOND)
            + (minute as AbsoluteType * MINUTE_SECOND)
    }

    // unused
    pub fn add_to_hour_minute(&self, date: &mut DateListResultWithoutSecond, hour: i8, minute: i8) {
        // assuming input date is a valid date

        // println!("input {:?} , hour: {} , minute: {}",date,hour,minute);

        let total_minute = date.minute as i8 + minute;
        let total_hour = date.hour as i8 + hour + total_minute.div_euclid(60);

        date.hour = total_hour.rem_euclid(24) as HourType;
        date.minute = total_minute.rem_euclid(60) as MinuteType;

        let mut new_day = total_hour.div_euclid(24) as i16 + date.day as i16;

        // println!("total_hour: {} total_minute: {} new_day: {}",total_hour,total_minute,new_day);


        if new_day.is_positive() {
            let mut max_day = self.get_max_day(date);
            while new_day as DayType > max_day {
                new_day -= max_day as i16;

                if date.month == SIZE as MonthType {
                    date.month = 1;
                    date.year += 1;
                } else { date.month += 1; }

                max_day = self.get_max_day(date);
            }
        } else {
            // new_day is zero/negative

            let mut last_month_max_day = self.get_last_month_max_day(date) as i16;
            loop {
                new_day += last_month_max_day;
                if date.month == 1 {
                    date.month = SIZE as MonthType;
                    date.year -= 1;
                } else { date.month -= 1; }
                last_month_max_day += self.get_last_month_max_day(date) as i16;
                if new_day > 0 {
                    break;
                }
            }
        }
        date.day = new_day as DayType;
    }
    // unused
    pub fn get_max_day(&self, date: &DateListResultWithoutSecond) -> DayType {
        self.months[(date.month - 1) as usize] + if date.month == self.leap_month as MonthType { if (self.leap_functions.is_leap_year)(date.year) { 1 } else { 0 } } else { 0 }
    }
    // unused
    pub fn get_last_month_max_day(&self, date: &DateListResultWithoutSecond) -> DayType {
        let mut use_last_year: bool = false;
        let month_index = if date.month == 1 {
            use_last_year = true;
            SIZE
        } else { (date.month - 1) as usize } - 1;
        self.months[month_index] + if date.month == self.leap_month as MonthType { if (self.leap_functions.is_leap_year)(if use_last_year { date.year - 1 } else { date.year }) { 1 } else { 0 } } else { 0 }
    }

    // unused
    // will only add positive hour
    pub fn add_one_hour(&self, date: &mut DateListResultWithoutSecond) {
        if date.hour == 23 {
            date.hour = 0;


            if date.day == self.months[(date.month - 1) as usize] + if self.leap_month == date.month as usize { (self.leap_functions.is_leap_year)(date.year) as u8 } else { 0 } {
                date.day = 1;
                if date.month == 12 {
                    date.year += 1;
                    date.month = 1;
                } else {
                    date.month += 1;
                }
            } else {
                date.day += 1;
            }
        } else {
            date.hour += 1;
        }
    }
}

#[repr(C)]
pub struct YearView {
    pub offset: u8,
    pub slot_1_start: u8,
    pub slot_1_length: u8,
    pub slot_2_length: u8,
    pub slot_3_length: u8,
    pub slot_4_length: u8,
    pub slot_5_length: u8,
    pub slot_6_length: u8,
    pub slot_7_length: u8,
    pub slot_8_length: u8,
    pub slot_9_length: u8,
    pub slot_10_length: u8,
    pub slot_11_length: u8,
    pub slot_12_length: u8,
}

#[repr(C)]
pub struct MonthView {
    pub offset: u8,
    pub slot_length: u8,
}

impl<const SIZE: usize> CalendarTemplate<SIZE> {
    fn day_index(&self, date: AbsoluteType) -> u8 {
        ((self.first_day as i64 + date.div_euclid(DAY_SECOND)) % 7) as u8
    }

    pub fn generate_month_view(&self, date: AbsoluteType) -> MonthView {
        // Default size : 32
        let date_list = self.second_to_date_list(date);

        let offset: u8 = self.day_index(self.date_list_to_second(date_list.year, date_list.month as DateItemMaxType, 1, 0, 0));
        let slot_length = self.get_max_day(&date_list);

        MonthView { offset, slot_length }
    }
}

impl CalendarTemplate<12> {
    // start and end should be on the same month, otherwise start->end of the start month will be returned
    // end should be greater than start
    pub fn generate_year_view(&self, start: AbsoluteType, end: AbsoluteType) -> YearView {
        let start_date_list = self.second_to_date_list(start);
        let end_date_list = self.second_to_date_list(end);

        let start_year = start_date_list.year;
        let start_month = start_date_list.month;

        let end_year = end_date_list.year;
        let end_month = end_date_list.month;


        let offset = self.day_index(self.date_list_to_second(start_date_list.year, start_date_list.month as DateItemMaxType, start_date_list.day as DateItemMaxType, 0, 0));

        let mut all_slot_length: [u8; 12] = [0; 12];

        let slot_1_start = start_date_list.day as u8;
        all_slot_length[0] = if start_date_list.year == end_date_list.year && start_date_list.month == end_date_list.month { end_date_list.day } else { self.get_max_day(&start_date_list) };

        (1..12).for_each(|index| {
            let temp_year = start_year + (start_month + index - 1).div_euclid(12) as YearType;
            let temp_month = (start_month + index - 1).rem_euclid(12) + 1;

            if temp_year < end_year || (temp_year == end_year && temp_month < end_month) {
                all_slot_length[index as usize] = self.get_max_day(&DateListResultWithoutSecond {
                    year: temp_year,
                    month: temp_month,
                    day: 1,
                    hour: 0,
                    minute: 0,
                });
            } else if temp_year == end_year && temp_month == end_month {
                all_slot_length[index as usize] = end_date_list.day;
            }
        });

        YearView {
            offset,
            slot_1_start,
            slot_1_length: all_slot_length[0],
            slot_2_length: all_slot_length[1],
            slot_3_length: all_slot_length[2],
            slot_4_length: all_slot_length[3],
            slot_5_length: all_slot_length[4],
            slot_6_length: all_slot_length[5],
            slot_7_length: all_slot_length[6],
            slot_8_length: all_slot_length[7],
            slot_9_length: all_slot_length[8],
            slot_10_length: all_slot_length[9],
            slot_11_length: all_slot_length[10],
            slot_12_length: all_slot_length[11],
        }
    }
}


struct LeapFunctions {
    is_leap_year: fn(YearType) -> bool,
    count_leap_years: fn(YearType) -> YearType,
}

struct CalendarOptions {
    has_year_0: bool,
    mean_year_length_second: Option<AbsoluteType>,
}


const MAX_LEAP_CYCLE_LENGTH: usize = 32;

pub struct LeapCycle {
    pub cycle_generator: fn(year: YearType) -> LeapSubCycle,
    pub length: YearType,
    pub number_of_leap_years_per_cycle: YearType,
    pub start_year_offset: YearType,
    pub number_of_leap_years_in_offset: YearType,
}

impl LeapCycle {
    pub fn is_leap_year(&self, mut year: YearType) -> bool {
        if year == 0 {
            year = 1;
        }

        // println!("-- year (index){} ",year);
        // println!("-- given year {}",year);
        // let temp = year ;

        year = (if year.is_positive() { year - 1 } else { year } + self.start_year_offset).rem_euclid(self.length) + 1;

        // println!("--[is] year was {} is {}",temp,year);
        // println!("-- year is {}",year);

        let mut current_sub_cycle_index: usize = 0;
        let mut temp_sub_cycle = (self.cycle_generator)(self.length);
        loop {

            // println!("-- cycle {:?} ",temp_sub_cycle.accumulated_sub_cycles);
            if let Some(value) = temp_sub_cycle.accumulated_sub_cycles.iter().position(|&x| x as YearType >= year)
            {
                current_sub_cycle_index = value;
            }

            // println!("-- before year was {}",year);

            if current_sub_cycle_index > 0 {
                let temp = temp_sub_cycle.accumulated_sub_cycles[current_sub_cycle_index - 1];
                temp_sub_cycle = (self.cycle_generator)((temp_sub_cycle.accumulated_sub_cycles[current_sub_cycle_index] - temp) as YearType);
                year -= temp as YearType;

                // println!("-- should subtract {} years ",temp);

            } else {
                temp_sub_cycle = (self.cycle_generator)(temp_sub_cycle.accumulated_sub_cycles[current_sub_cycle_index] as YearType);
            }

            // println!("-- index found is {} , year is {}",current_sub_cycle_index,year);

            if !temp_sub_cycle.has_sub_sub_cycle {

                // println!("year is {}",year);

                return temp_sub_cycle.accumulated_sub_cycles[(year - 1) as usize] == 1;
            }
        }
    }

    pub fn count_leap_years(&self, mut year: YearType) -> YearType {
        // don't have year zero
        if year == 0 {
            year = 1;
        }

        // println!("-- year (index){} ",year);

        let mut current_sub_cycle_index: usize = 0;
        let mut temp_sub_cycle = (self.cycle_generator)(self.length);

        // println!("year {} , n-cycle : {} , each cycle have {} leap years",year,(year-1).div_euclid(self.length),self.number_of_leap_years_per_cycle);

        year += self.start_year_offset - 1;

        let mut count: YearType = year.div_euclid(self.length) * self.number_of_leap_years_per_cycle;

        // let temp = year;
        // println!("-- initial count is {} , year is {}",count,year);

        year = year.rem_euclid(self.length) + 1;

        // println!("--[CO] year was {} , is {}",temp,year);

        return loop {

            // println!("-- now cycle {} ",temp_sub_cycle.last_item);
            // println!("-- count is {}" ,count);

            if let Some(value) = temp_sub_cycle.accumulated_sub_cycles.iter().position(|&x| x as YearType >= year) {
                current_sub_cycle_index = value;

                // println!("-- found index at {}",value);

            };

            // println!("-- before year was {}",year);

            if current_sub_cycle_index > 0 {
                let temp = temp_sub_cycle.accumulated_sub_cycles[current_sub_cycle_index - 1];
                let temp_next = temp_sub_cycle.accumulated_sub_cycles[current_sub_cycle_index];

                // if year is one of the sub-cycle return directly without going deep
                if year == temp_next as YearType {
                    break count + temp_sub_cycle.accumulated_leap_years_per_cycle[current_sub_cycle_index] as YearType;
                }
                year -= temp as YearType;
                count += temp_sub_cycle.accumulated_leap_years_per_cycle[current_sub_cycle_index - 1] as YearType;

                // println!("-- with {} in {:?} should add {} years ",current_sub_cycle_index - 1,temp_sub_cycle.accumulated_leap_years_per_cycle,temp_sub_cycle.accumulated_leap_years_per_cycle[current_sub_cycle_index - 1]);

                temp_sub_cycle = (self.cycle_generator)((temp_next - temp) as YearType);
            } else {
                temp_sub_cycle = (self.cycle_generator)(temp_sub_cycle.accumulated_sub_cycles[current_sub_cycle_index] as YearType);
            }

            // println!("-- index found is {} , year is {}",current_sub_cycle_index,year);

            if !temp_sub_cycle.has_sub_sub_cycle {
                break count + temp_sub_cycle.accumulated_leap_years_per_cycle[(year - 1) as usize] as YearType;
            }
        } - self.number_of_leap_years_in_offset;
    }
}


pub struct LeapSubCycle {
    pub accumulated_sub_cycles: [u16; MAX_LEAP_CYCLE_LENGTH],
    accumulated_leap_years_per_cycle: [u16; MAX_LEAP_CYCLE_LENGTH],
    has_sub_sub_cycle: bool,
    pub leap_years_per_cycle: u16,
}

impl LeapSubCycle {
    // SIZE should be less than MAX_LEAP_CYCLE_LENGTH
    pub const fn new<const SIZE: usize>(sub_cycles: [u16; SIZE], leap_years_per_cycle: Option<[u16; SIZE]>) -> LeapSubCycle {

        // "panicking in constant functions" - is not supported yet
        // assert!(SIZE<=MAX_LEAP_CYCLE_LENGTH,"Number of months is larger than default size (MAX_LEAP_CYCLE_LENGTH).");

        // calculate accumulated_sub_cycles
        let mut accumulated_sub_cycles = [0; MAX_LEAP_CYCLE_LENGTH];
        let mut sum: u16 = 0;
        {
            let mut index = 0;
            while index < SIZE {
                let mut inner_index = 0;
                sum = 0;
                while inner_index <= index {
                    sum += sub_cycles[inner_index];
                    inner_index += 1;
                }
                accumulated_sub_cycles[index] = sum;
                index += 1;
            }
        }

        // calculate leap_year_per_cycle
        let mut temp_leap_year_per_cycle = [0; MAX_LEAP_CYCLE_LENGTH];

        if let Some(value) = leap_years_per_cycle {
            let mut index = 0;
            while index < SIZE {
                let mut inner_index = 0;
                let mut sum: u16 = 0;
                while inner_index <= index {
                    sum += value[inner_index];
                    inner_index += 1;
                }
                temp_leap_year_per_cycle[index] = sum;
                index += 1;
            }
        } else {
            temp_leap_year_per_cycle = accumulated_sub_cycles;
        }

        LeapSubCycle {
            accumulated_sub_cycles,
            accumulated_leap_years_per_cycle: temp_leap_year_per_cycle,
            has_sub_sub_cycle: !matches!(sub_cycles[0],0|1),
            leap_years_per_cycle: sum,
        }
    }
}

