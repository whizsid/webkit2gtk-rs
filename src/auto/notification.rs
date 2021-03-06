// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_8", feature = "dox"))]
use std::mem::transmute;
use webkit2_sys;

glib_wrapper! {
    pub struct Notification(Object<webkit2_sys::WebKitNotification, webkit2_sys::WebKitNotificationClass, NotificationClass>);

    match fn {
        get_type => || webkit2_sys::webkit_notification_get_type(),
    }
}

pub const NONE_NOTIFICATION: Option<&Notification> = None;

pub trait NotificationExt: 'static {
    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn clicked(&self);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn close(&self);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_body(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_id(&self) -> u64;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_tag(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_title(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_property_body_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_tag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Notification>> NotificationExt for O {
    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn clicked(&self) {
        unsafe {
            webkit2_sys::webkit_notification_clicked(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn close(&self) {
        unsafe {
            webkit2_sys::webkit_notification_close(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_body(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_notification_get_body(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_id(&self) -> u64 {
        unsafe { webkit2_sys::webkit_notification_get_id(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_tag(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_notification_get_tag(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_notification_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clicked_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitNotification,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Notification>,
        {
            let f: &F = &*(f as *const F);
            f(&Notification::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clicked\0".as_ptr() as *const _,
                Some(transmute(clicked_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitNotification,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Notification>,
        {
            let f: &F = &*(f as *const F);
            f(&Notification::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(transmute(closed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_property_body_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_body_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitNotification,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Notification>,
        {
            let f: &F = &*(f as *const F);
            f(&Notification::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::body\0".as_ptr() as *const _,
                Some(transmute(notify_body_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitNotification,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Notification>,
        {
            let f: &F = &*(f as *const F);
            f(&Notification::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute(notify_id_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_tag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tag_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitNotification,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Notification>,
        {
            let f: &F = &*(f as *const F);
            f(&Notification::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tag\0".as_ptr() as *const _,
                Some(transmute(notify_tag_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitNotification,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Notification>,
        {
            let f: &F = &*(f as *const F);
            f(&Notification::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute(notify_title_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notification")
    }
}
