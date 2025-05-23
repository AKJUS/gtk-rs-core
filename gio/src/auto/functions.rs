// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, AsyncResult, BusType, Cancellable, DBusConnection, File, IOErrorEnum, IOStream, Icon,
    InputStream, Resource, ResourceLookupFlags, SettingsBackend,
};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, pin::Pin};

#[doc(alias = "g_bus_get")]
pub fn bus_get<P: FnOnce(Result<DBusConnection, glib::Error>) + 'static>(
    bus_type: BusType,
    cancellable: Option<&impl IsA<Cancellable>>,
    callback: P,
) {
    let main_context = glib::MainContext::ref_thread_default();
    let is_main_context_owner = main_context.is_owner();
    let has_acquired_main_context = (!is_main_context_owner)
        .then(|| main_context.acquire().ok())
        .flatten();
    assert!(
        is_main_context_owner || has_acquired_main_context.is_some(),
        "Async operations only allowed if the thread is owning the MainContext"
    );

    let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
        Box_::new(glib::thread_guard::ThreadGuard::new(callback));
    unsafe extern "C" fn bus_get_trampoline<
        P: FnOnce(Result<DBusConnection, glib::Error>) + 'static,
    >(
        _source_object: *mut glib::gobject_ffi::GObject,
        res: *mut crate::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = std::ptr::null_mut();
        let ret = ffi::g_bus_get_finish(res, &mut error);
        let result = if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        };
        let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::from_raw(user_data as *mut _);
        let callback: P = callback.into_inner();
        callback(result);
    }
    let callback = bus_get_trampoline::<P>;
    unsafe {
        ffi::g_bus_get(
            bus_type.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box_::into_raw(user_data) as *mut _,
        );
    }
}

pub fn bus_get_future(
    bus_type: BusType,
) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusConnection, glib::Error>> + 'static>> {
    Box_::pin(crate::GioFuture::new(
        &(),
        move |_obj, cancellable, send| {
            bus_get(bus_type, Some(cancellable), move |res| {
                send.resolve(res);
            });
        },
    ))
}

#[doc(alias = "g_bus_get_sync")]
pub fn bus_get_sync(
    bus_type: BusType,
    cancellable: Option<&impl IsA<Cancellable>>,
) -> Result<DBusConnection, glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let ret = ffi::g_bus_get_sync(
            bus_type.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

//#[doc(alias = "g_bus_own_name")]
//pub fn bus_own_name(bus_type: BusType, name: &str, flags: BusNameOwnerFlags, bus_acquired_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>, name_acquired_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>, name_lost_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>) -> u32 {
//    unsafe { TODO: call ffi:g_bus_own_name() }
//}

//#[doc(alias = "g_bus_own_name_on_connection")]
//pub fn bus_own_name_on_connection(connection: &DBusConnection, name: &str, flags: BusNameOwnerFlags, name_acquired_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>, name_lost_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>) -> u32 {
//    unsafe { TODO: call ffi:g_bus_own_name_on_connection() }
//}

//#[doc(alias = "g_bus_watch_name")]
//pub fn bus_watch_name(bus_type: BusType, name: &str, flags: BusNameWatcherFlags, name_appeared_handler: Option<Box_<dyn Fn(&DBusConnection, &str, &str) + 'static>>, name_vanished_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>) -> u32 {
//    unsafe { TODO: call ffi:g_bus_watch_name() }
//}

//#[doc(alias = "g_bus_watch_name_on_connection")]
//pub fn bus_watch_name_on_connection(connection: &DBusConnection, name: &str, flags: BusNameWatcherFlags, name_appeared_handler: Option<Box_<dyn Fn(&DBusConnection, &str, &str) + 'static>>, name_vanished_handler: Option<Box_<dyn Fn(&DBusConnection, &str) + 'static>>) -> u32 {
//    unsafe { TODO: call ffi:g_bus_watch_name_on_connection() }
//}

#[doc(alias = "g_content_type_can_be_executable")]
pub fn content_type_can_be_executable(type_: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_can_be_executable(
            type_.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_equals")]
pub fn content_type_equals(type1: &str, type2: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_equals(
            type1.to_glib_none().0,
            type2.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_from_mime_type")]
pub fn content_type_from_mime_type(mime_type: &str) -> Option<glib::GString> {
    unsafe {
        from_glib_full(ffi::g_content_type_from_mime_type(
            mime_type.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_get_description")]
pub fn content_type_get_description(type_: &str) -> glib::GString {
    unsafe { from_glib_full(ffi::g_content_type_get_description(type_.to_glib_none().0)) }
}

#[doc(alias = "g_content_type_get_generic_icon_name")]
pub fn content_type_get_generic_icon_name(type_: &str) -> Option<glib::GString> {
    unsafe {
        from_glib_full(ffi::g_content_type_get_generic_icon_name(
            type_.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_get_icon")]
pub fn content_type_get_icon(type_: &str) -> Icon {
    unsafe { from_glib_full(ffi::g_content_type_get_icon(type_.to_glib_none().0)) }
}

#[cfg(feature = "v2_60")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
#[doc(alias = "g_content_type_get_mime_dirs")]
pub fn content_type_get_mime_dirs() -> Vec<glib::GString> {
    unsafe { FromGlibPtrContainer::from_glib_none(ffi::g_content_type_get_mime_dirs()) }
}

#[doc(alias = "g_content_type_get_mime_type")]
pub fn content_type_get_mime_type(type_: &str) -> Option<glib::GString> {
    unsafe { from_glib_full(ffi::g_content_type_get_mime_type(type_.to_glib_none().0)) }
}

#[doc(alias = "g_content_type_get_symbolic_icon")]
pub fn content_type_get_symbolic_icon(type_: &str) -> Icon {
    unsafe {
        from_glib_full(ffi::g_content_type_get_symbolic_icon(
            type_.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_guess_for_tree")]
pub fn content_type_guess_for_tree(root: &impl IsA<File>) -> Vec<glib::GString> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::g_content_type_guess_for_tree(
            root.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_is_a")]
pub fn content_type_is_a(type_: &str, supertype: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_is_a(
            type_.to_glib_none().0,
            supertype.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_is_mime_type")]
pub fn content_type_is_mime_type(type_: &str, mime_type: &str) -> bool {
    unsafe {
        from_glib(ffi::g_content_type_is_mime_type(
            type_.to_glib_none().0,
            mime_type.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_content_type_is_unknown")]
pub fn content_type_is_unknown(type_: &str) -> bool {
    unsafe { from_glib(ffi::g_content_type_is_unknown(type_.to_glib_none().0)) }
}

#[cfg(feature = "v2_60")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
#[doc(alias = "g_content_type_set_mime_dirs")]
pub fn content_type_set_mime_dirs(dirs: &[&str]) {
    unsafe {
        ffi::g_content_type_set_mime_dirs(dirs.to_glib_none().0);
    }
}

#[doc(alias = "g_content_types_get_registered")]
pub fn content_types_get_registered() -> Vec<glib::GString> {
    unsafe { FromGlibPtrContainer::from_glib_full(ffi::g_content_types_get_registered()) }
}

#[doc(alias = "g_dbus_address_escape_value")]
pub fn dbus_address_escape_value(string: &str) -> glib::GString {
    unsafe { from_glib_full(ffi::g_dbus_address_escape_value(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_address_get_for_bus_sync")]
pub fn dbus_address_get_for_bus_sync(
    bus_type: BusType,
    cancellable: Option<&impl IsA<Cancellable>>,
) -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let ret = ffi::g_dbus_address_get_for_bus_sync(
            bus_type.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_dbus_address_get_stream")]
pub fn dbus_address_get_stream<
    P: FnOnce(Result<(IOStream, Option<glib::GString>), glib::Error>) + 'static,
>(
    address: &str,
    cancellable: Option<&impl IsA<Cancellable>>,
    callback: P,
) {
    let main_context = glib::MainContext::ref_thread_default();
    let is_main_context_owner = main_context.is_owner();
    let has_acquired_main_context = (!is_main_context_owner)
        .then(|| main_context.acquire().ok())
        .flatten();
    assert!(
        is_main_context_owner || has_acquired_main_context.is_some(),
        "Async operations only allowed if the thread is owning the MainContext"
    );

    let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
        Box_::new(glib::thread_guard::ThreadGuard::new(callback));
    unsafe extern "C" fn dbus_address_get_stream_trampoline<
        P: FnOnce(Result<(IOStream, Option<glib::GString>), glib::Error>) + 'static,
    >(
        _source_object: *mut glib::gobject_ffi::GObject,
        res: *mut crate::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = std::ptr::null_mut();
        let mut out_guid = std::ptr::null_mut();
        let ret = ffi::g_dbus_address_get_stream_finish(res, &mut out_guid, &mut error);
        let result = if error.is_null() {
            Ok((from_glib_full(ret), from_glib_full(out_guid)))
        } else {
            Err(from_glib_full(error))
        };
        let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::from_raw(user_data as *mut _);
        let callback: P = callback.into_inner();
        callback(result);
    }
    let callback = dbus_address_get_stream_trampoline::<P>;
    unsafe {
        ffi::g_dbus_address_get_stream(
            address.to_glib_none().0,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box_::into_raw(user_data) as *mut _,
        );
    }
}

pub fn dbus_address_get_stream_future(
    address: &str,
) -> Pin<
    Box_<
        dyn std::future::Future<Output = Result<(IOStream, Option<glib::GString>), glib::Error>>
            + 'static,
    >,
> {
    let address = String::from(address);
    Box_::pin(crate::GioFuture::new(
        &(),
        move |_obj, cancellable, send| {
            dbus_address_get_stream(&address, Some(cancellable), move |res| {
                send.resolve(res);
            });
        },
    ))
}

#[doc(alias = "g_dbus_address_get_stream_sync")]
pub fn dbus_address_get_stream_sync(
    address: &str,
    cancellable: Option<&impl IsA<Cancellable>>,
) -> Result<(IOStream, Option<glib::GString>), glib::Error> {
    unsafe {
        let mut out_guid = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let ret = ffi::g_dbus_address_get_stream_sync(
            address.to_glib_none().0,
            &mut out_guid,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok((from_glib_full(ret), from_glib_full(out_guid)))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(feature = "v2_68")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
#[doc(alias = "g_dbus_escape_object_path")]
pub fn dbus_escape_object_path(s: &str) -> glib::GString {
    unsafe { from_glib_full(ffi::g_dbus_escape_object_path(s.to_glib_none().0)) }
}

//#[cfg(feature = "v2_68")]
//#[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
//#[doc(alias = "g_dbus_escape_object_path_bytestring")]
//pub fn dbus_escape_object_path_bytestring(bytes: &[u8]) -> glib::GString {
//    unsafe { TODO: call ffi:g_dbus_escape_object_path_bytestring() }
//}

#[doc(alias = "g_dbus_generate_guid")]
pub fn dbus_generate_guid() -> glib::GString {
    unsafe { from_glib_full(ffi::g_dbus_generate_guid()) }
}

#[doc(alias = "g_dbus_gvalue_to_gvariant")]
pub fn dbus_gvalue_to_gvariant(gvalue: &glib::Value, type_: &glib::VariantTy) -> glib::Variant {
    unsafe {
        from_glib_full(ffi::g_dbus_gvalue_to_gvariant(
            gvalue.to_glib_none().0,
            type_.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_dbus_gvariant_to_gvalue")]
pub fn dbus_gvariant_to_gvalue(value: &glib::Variant) -> glib::Value {
    unsafe {
        let mut out_gvalue = glib::Value::uninitialized();
        ffi::g_dbus_gvariant_to_gvalue(value.to_glib_none().0, out_gvalue.to_glib_none_mut().0);
        out_gvalue
    }
}

#[doc(alias = "g_dbus_is_address")]
pub fn dbus_is_address(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_address(string.to_glib_none().0)) }
}

#[cfg(feature = "v2_70")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
#[doc(alias = "g_dbus_is_error_name")]
pub fn dbus_is_error_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_error_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_guid")]
pub fn dbus_is_guid(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_guid(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_interface_name")]
pub fn dbus_is_interface_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_interface_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_member_name")]
pub fn dbus_is_member_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_member_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_name")]
pub fn dbus_is_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_dbus_is_supported_address")]
pub fn dbus_is_supported_address(string: &str) -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::g_dbus_is_supported_address(string.to_glib_none().0, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_dbus_is_unique_name")]
pub fn dbus_is_unique_name(string: &str) -> bool {
    unsafe { from_glib(ffi::g_dbus_is_unique_name(string.to_glib_none().0)) }
}

#[doc(alias = "g_io_error_from_errno")]
pub fn io_error_from_errno(err_no: i32) -> IOErrorEnum {
    unsafe { from_glib(ffi::g_io_error_from_errno(err_no)) }
}

//#[doc(alias = "g_io_modules_load_all_in_directory")]
//pub fn io_modules_load_all_in_directory(dirname: impl AsRef<std::path::Path>) -> /*Ignored*/Vec<IOModule> {
//    unsafe { TODO: call ffi:g_io_modules_load_all_in_directory() }
//}

//#[doc(alias = "g_io_modules_load_all_in_directory_with_scope")]
//pub fn io_modules_load_all_in_directory_with_scope(dirname: impl AsRef<std::path::Path>, scope: /*Ignored*/&mut IOModuleScope) -> /*Ignored*/Vec<IOModule> {
//    unsafe { TODO: call ffi:g_io_modules_load_all_in_directory_with_scope() }
//}

#[doc(alias = "g_io_modules_scan_all_in_directory")]
pub fn io_modules_scan_all_in_directory(dirname: impl AsRef<std::path::Path>) {
    unsafe {
        ffi::g_io_modules_scan_all_in_directory(dirname.as_ref().to_glib_none().0);
    }
}

//#[doc(alias = "g_io_modules_scan_all_in_directory_with_scope")]
//pub fn io_modules_scan_all_in_directory_with_scope(dirname: impl AsRef<std::path::Path>, scope: /*Ignored*/&mut IOModuleScope) {
//    unsafe { TODO: call ffi:g_io_modules_scan_all_in_directory_with_scope() }
//}

#[doc(alias = "g_keyfile_settings_backend_new")]
pub fn keyfile_settings_backend_new(
    filename: &str,
    root_path: &str,
    root_group: Option<&str>,
) -> SettingsBackend {
    unsafe {
        from_glib_full(ffi::g_keyfile_settings_backend_new(
            filename.to_glib_none().0,
            root_path.to_glib_none().0,
            root_group.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_memory_settings_backend_new")]
pub fn memory_settings_backend_new() -> SettingsBackend {
    unsafe { from_glib_full(ffi::g_memory_settings_backend_new()) }
}

#[doc(alias = "g_null_settings_backend_new")]
pub fn null_settings_backend_new() -> SettingsBackend {
    unsafe { from_glib_full(ffi::g_null_settings_backend_new()) }
}

#[doc(alias = "g_resources_enumerate_children")]
pub fn resources_enumerate_children(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<Vec<glib::GString>, glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let ret = ffi::g_resources_enumerate_children(
            path.to_glib_none().0,
            lookup_flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(FromGlibPtrContainer::from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_resources_get_info")]
pub fn resources_get_info(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<(usize, u32), glib::Error> {
    unsafe {
        let mut size = std::mem::MaybeUninit::uninit();
        let mut flags = std::mem::MaybeUninit::uninit();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::g_resources_get_info(
            path.to_glib_none().0,
            lookup_flags.into_glib(),
            size.as_mut_ptr(),
            flags.as_mut_ptr(),
            &mut error,
        );
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok((size.assume_init(), flags.assume_init()))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(feature = "v2_84")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_84")))]
#[doc(alias = "g_resources_has_children")]
pub fn resources_has_children(path: &str) -> bool {
    unsafe { from_glib(ffi::g_resources_has_children(path.to_glib_none().0)) }
}

#[doc(alias = "g_resources_lookup_data")]
pub fn resources_lookup_data(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<glib::Bytes, glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let ret = ffi::g_resources_lookup_data(
            path.to_glib_none().0,
            lookup_flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_resources_open_stream")]
pub fn resources_open_stream(
    path: &str,
    lookup_flags: ResourceLookupFlags,
) -> Result<InputStream, glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let ret = ffi::g_resources_open_stream(
            path.to_glib_none().0,
            lookup_flags.into_glib(),
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_resources_register")]
pub fn resources_register(resource: &Resource) {
    unsafe {
        ffi::g_resources_register(resource.to_glib_none().0);
    }
}

#[doc(alias = "g_resources_unregister")]
pub fn resources_unregister(resource: &Resource) {
    unsafe {
        ffi::g_resources_unregister(resource.to_glib_none().0);
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[doc(alias = "g_unix_is_mount_path_system_internal")]
pub fn unix_is_mount_path_system_internal(mount_path: impl AsRef<std::path::Path>) -> bool {
    unsafe {
        from_glib(ffi::g_unix_is_mount_path_system_internal(
            mount_path.as_ref().to_glib_none().0,
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[doc(alias = "g_unix_is_system_device_path")]
pub fn unix_is_system_device_path(device_path: impl AsRef<std::path::Path>) -> bool {
    unsafe {
        from_glib(ffi::g_unix_is_system_device_path(
            device_path.as_ref().to_glib_none().0,
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[doc(alias = "g_unix_is_system_fs_type")]
pub fn unix_is_system_fs_type(fs_type: &str) -> bool {
    unsafe { from_glib(ffi::g_unix_is_system_fs_type(fs_type.to_glib_none().0)) }
}
