/// The entities the user can grant access to.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNEntityType {
    /// The user's contacts.
    Contacts,
}

/// An authorization status the user can grant for an app to access the specified entity type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNAuthorizationStatus {
    /// The user has not yet made a choice regarding whether the application may access contact data.
    NotDetermined,
    /// The application is not authorized to access contact data. The user cannot change this application’s status, possibly due to active restrictions such as parental controls being in place.
    Restricted,
    /// The application is authorized to access contact data.
    Authorized,
    /// The user explicitly denied access to contact data for the application.
    Denied,
}

/// The types a contact can be.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNContactType {
    /// A person.
    Person,
    /// A organization.
    Organization,
}

/// Indicates the sorting order for contacts.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNContactSortOrder {
    /// No sorting order.
    None,
    /// The user’s default sorting order.
    UserDefault,
    /// Sorting contacts by given name.
    GivenName,
    /// Sorting contacts by family name.
    FamilyName,
}

/// The container may be local on the device or associated with a server account that has contacts.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum CNContainerType {
    ///
    Unassigned = 0,
    /// A container for contacts only stored locally on the device. There is only one local container for a device.
    Local,
    /// A container for contacts stored in an Exchange folder from an Exchange server.
    Exchange,
    /// A container for contacts stored in an CardDAV server, such as iCloud.
    CardDav,
}
