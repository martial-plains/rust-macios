use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::{ns_date::NSDate, NSComparisonResult, NSTimeInterval},
    objective_c_runtime::traits::{FromId, PNSObject},
    utils::to_bool,
};

/// A representation of a specific point in time, independent of any calendar or time zone.
pub trait INSDate: PNSObject {
    /* Creating a Date
     */

    /// Creates and returns a new date object set to the current date and time.
    fn tm_date() -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::im_class(), date]) }
    }

    /// Creates and returns a date object set to a given number of seconds from the current date and time.
    fn tm_date_with_time_interval_since_now(time_interval: NSTimeInterval) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                dateWithTimeIntervalSinceNow: time_interval
            ])
        }
    }

    /// Creates and returns a date object set to a given number of seconds from the specified date.
    fn tm_date_with_time_interval_since_date(secs_to_be_added: NSTimeInterval, date: NSDate) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                dateWithTimeInterval: secs_to_be_added
                sinceDate: date
            ])
        }
    }

    /// Creates and returns a date object set to a given number of seconds from 00:00:00 UTC on 1 January 2001.
    fn tm_date_with_time_interval_since_reference_date(secs_to_be_added: NSTimeInterval) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                dateWithTimeIntervalSinceReferenceDate: secs_to_be_added
            ])
        }
    }

    /// Creates and returns a date object set to the given number of seconds from 00:00:00 UTC on 1 January 1970.
    fn tm_date_with_time_interval_since1970(secs_to_be_added: NSTimeInterval) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::im_class(),
                dateWithTimeIntervalSince1970: secs_to_be_added
            ])
        }
    }

    /* Getting Temporal Boundaries
     */

    /// A date object representing a date in the distant future.
    fn tp_distant_future() -> NSDate {
        unsafe { NSDate::from_id(msg_send![Self::im_class(), distantFuture]) }
    }

    /// A date object representing a date in the distant past.
    fn tp_distant_past() -> NSDate {
        unsafe { NSDate::from_id(msg_send![Self::im_class(), distantPast]) }
    }

    /* Retrieving the Current Date
     */

    /// The current date and time, as of the time of access.
    fn tp_now() -> NSDate {
        unsafe { NSDate::from_id(msg_send![Self::im_class(), now]) }
    }

    /* Comparing Dates
     */

    /// Returns a Boolean value that indicates whether a given object is a date that is exactly equal the receiver.
    fn im_is_equal_to_date(&self, date: NSDate) -> bool {
        unsafe { to_bool(msg_send![self.im_self(), isEqualToDate: date]) }
    }

    /// Returns the earlier of the receiver and another given date.
    fn im_earlier_date(&self, date: NSDate) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), earlierDate: date]) }
    }

    /// Returns the later of the receiver and another given date.
    fn im_later_date(&self, date: NSDate) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), laterDate: date]) }
    }

    /// Indicates the temporal ordering of the receiver and another given date.
    fn im_compare(&self, date: NSDate) -> NSComparisonResult {
        unsafe { msg_send![self.im_self(), compare: date] }
    }

    /* Getting Time Intervals
     */

    /// Returns the interval between the receiver and another given date.
    fn im_time_interval_since_date(&self, date: NSDate) -> NSTimeInterval {
        unsafe { msg_send![self.im_self(), timeIntervalSinceDate: date] }
    }

    /// The interval between the date object and the current date and time.
    fn ip_time_interval_since_now(&self) -> NSTimeInterval {
        unsafe { msg_send![self.im_self(), timeIntervalSinceNow] }
    }

    /// The interval between the date object and 00:00:00 UTC on 1 January 2001.
    fn tp_time_interval_since_reference_date() -> NSTimeInterval {
        unsafe { msg_send![Self::im_class(), timeIntervalSinceReferenceDate] }
    }

    /// The interval between the date object and 00:00:00 UTC on 1 January 1970.
    fn tp_time_interval_since_1970() -> NSTimeInterval {
        unsafe { msg_send![Self::im_class(), timeIntervalSince1970] }
    }

    /* Adding Time Intervals
     */

    /// Returns a new date object that is set to a given number of seconds relative to the receiver.
    fn im_date_by_adding_time_interval(&self, secs_to_be_added: NSTimeInterval) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                self.im_self(),
                dateByAddingTimeInterval: secs_to_be_added
            ])
        }
    }
}
