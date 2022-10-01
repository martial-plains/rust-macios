use crate::{
    foundation::{ns_mutable_array::INSMutableArray, NSMutableArray, NSNumber, NSString, UInt},
    objective_c_runtime::traits::{FromId, PNSObject},
};

use super::{INSArray, NSArray};

/// An iterator for `Array`
#[derive(Debug)]
pub struct Iter<'a, T>
where
    T: PNSObject,
{
    pub(super) array: &'a NSArray<T>,
    pub(super) index: UInt,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: PNSObject + FromId,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.p_count() {
            None
        } else {
            let item = self.array.m_object_at_index(self.index);
            self.index += 1;
            Some(item)
        }
    }
}

impl<'a, T> FromIterator<&'a T> for NSArray<T>
where
    T: PNSObject + FromId + 'a,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        let mut mut_arr = NSMutableArray::tm_array_with_capacity(0);
        for item in iter {
            mut_arr.im_add_object(item);
        }

        NSArray::from(mut_arr)
    }
}

impl FromIterator<NSString> for NSArray<NSString> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = NSString>,
    {
        let mut mut_arr = NSMutableArray::tm_array_with_capacity(0);
        for item in iter {
            mut_arr.im_add_object(&item);
        }

        NSArray::from(mut_arr)
    }
}

impl FromIterator<NSNumber> for NSArray<NSNumber> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = NSNumber>,
    {
        let mut mut_arr = NSMutableArray::tm_array_with_capacity(0);
        for item in iter {
            mut_arr.im_add_object(&item);
        }

        NSArray::from(mut_arr)
    }
}
