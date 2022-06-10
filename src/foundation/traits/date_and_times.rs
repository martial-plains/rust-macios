use crate::{
    foundation::{ns_date::NSDate, NSComparisonResult, NSTimeInterval},
    objective_c_runtime::traits::PNSObject,
};

/// A representation of a specific point in time, independent of any calendar or time zone.
pub trait INSDate: PNSObject {
    /* Creating a Date
     */

    /// Creates and returns a new date object set to the current date and time.
    fn tm_date() -> Self;

    /// Creates and returns a date object set to a given number of seconds from the current date and time.
    fn tm_date_with_time_interval_since_now(time_interval: f64) -> Self;

    /// Creates and returns a date object set to a given number of seconds from the specified date.
    fn tm_date_with_time_interval_since_date(
        secs_to_be_added: NSTimeInterval,
        date: NSDate,
    ) -> Self;

    /// Creates and returns a date object set to a given number of seconds from 00:00:00 UTC on 1 January 2001.
    fn tm_date_with_time_interval_since_reference_date(secs_to_be_added: NSTimeInterval) -> Self;

    /// Creates and returns a date object set to the given number of seconds from 00:00:00 UTC on 1 January 1970.
    fn tm_date_with_time_interval_since1970(secs_to_be_added: NSTimeInterval) -> Self;

    /* Getting Temporal Boundaries
     */

    /// A date object representing a date in the distant future.
    fn tp_distant_future() -> Self;

    /// A date object representing a date in the distant past.
    fn tp_distant_past() -> Self;

    /* Retrieving the Current Date
     */

    /// The current date and time, as of the time of access.
    fn tp_now() -> Self;

    /* Comparing Dates
     */

    /// Returns a Boolean value that indicates whether a given object is a date that is exactly equal the receiver.
    fn im_is_equal_to_date(date: NSDate) -> bool;

    /// Returns the earlier of the receiver and another given date.
    fn im_earlier_date(date: NSDate) -> Self;

    /// Returns the later of the receiver and another given date.
    fn im_later_date(date: NSDate) -> Self;

    /// Indicates the temporal ordering of the receiver and another given date.
    fn im_compare(date: NSDate) -> NSComparisonResult;

    /* Getting Time Intervals
     */

    /// Returns the interval between the receiver and another given date.
    fn im_time_interval_since_date(date: NSDate) -> NSTimeInterval;

    /// The interval between the date object and the current date and time.
    fn ip_time_interval_since_now() -> NSTimeInterval;

    /// The interval between the date object and 00:00:00 UTC on 1 January 2001.
    fn tp_time_interval_since_reference_date() -> NSTimeInterval;

    /// The interval between the date object and 00:00:00 UTC on 1 January 1970.
    fn tp_time_interval_since_1970() -> NSTimeInterval;

    /* Adding Time Intervals
     */

    /// Returns a new date object that is set to a given number of seconds relative to the receiver.
    fn im_date_by_adding_time_interval(secs_to_be_added: NSTimeInterval) -> Self;
}
