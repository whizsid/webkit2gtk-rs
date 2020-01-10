// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use PrintCustomWidget;
use PrintOperationResponse;
use WebView;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_sys;

glib_wrapper! {
    pub struct PrintOperation(Object<webkit2_sys::WebKitPrintOperation, webkit2_sys::WebKitPrintOperationClass, PrintOperationClass>);

    match fn {
        get_type => || webkit2_sys::webkit_print_operation_get_type(),
    }
}

impl PrintOperation {
    pub fn new<P: IsA<WebView>>(web_view: &P) -> PrintOperation {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_print_operation_new(web_view.as_ref().to_glib_none().0))
        }
    }
}

pub const NONE_PRINT_OPERATION: Option<&PrintOperation> = None;

pub trait PrintOperationExt: 'static {
    fn get_page_setup(&self) -> Option<gtk::PageSetup>;

    fn get_print_settings(&self) -> Option<gtk::PrintSettings>;

    fn print(&self);

    fn run_dialog<P: IsA<gtk::Window>>(&self, parent: Option<&P>) -> PrintOperationResponse;

    fn set_page_setup(&self, page_setup: &gtk::PageSetup);

    fn set_print_settings(&self, print_settings: &gtk::PrintSettings);

    fn get_property_web_view(&self) -> Option<WebView>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_create_custom_widget<F: Fn(&Self) -> PrintCustomWidget + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_failed<F: Fn(&Self, &Error) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_print_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintOperation>> PrintOperationExt for O {
    fn get_page_setup(&self) -> Option<gtk::PageSetup> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_print_operation_get_page_setup(self.as_ref().to_glib_none().0))
        }
    }

    fn get_print_settings(&self) -> Option<gtk::PrintSettings> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_print_operation_get_print_settings(self.as_ref().to_glib_none().0))
        }
    }

    fn print(&self) {
        unsafe {
            webkit2_sys::webkit_print_operation_print(self.as_ref().to_glib_none().0);
        }
    }

    fn run_dialog<P: IsA<gtk::Window>>(&self, parent: Option<&P>) -> PrintOperationResponse {
        unsafe {
            from_glib(webkit2_sys::webkit_print_operation_run_dialog(self.as_ref().to_glib_none().0, parent.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    fn set_page_setup(&self, page_setup: &gtk::PageSetup) {
        unsafe {
            webkit2_sys::webkit_print_operation_set_page_setup(self.as_ref().to_glib_none().0, page_setup.to_glib_none().0);
        }
    }

    fn set_print_settings(&self, print_settings: &gtk::PrintSettings) {
        unsafe {
            webkit2_sys::webkit_print_operation_set_print_settings(self.as_ref().to_glib_none().0, print_settings.to_glib_none().0);
        }
    }

    fn get_property_web_view(&self) -> Option<WebView> {
        unsafe {
            let mut value = Value::from_type(<WebView as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"web-view\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_create_custom_widget<F: Fn(&Self) -> PrintCustomWidget + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"create-custom-widget\0".as_ptr() as *const _,
                Some(transmute(create_custom_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_failed<F: Fn(&Self, &Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"failed\0".as_ptr() as *const _,
                Some(transmute(failed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"finished\0".as_ptr() as *const _,
                Some(transmute(finished_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::page-setup\0".as_ptr() as *const _,
                Some(transmute(notify_page_setup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_print_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::print-settings\0".as_ptr() as *const _,
                Some(transmute(notify_print_settings_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn create_custom_widget_trampoline<P, F: Fn(&P) -> PrintCustomWidget + 'static>(this: *mut webkit2_sys::WebKitPrintOperation, f: glib_sys::gpointer) -> *mut webkit2_sys::WebKitPrintCustomWidget
where P: IsA<PrintOperation> {
    let f: &F = &*(f as *const F);
    f(&PrintOperation::from_glib_borrow(this).unsafe_cast()).to_glib_full()
}

unsafe extern "C" fn failed_trampoline<P, F: Fn(&P, &Error) + 'static>(this: *mut webkit2_sys::WebKitPrintOperation, error: *mut glib_sys::GError, f: glib_sys::gpointer)
where P: IsA<PrintOperation> {
    let f: &F = &*(f as *const F);
    f(&PrintOperation::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(error))
}

unsafe extern "C" fn finished_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitPrintOperation, f: glib_sys::gpointer)
where P: IsA<PrintOperation> {
    let f: &F = &*(f as *const F);
    f(&PrintOperation::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_page_setup_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitPrintOperation, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintOperation> {
    let f: &F = &*(f as *const F);
    f(&PrintOperation::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_print_settings_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitPrintOperation, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<PrintOperation> {
    let f: &F = &*(f as *const F);
    f(&PrintOperation::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for PrintOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PrintOperation")
    }
}
