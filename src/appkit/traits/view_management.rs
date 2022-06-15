use objc::{msg_send, sel, sel_impl};

use crate::{
    appkit::NSNibName,
    foundation::NSBundle,
    objective_c_runtime::{id, traits::FromId},
};

use super::{INSResponder, INSView};

/// A controller that manages a view, typically loaded from a nib file.
pub trait INSViewController: INSResponder {
    /* Creating A View Controller
     */

    /// Returns a view controller object initialized to the nib file in the specified bundle.
    fn im_init_with_nib_name_bundle(nib_name: NSNibName, bundle: NSBundle) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let obj: id = msg_send![Self::im_class(), alloc];
            Self::from_id(msg_send![obj, initWithNibName: nib_name bundle: bundle])
        }
    }

    /// Instantiates a view from a nib file and sets the value of the view property.
    fn im_load_view(&self) {
        unsafe { msg_send![self.im_self(), loadView] }
    }

    /* Represented Object
     */

    /// Returns the represented object of the view controller.
    fn ip_represented_object(&self) -> id {
        unsafe { msg_send![self.im_self(), representedObject] }
    }

    /* Nib Properties
     */

    /// The nib bundle to be loaded to instantiate the receiver’s primary view.
    fn ip_nib_bundle(&self) -> NSBundle {
        unsafe { NSBundle::from_id(msg_send![self.im_self(), nibBundle]) }
    }

    /// The name of the nib file to be loaded to instantiate the receiver’s primary view.
    fn ip_nib_name(&self) -> NSNibName {
        unsafe { NSNibName::from_id(msg_send![self.im_self(), nibName]) }
    }

    /* View Properties
     */

    /// The view controller’s primary view.
    fn ip_view(&self) -> id {
        unsafe { msg_send![self.im_self(), view] }
    }

    /// Sets the view controller’s primary view.
    ///
    /// # Arguments
    ///
    /// * `view` - The view to be set as the primary view.
    fn ip_set_view<V>(&self, view: V)
    where
        V: INSView,
    {
        unsafe { msg_send![self.im_self(), setView: view] }
    }

    /// The localized title of the receiver’s primary view.
    fn ip_title(&self) -> id {
        unsafe { msg_send![self.im_self(), title] }
    }

    /// Sets the localized title of the receiver’s primary view.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to be set as the primary view.
    fn ip_set_title(&self, title: id) {
        unsafe { msg_send![self.im_self(), setTitle: title] }
    }
}
