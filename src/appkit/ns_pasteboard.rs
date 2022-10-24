use objc::{msg_send, runtime::Class, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use super::{ns_pasteboard_item::NSPasteboardItem, object};

use crate::{
    foundation::{Int, NSArray, NSData, NSDictionary, NSFileWrapper, NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
    utils::{to_bool, to_optional},
};

pub type NSPasteboardType = NSString;

extern "C" {
    /* Pasteboard Types
     */

    /// URL data for one file or resource.
    pub static NSPasteboardTypeURL: NSPasteboardType;

    /// Color data.
    pub static NSPasteboardTypeColor: NSPasteboardType;

    /// A representation of a file’s contents.
    pub static NSFileContentsPboardType: NSPasteboardType;

    /// A file URL.
    pub static NSPasteboardTypeFileURL: NSPasteboardType;

    /// Type for the find panel metadata property list.
    pub static NSFindPanelSearchOptionsPboardType: NSPasteboardType;

    /// Font and character information.
    pub static NSPasteboardTypeFont: NSPasteboardType;

    /// Type for HTML content.
    pub static NSPasteboardTypeHTML: NSPasteboardType;

    /// Multiple text selection.
    pub static NSPasteboardTypeMultipleTextSelection: NSPasteboardType;

    /// PDF data.
    pub static NSPasteboardTypePDF: NSPasteboardType;

    /// PNG image data.
    pub static NSPasteboardTypePNG: NSPasteboardType;

    /// Rich Text Format (RTF) data.
    pub static NSPasteboardTypeRTF: NSPasteboardType;

    /// RTFD formatted file contents.
    pub static NSPasteboardTypeRTFD: NSPasteboardType;

    /// Paragraph formatting information.
    pub static NSPasteboardTypeRuler: NSPasteboardType;

    /// Sound data.
    pub static NSPasteboardTypeSound: NSPasteboardType;

    /// String data.
    pub static NSPasteboardTypeString: NSPasteboardType;

    /// Tab-separated fields of text.
    pub static NSPasteboardTypeTabularText: NSPasteboardType;

    /// Type for the Find panel metadata property list.
    pub static NSPasteboardTypeTextFinderOptions: NSPasteboardType;

    /// Tag Image File Format (TIFF) data.
    pub static NSPasteboardTypeTIFF: NSPasteboardType;

}

/// Search options for the find panel.
pub type NSPasteboardTypeFindPanelSearchOptionKey = NSString;

extern "C" {
    /// A Boolean value indicating whether the search is case-insensitive.
    pub static NSFindPanelCaseInsensitiveSearch: NSPasteboardTypeFindPanelSearchOptionKey;

    /// A number object containing the match type to use in the find panel.
    pub static NSFindPanelSubstringMatch: NSPasteboardTypeFindPanelSearchOptionKey;
}

/// Search options for text in Finder.
pub type NSPasteboardTypeTextFinderOptionKey = NSString;

extern "C" {
    /// A Boolean value indicating whether the search is case insensitive.
    pub static NSTextFinderCaseInsensitiveKey: NSPasteboardTypeTextFinderOptionKey;

    /// A number object containing the match type to use.
    pub static NSTextFinderMatchingTypeKey: NSPasteboardTypeTextFinderOptionKey;
}

/// Constants that represent the standard pasteboard names.
pub type NSPasteboardName = NSString;

extern "C" {
    /* Named Pasteboards
     */

    /// The pasteboard that stores data to move as the result of a drag operation.
    pub static NSPasteboardNameDrag: NSPasteboardName;

    /// The pasteboard that holds information about the current state of the active application’s find panel.
    pub static NSPasteboardNameFind: NSPasteboardName;

    /// The pasteboard that holds font and character information and supports Copy Font and Paste Font commands that the text editor may implement.
    pub static NSPasteboardNameFont: NSPasteboardName;

    /// The pasteboard you use to perform ordinary cut, copy, and paste operations.
    pub static NSNSPasteboardNameGeneral: NSPasteboardName;

    /// The pasteboard that holds information about paragraph formats and supports the Copy Ruler and Paste Ruler commands that the text editor may implement.
    pub static NSPasteboardNameRuler: NSPasteboardName;
}

/// Options for reading pasteboard data.
pub type NSPasteboardReadingOptionKey = NSString;

extern "C" {
    /// Option for reading URLs to restrict the results to URLs with contents that conform to any of the provided UTI types.
    pub static NSPasteboardURLReadingContentsConformToTypesKey: NSPasteboardReadingOptionKey;

    pub static staticNSPasteboardURLReadingFileURLsOnlyKey: NSPasteboardReadingOptionKey;
}

/// Options that specify how to interpret data on the pasteboard when initializing pasteboard data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum NSPasteboardReadingOptions {
    AsData = 0,
    AsString = 1 << 0,
    AsPropertyList = 1 << 1,
    AsKeyedArchive = 1 << 2,
}

/// Options for preparing the pasteboard.
#[repr(u32)]
pub enum NSPasteboardContentsOptions {
    CurrentHostOnly = 1,
}

object! {
    /// An object that transfers data to and from the pasteboard server.
    unsafe pub struct NSPasteboard;
}

#[interface_impl(NSObject)]
impl NSPasteboard {
    /* Creating and Releasing a Pasteboard
     */

    /// The shared pasteboard object to use for general content.
    #[property]
    pub fn general_pasteboard() -> NSPasteboard {
        unsafe { NSPasteboard::from_id(msg_send![Self::m_class(), generalPasteboard]) }
    }

    /// Creates a new pasteboard object that supplies the specified data in as many types as possible based on the available filter services.
    #[method]
    pub fn pasteboard_by_filtering_data_of_type(
        data: NSData,
        r#type: NSPasteboardType,
    ) -> NSPasteboard {
        unsafe {
            NSPasteboard::from_id(
                msg_send![Self::m_class(), pasteboardByFilteringData: data ofType: r#type],
            )
        }
    }

    /// Creates a new pasteboard object that supplies the specified file in as many types as possible based on the available filter services.
    #[method]
    pub fn pasteboard_by_filtering_file(filename: NSString) -> NSPasteboard {
        unsafe {
            NSPasteboard::from_id(msg_send![
                Self::m_class(),
                pasteboardByFilteringFile: filename
            ])
        }
    }

    /// Creates a new pasteboard object that supplies the specified pasteboard data in as many types as possible based on the available filter services.
    #[method]
    pub fn pasteboard_by_filtering_types_in_pasteboard(pboard: NSPasteboard) -> NSPasteboard {
        unsafe {
            NSPasteboard::from_id(msg_send![
                Self::m_class(),
                pasteboardByFilteringTypesInPasteboard: pboard
            ])
        }
    }

    /// Returns the pasteboard with the specified name.
    #[method]
    pub fn pasteboard_with_name(name: NSPasteboardName) -> NSPasteboard {
        unsafe { NSPasteboard::from_id(msg_send![Self::m_class(), pasteboardWithName: name]) }
    }

    /// Creates and returns a new pasteboard with a name that is guaranteed to be unique with respect to other pasteboards in the system.
    #[method]
    pub fn pasteboard_with_unique_name() -> NSPasteboard {
        unsafe { NSPasteboard::from_id(msg_send![Self::m_class(), pasteboardWithUniqueName]) }
    }

    /// Releases the receiver’s resources in the pasteboard server.
    #[method]
    pub fn release_globally(&self) {
        unsafe { msg_send![self.m_self(), releaseGlobally] }
    }

    /* Writing Data
     */

    /// Clears the existing contents of the pasteboard.
    #[method]
    pub fn clear_contents(&mut self) -> Int {
        unsafe { msg_send![self.m_self(), clearContents] }
    }

    /// Writes an array of objects to the receiver.
    #[method]
    pub fn write_objects(&mut self, objects: NSArray<id>) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), writeObjects: objects]) }
    }

    /// Sets the data as the representation for the specified type for the first item on the receiver.
    #[method]
    pub fn set_data_for_type(&mut self, data: NSData, data_type: NSPasteboardType) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), setData: data forType: data_type]) }
    }

    /// Sets the given property list as the representation for the specified type for the first item on the receiver.
    #[method]
    pub fn set_property_list_for_type(&mut self, plist: id, data_type: NSPasteboardType) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), setPropertyList: plist forType: data_type]) }
    }

    /// Sets the given string as the representation for the specified type for the first item on the receiver.
    #[method]
    pub fn set_string_for_type(&mut self, string: NSString, data_type: NSPasteboardType) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), setString: string forType: data_type]) }
    }

    /* Reading Data
     */

    /// Reads from the receiver objects that best match the specified array of classes.
    #[method]
    pub fn read_objects_for_classes_options(
        &self,
        class_array: NSArray<Class>,
        options: NSDictionary<NSPasteboardReadingOptionKey, id>,
    ) -> Option<NSArray<id>> {
        unsafe {
            to_optional(
                msg_send![self.m_self(), readObjectsForClasses: class_array options: options],
            )
        }
    }

    /// An array that contains all the items held by the pasteboard.
    #[property]
    pub fn pasteboard_items(&self) -> Option<NSArray<NSPasteboardItem>> {
        unsafe { to_optional(msg_send![self.m_self(), pasteboardItems]) }
    }

    /// Returns the index of the specified pasteboard item.
    #[method]
    pub fn index_of_pasteboard_item(&self, pasteboard_item: NSPasteboardItem) -> UInt {
        unsafe { msg_send![self.m_self(), indexOfPasteboardItem: pasteboard_item] }
    }

    /// Returns the data for the specified type from the first item in the receiver that contains the type.
    #[method]
    pub fn data_for_type(&self, data_type: NSPasteboardType) -> Option<NSData> {
        unsafe { to_optional(msg_send![self.m_self(), dataForType: data_type]) }
    }

    /// Returns the property list for the specified type from the first item in the receiver that contains the type.
    #[method]
    pub fn property_list_for_type(&self, data_type: NSPasteboardType) -> Option<id> {
        unsafe {
            let ptr: id = msg_send![self.m_self(), propertyListForType: data_type];

            if ptr.is_null() {
                None
            } else {
                Some(ptr)
            }
        }
    }

    /// Returns a concatenation of the strings for the specified type from all the items in the receiver that contain the type.
    #[method]
    pub fn string_for_type(&self, data_type: NSPasteboardType) -> Option<NSString> {
        unsafe { to_optional(msg_send![self.m_self(), stringForType: data_type]) }
    }

    /// Scans the specified types for a type that the receiver supports.
    #[method]
    pub fn available_type_from_array(
        &self,
        types: NSArray<NSPasteboardType>,
    ) -> Option<NSPasteboardType> {
        unsafe { to_optional(msg_send![self.m_self(), availableTypeFromArray: types]) }
    }

    /// Returns a Boolean value that indicates whether the receiver contains any items that conform to the specified UTIs.
    #[method]
    pub fn can_read_item_with_data_conforming_to_types(&self, types: NSArray<NSString>) -> bool {
        unsafe {
            to_bool(msg_send![
                self.m_self(),
                canReadItemWithDataConformingToTypes: types
            ])
        }
    }

    /// Returns a Boolean value that indicates whether the receiver contains any items that can be represented as an instance of any class in a given array.
    #[method]
    pub fn can_read_object_for_classes_options(
        &self,
        class_array: NSArray<Class>,
        options: NSDictionary<NSPasteboardReadingOptionKey, id>,
    ) -> bool {
        unsafe {
            to_bool(msg_send![self.m_self(), canReadObjectForClasses: class_array options: options])
        }
    }

    /// An array of the receiver’s supported data types.
    #[property]
    pub fn types(&self) -> Option<NSArray<NSPasteboardType>> {
        unsafe { to_optional(msg_send![self.m_self(), types]) }
    }

    /// Returns the data types that can be converted to the specified type using the available filter services.
    #[method]
    pub fn types_filterable_to(r#type: NSPasteboardType) -> NSArray<NSPasteboardType> {
        unsafe { NSArray::from_id(msg_send![Self::m_class(), typesFilterableTo: r#type]) }
    }

    /* Preparing the Pasteboard for Content
     */

    /// Prepares the pasteboard to receive new contents, removing the existing pasteboard contents.
    #[method]
    pub fn prepare_for_new_contents_with_options(
        &self,
        options: NSPasteboardContentsOptions,
    ) -> Int {
        unsafe { msg_send![self.m_self(), prepareForNewContentsWithOptions: options] }
    }

    /* Getting Information about a Pasteboard
     */

    /// The receiver’s name.
    #[property]
    pub fn name(&self) -> NSPasteboardName {
        unsafe { NSPasteboardName::from_id(msg_send![self.m_self(), name]) }
    }

    /// The receiver’s change count.
    #[property]
    pub fn change_count(&self) -> Int {
        unsafe { msg_send![self.m_self(), changeCount] }
    }

    /*  Writing Data (macOS 10.5 and Earlier)
     */

    /// Prepares the receiver for a change in its contents by declaring the new types of data it will contain and a new owner.
    #[method]
    pub fn declare_types_owner(&self, new_types: NSArray<NSPasteboardType>, new_owner: id) -> Int {
        unsafe { msg_send![self.m_self(), declareTypes: new_types owner: new_owner ] }
    }

    /// Adds promises for the specified types to the first pasteboard item.
    #[method]
    pub fn add_types_owner(&mut self, new_types: NSArray<NSPasteboardType>, new_owner: id) -> Int {
        unsafe { msg_send![self.m_self(), addTypes: new_types owner: new_owner] }
    }

    /// Writes the contents of the specified file to the pasteboard.
    #[method]
    pub fn write_file_contents(&mut self, filename: NSString) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), writeFileContents: filename]) }
    }

    /// Writes the serialized contents of the specified file wrapper to the pasteboard.
    #[method]
    pub fn write_file_wrapper(&mut self, wrapper: NSFileWrapper) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), writeFileWrapper: wrapper]) }
    }

    /* Reading Data (macOS 10.5 and Earlier)
     */

    /// Reads data representing a file’s contents from the receiver and writes it to the specified file.
    #[method]
    pub fn read_file_contents_type_to_file(
        &self,
        r#type: NSPasteboardType,
        filename: NSString,
    ) -> Option<NSString> {
        unsafe {
            to_optional(msg_send![
                self.m_self(),
                readFileContentsType: r#type
                toFile: filename
            ])
        }
    }

    /// Reads data representing a file’s contents from the receiver and returns it as a file wrapper.
    #[method]
    pub fn read_file_wrapper(&self) -> Option<NSFileWrapper> {
        unsafe { to_optional(msg_send![self.m_self(), readFileWrapper]) }
    }
}
