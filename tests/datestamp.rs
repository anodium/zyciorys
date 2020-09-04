use zyciorys::resume::datestamp::Datestamp;

#[test]
pub fn test_parser() {
    test_parser_success();
    test_parser_failure();
}

// TODO: Implement test
#[test]
pub fn test_parser_success() {
    for case in TEST_CASES_SUCESS {
        todo!();
    }
}

// TODO: Implement test
#[test]
pub fn test_parser_failure() {
    for case in TEST_CASES_FAILURE {
        todo!();
    }
}

// TODO: Add partial success cases
const TEST_CASES_SUCESS: [(&str, Datestamp)] = [
    // NOTE: Test cases adapted from this query
    // https://www.random.org/calendar-dates/?num=12&start_day=15&start_month=10&start_year=1582&end_day=31&end_month=12&end_year=3000&mondays=on&tuesdays=on&wednesdays=on&thursdays=on&fridays=on&saturdays=on&sundays=on&display=2&format=plain&rnd=date.2020-09-03

    ("1734-08-20",          YearMonthDay(1734, 8, 20)),
    ("1862/05/23",          YearMonthDay(1862, 5, 23)),
    ("2112 June 12",        YearMonthDay(2112, 6, 12)),
    ("2255 Feb 28",         YearMonthDay(2255, 2, 28)),
    ("10-08-2311",          YearMonthDay(2311, 8, 10)),
    ("5/11/2368",           YearMonthDay(2368, 11, 5)),
    ("5 January 2417",      YearMonthDay(2417, 1, 5)),
    ("6 Apr 2559",          YearMonthDay(2559, 4, 6)),
    ("9-26-2568",           YearMonthDay(2568, 9, 26)),
    ("5/20/2690",           YearMonthDay(2690, 5, 20)),
    ("February 19, 2952",   YearMonthDay(2952, 2, 19)),
    ("Jan 6 2970",          YearMonthDay(2970, 1, 6))
];

// TODO: Add failure cases
const TEST_CASES_FAILURE: [(&str, Datestamp)] = [
    ("", Year(0)),
    ("", Year(0)),
];
