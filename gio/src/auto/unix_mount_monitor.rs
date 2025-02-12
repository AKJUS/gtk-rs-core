// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GUnixMountMonitor")]
    pub struct UnixMountMonitor(Object<ffi::GUnixMountMonitor, ffi::GUnixMountMonitorClass>);

    match fn {
        type_ => || ffi::g_unix_mount_monitor_get_type(),
    }
}

impl UnixMountMonitor {
    #[doc(alias = "g_unix_mount_monitor_get")]
    pub fn get() -> UnixMountMonitor {
        unsafe { from_glib_full(ffi::g_unix_mount_monitor_get()) }
    }

    #[doc(alias = "mountpoints-changed")]
    pub fn connect_mountpoints_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn mountpoints_changed_trampoline<F: Fn(&UnixMountMonitor) + 'static>(
            this: *mut ffi::GUnixMountMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"mountpoints-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    mountpoints_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mounts-changed")]
    pub fn connect_mounts_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn mounts_changed_trampoline<F: Fn(&UnixMountMonitor) + 'static>(
            this: *mut ffi::GUnixMountMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"mounts-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    mounts_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
