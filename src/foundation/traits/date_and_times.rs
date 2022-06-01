use crate::{
    foundation::{ns_date::NSDate, NSComparisonResult, NSTimeInterval},
    objective_c_runtime::traits::PNSObject,
};

/// A representation of a specific point in time, independent of any calendar or time zone.
pub trait INSDate: PNSObject {
    /* Creating a Date
     */

    /// Creates and returns a new date object set to the current date and time.
    fn date() -> Self;

    /// Creates and returns a date object set to a given number of seconds from the current date and time.
    fn dateWithTimeIntervalSinceNow(timeInterval: f64) -> Self;

    /// Creates and returns a date object set to a given number of seconds from the specified date.
    fn dateWithTimeIntervalSinceDate(secsToBeAdded: NSTimeInterval, date: NSDate) -> Self;

    /// Creates and returns a date object set to a given number of seconds from 00:00:00 UTC on 1 January 2001.
    fn dateWithTimeIntervalSinceReferenceDate(secsToBeAdded: NSTimeInterval) -> Self;

    /// Creates and returns a date object set to the given number of seconds from 00:00:00 UTC on 1 January 1970.
    fn dateWithTimeIntervalSince1970(secsToBeAdded: NSTimeInterval) -> Self;

    /* Getting Temporal Boundaries
     */

    /// A date object representing a date in the distant future.
    fn distantFuture() -> Self;

    /// A date object representing a date in the distant past.
    fn distantPast() -> Self;

    /* Retrieving the Current Date
     */

    /// The current date and time, as of the time of access.
    fn now() -> Self;

    /* Comparing Dates
     */

    /// Returns a Boolean value that indicates whether a given object is a date that is exactly equal the receiver.
    fn isEqualToDate(date: NSDate) -> bool;

    /// Returns the earlier of the receiver and another given date.
    fn earlierDate(date: NSDate) -> Self;

    /// Returns the later of the receiver and another given date.
    fn laterDate(date: NSDate) -> Self;

    /// Indicates the temporal ordering of the receiver and another given date.
    fn compare(date: NSDate) -> NSComparisonResult;

    /* Getting Time Intervals
     */

    /// Returns the interval between the receiver and another given date.
    fn timeIntervalSinceDate(date: NSDate) -> NSTimeInterval;

    /// The interval between the date object and the current date and time.
    fn timeIntervalSinceNow() -> NSTimeInterval;

    /// The interval between the date object and 00:00:00 UTC on 1 January 2001.
    fn timeIntervalSinceReferenceDate() -> NSTimeInterval;

    /// The interval between the date object and 00:00:00 UTC on 1 January 1970.
    fn timeIntervalSince1970() -> NSTimeInterval;

    /* Adding Time Intervals
     */

    /// Returns a new date object that is set to a given number of seconds relative to the receiver.
    fn dateByAddingTimeInterval(secsToBeAdded: NSTimeInterval) -> Self;
}
