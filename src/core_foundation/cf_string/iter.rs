use crate::{
    core_foundation::{CFIndex, CFTypeObject},
    kernel::UniChar,
};

use super::CFString;

pub struct Iter<'a> {
    pub(super) string: &'a CFString,
    pub(super) index: CFIndex,
}

impl<'a> Iterator for Iter<'a> {
    type Item = UniChar;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.index >= CFString::get_length(self.string.get_internal_object()) {
                None
            } else {
                let item =
                    CFString::get_character_at_index(self.string.get_internal_object(), self.index);
                self.index += 1;
                Some(item)
            }
        }
    }
}
