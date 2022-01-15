#include <new>

typedef int64_t YearType;
typedef int64_t DateItemMaxType;
typedef int64_t AbsoluteType;
typedef int64_t YearType;

struct DateListResultWithoutSecond {
    YearType year;
    uint8_t month;
    uint8_t day;
    uint8_t hour;
    uint8_t minute;
};

struct MonthView {
    uint8_t offset;
    uint8_t slot_length;
};

struct YearView {
    uint8_t offset;
    uint8_t slot_1_start;
    uint8_t slot_1_length;
    uint8_t slot_2_length;
    uint8_t slot_3_length;
    uint8_t slot_4_length;
    uint8_t slot_5_length;
    uint8_t slot_6_length;
    uint8_t slot_7_length;
    uint8_t slot_8_length;
    uint8_t slot_9_length;
    uint8_t slot_10_length;
    uint8_t slot_11_length;
    uint8_t slot_12_length;
};                        

extern "C" {

DateListResultWithoutSecond date_list_now();

DateListResultWithoutSecond second_to_date_list(AbsoluteType absolute_value);

DateListResultWithoutSecond unix_second_to_date_list(AbsoluteType absolute_value);

DateListResultWithoutSecond julian_second_to_date_list(AbsoluteType absolute_value);

DateListResultWithoutSecond date_list_to_date_list(DateItemMaxType year,
    DateItemMaxType month,
    DateItemMaxType day,
    DateItemMaxType hour,
    DateItemMaxType minute);

double absolute_time_now();

double unix_time_now();

double julian_time_now();

AbsoluteType date_list_to_second(DateItemMaxType year,
    DateItemMaxType month,
    DateItemMaxType day,
    DateItemMaxType hour,
    DateItemMaxType minute);

AbsoluteType julian_date_list_to_second(DateItemMaxType year,
    DateItemMaxType month,
    DateItemMaxType day,
    DateItemMaxType hour,
    DateItemMaxType minute);

AbsoluteType unix_date_list_to_second(DateItemMaxType year,
    DateItemMaxType month,
    DateItemMaxType day,
    DateItemMaxType hour,
    DateItemMaxType minute);

bool is_holiday_with_correction(DateItemMaxType year, DateItemMaxType month, DateItemMaxType day);


DateListResultWithoutSecond arithmetic_persian_second_to_date_list(uint8_t cycle_id, AbsoluteType absolute_value);

AbsoluteType arithmetic_persian_date_list_to_second(uint8_t cycle_id,
    DateItemMaxType year,
    DateItemMaxType month,
    DateItemMaxType day,
    DateItemMaxType hour,
    DateItemMaxType minute);

// DateListResultWithoutSecond astronomical_persian_second_to_date_list(AbsoluteType absolute_value);
//
// AbsoluteType astronomical_persian_date_list_to_second(DateItemMaxType year,
//    DateItemMaxType month,
//    DateItemMaxType day,
//    DateItemMaxType hour,
//    DateItemMaxType minute);


DateListResultWithoutSecond islamic_second_to_date_list(uint8_t cycle_id, AbsoluteType absolute_value);

AbsoluteType islamic_date_list_to_second(uint8_t cycle_id,
    DateItemMaxType year,
    DateItemMaxType month,
    DateItemMaxType day,
    DateItemMaxType hour,
    DateItemMaxType minute);

bool gregorian_is_leap_year(YearType year);
bool arithmetic_persian_is_leap_year(uint8_t cycle_id, YearType year);
bool islamic_is_leap_year(uint8_t cycle_id, YearType year);
// bool astronomical_persian_is_leap_year(AbsoluteType absolute_value);

MonthView gergorian_month_view(AbsoluteType date);
YearView gergorian_year_view(AbsoluteType start, AbsoluteType end);


MonthView arithmetic_persian_month_view(uint8_t cycle_id, AbsoluteType date);
YearView arithmetic_persian_year_view(uint8_t cycle_id, AbsoluteType start, AbsoluteType end);

// MonthView astronomical_persian_month_view(AbsoluteType date);
// YearView astronomical_persian_year_view(AbsoluteType start, AbsoluteType end);

MonthView islamic_month_view(uint8_t cycle_id, AbsoluteType date);
YearView islamic_year_view(uint8_t cycle_id, AbsoluteType start, AbsoluteType end);



} // extern "C"


