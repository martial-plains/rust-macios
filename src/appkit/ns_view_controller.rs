use objc::{class, msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSBundle,
    objective_c_runtime::{id, traits::FromId},
};

use super::{object, INSResponder, INSView, NSNibName, NSView};

object! {
    /// A controller that manages a view, typically loaded from a nib file.
    unsafe pub struct NSViewController;
}

impl NSViewController {
    /// Creates a new view controller.
    pub fn new() -> Self {
        unsafe { Self::from_id(msg_send![class!(NSViewController), new]) }
    }
}

impl Default for NSViewController {
    fn default() -> Self {
        Self::new()
    }
}

impl INSResponder for NSViewController {}

#[interface_impl(NSResponder)]
impl NSViewController {
    /* Creating A View Controller
     */

    /// Returns a view controller object initialized to the nib file in the specified bundle.
    #[method]
    pub fn init_with_nib_name_bundle(nib_name: NSNibName, bundle: NSBundle) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let obj: id = msg_send![Self::m_class(), alloc];
            Self::from_id(msg_send![obj, initWithNibName: nib_name bundle: bundle])
        }
    }

    /// Instantiates a view from a nib file and sets the value of the view property.
    #[method]
    pub fn load_view(&self) {
        unsafe { msg_send![self.m_self(), loadView] }
    }

    /* Represented Object
     */

    /// Returns the represented object of the view controller.
    #[property]
    pub fn represented_object(&self) -> id {
        unsafe { msg_send![self.m_self(), representedObject] }
    }

    /* Nib Properties
     */

    /// The nib bundle to be loaded to instantiate the receiver’s primary view.
    #[property]
    pub fn nib_bundle(&self) -> NSBundle {
        unsafe { NSBundle::from_id(msg_send![self.m_self(), nibBundle]) }
    }

    /// The name of the nib file to be loaded to instantiate the receiver’s primary view.
    #[property]
    pub fn nib_name(&self) -> NSNibName {
        unsafe { NSNibName::from_id(msg_send![self.m_self(), nibName]) }
    }

    /* View Properties
     */

    /// The view controller’s primary view.
    #[property]
    pub fn view(&self) -> NSView {
        unsafe { NSView::from_id(msg_send![self.m_self(), view]) }
    }

    /// Sets the view controller’s primary view.
    ///
    /// # Arguments
    ///
    /// * `view` - The view to be set as the primary view.
    #[property]
    pub fn set_view<V>(&self, view: V)
    where
        V: INSView,
    {
        unsafe { msg_send![self.m_self(), setView: view] }
    }

    /// The localized title of the receiver’s primary view.
    #[property]
    pub fn title(&self) -> id {
        unsafe { msg_send![self.m_self(), title] }
    }

    /// Sets the localized title of the receiver’s primary view.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to be set as the primary view.
    #[property]
    pub fn set_title(&self, title: id) {
        unsafe { msg_send![self.m_self(), setTitle: title] }
    }

    /* Responding to View Events
     */

    /// Called after the view controller’s view has been loaded into memory.
    #[property]
    pub fn view_did_load(&self) {
        unsafe { msg_send![self.m_self(), viewDidLoad] }
    }
}
