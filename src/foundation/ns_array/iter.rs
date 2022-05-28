use crate::{
    foundation::{
        traits::{INSArray, INSMutableArray},
        NSMutableArray, NSNumber, NSString, UInt,
    },
    objective_c_runtime::traits::PNSObject,
};

use super::NSArray;

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
    T: PNSObject,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.count() {
            None
        } else {
            let item = self.array.objectAt(self.index);
            self.index += 1;
            Some(item)
        }
    }
}

impl<'a, T> FromIterator<&'a T> for NSArray<T>
where
    T: PNSObject + 'a,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        let mut mut_arr = NSMutableArray::new();
        for item in iter {
            mut_arr.add(item);
        }

        NSArray::from(mut_arr)
    }
}

impl FromIterator<NSString> for NSArray<NSString> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = NSString>,
    {
        let mut mut_arr = NSMutableArray::new();
        for item in iter {
            mut_arr.add(&item);
        }

        NSArray::from(mut_arr)
    }
}

impl FromIterator<NSNumber> for NSArray<NSNumber> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = NSNumber>,
    {
        let mut mut_arr = NSMutableArray::new();
        for item in iter {
            mut_arr.add(&item);
        }

        NSArray::from(mut_arr)
    }
}
