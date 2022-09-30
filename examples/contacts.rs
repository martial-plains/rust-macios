use rust_macios::{
    contacts::{
        CNContactStore, CNLabelHome, CNLabelWork, CNLabeledValue, CNMutableContact,
        CNMutablePostalAddress, CNSaveRequest,
    },
    foundation::{macros::nsarray, NSError, NSString},
    objective_c_runtime::traits::PNSObject,
};

fn main() -> Result<(), NSError> {
    // Create a new mutable contact (read/write)
    let mut contact = CNMutableContact::m_new();

    // Set standard properties
    contact.set_given_name("John".into());
    contact.set_family_name("Appleseed".into());

    // Add email addresses
    let home_email = CNLabeledValue::<NSString>::labeled_value_with_label_value(
        unsafe { &CNLabelHome },
        "john.appleseed@mac.com".into(),
    );
    let work_email = CNLabeledValue::<NSString>::labeled_value_with_label_value(
        unsafe { &CNLabelWork },
        "john.appleseed@apple.com".into(),
    );

    contact.set_email_addresses(nsarray![home_email, work_email]);

    // Add work address
    let mut work_address = CNMutablePostalAddress::m_new();

    work_address.set_street("1 Infinite Loop".into());
    work_address.set_city("Cupertino".into());
    work_address.set_state("CA".into());
    work_address.set_postal_code("95014".into());

    contact.set_postal_addresses(nsarray![CNLabeledValue::labeled_value_with_label_value(
        unsafe { &CNLabelWork.clone() },
        work_address,
    )]);

    // Save new contact
    let store = CNContactStore::m_new();
    let save_request = CNSaveRequest::m_new();

    save_request.add_contact_to_container_with_identifier(contact, None);

    if store.execute_save_request(save_request)? {
        println!("New contact saved!");
    }

    Ok(())
}
