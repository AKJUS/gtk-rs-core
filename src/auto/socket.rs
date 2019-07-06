// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ObjectExt;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Cancellable;
use Credentials;
use Error;
use InetAddress;
use SocketAddress;
use SocketConnection;
use SocketFamily;
use SocketProtocol;
use SocketType;

glib_wrapper! {
    pub struct Socket(Object<gio_sys::GSocket, gio_sys::GSocketClass, SocketClass>);

    match fn {
        get_type => || gio_sys::g_socket_get_type(),
    }
}

impl Socket {
    pub fn new(
        family: SocketFamily,
        type_: SocketType,
        protocol: SocketProtocol,
    ) -> Result<Socket, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_new(
                family.to_glib(),
                type_.to_glib(),
                protocol.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl glib::SendUnique for Socket {
    fn is_unique(&self) -> bool {
        self.ref_count() == 1
    }
}

pub const NONE_SOCKET: Option<&Socket> = None;

pub trait SocketExt: 'static {
    fn accept<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<Socket, Error>;

    fn bind<P: IsA<SocketAddress>>(&self, address: &P, allow_reuse: bool) -> Result<(), Error>;

    fn check_connect_result(&self) -> Result<(), Error>;

    fn close(&self) -> Result<(), Error>;

    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition;

    fn condition_timed_wait<P: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        timeout_us: i64,
        cancellable: Option<&P>,
    ) -> Result<(), Error>;

    fn condition_wait<P: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&P>,
    ) -> Result<(), Error>;

    fn connect<P: IsA<SocketAddress>, Q: IsA<Cancellable>>(
        &self,
        address: &P,
        cancellable: Option<&Q>,
    ) -> Result<(), Error>;

    fn connection_factory_create_connection(&self) -> Option<SocketConnection>;

    fn get_available_bytes(&self) -> isize;

    fn get_blocking(&self) -> bool;

    fn get_broadcast(&self) -> bool;

    fn get_credentials(&self) -> Result<Credentials, Error>;

    fn get_family(&self) -> SocketFamily;

    fn get_keepalive(&self) -> bool;

    fn get_listen_backlog(&self) -> i32;

    fn get_local_address(&self) -> Result<SocketAddress, Error>;

    fn get_multicast_loopback(&self) -> bool;

    fn get_multicast_ttl(&self) -> u32;

    fn get_option(&self, level: i32, optname: i32) -> Result<i32, Error>;

    fn get_protocol(&self) -> SocketProtocol;

    fn get_remote_address(&self) -> Result<SocketAddress, Error>;

    fn get_socket_type(&self) -> SocketType;

    fn get_timeout(&self) -> u32;

    fn get_ttl(&self) -> u32;

    fn is_closed(&self) -> bool;

    fn is_connected(&self) -> bool;

    fn join_multicast_group<P: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), Error>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn join_multicast_group_ssm<P: IsA<InetAddress>, Q: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: Option<&Q>,
        iface: Option<&str>,
    ) -> Result<(), Error>;

    fn leave_multicast_group<P: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), Error>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn leave_multicast_group_ssm<P: IsA<InetAddress>, Q: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: Option<&Q>,
        iface: Option<&str>,
    ) -> Result<(), Error>;

    fn listen(&self) -> Result<(), Error>;

    fn set_blocking(&self, blocking: bool);

    fn set_broadcast(&self, broadcast: bool);

    fn set_keepalive(&self, keepalive: bool);

    fn set_listen_backlog(&self, backlog: i32);

    fn set_multicast_loopback(&self, loopback: bool);

    fn set_multicast_ttl(&self, ttl: u32);

    fn set_option(&self, level: i32, optname: i32, value: i32) -> Result<(), Error>;

    fn set_timeout(&self, timeout: u32);

    fn set_ttl(&self, ttl: u32);

    fn shutdown(&self, shutdown_read: bool, shutdown_write: bool) -> Result<(), Error>;

    fn speaks_ipv4(&self) -> bool;

    fn get_property_type(&self) -> SocketType;

    fn connect_property_blocking_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_broadcast_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_keepalive_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_local_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_multicast_loopback_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_multicast_ttl_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_remote_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_ttl_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Socket>> SocketExt for O {
    fn accept<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<Socket, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_accept(
                self.as_ref().to_glib_none().0,
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

    fn bind<P: IsA<SocketAddress>>(&self, address: &P, allow_reuse: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_bind(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                allow_reuse.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn check_connect_result(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ =
                gio_sys::g_socket_check_connect_result(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_close(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition {
        unsafe {
            from_glib(gio_sys::g_socket_condition_check(
                self.as_ref().to_glib_none().0,
                condition.to_glib(),
            ))
        }
    }

    fn condition_timed_wait<P: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        timeout_us: i64,
        cancellable: Option<&P>,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_condition_timed_wait(
                self.as_ref().to_glib_none().0,
                condition.to_glib(),
                timeout_us,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn condition_wait<P: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&P>,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_condition_wait(
                self.as_ref().to_glib_none().0,
                condition.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect<P: IsA<SocketAddress>, Q: IsA<Cancellable>>(
        &self,
        address: &P,
        cancellable: Option<&Q>,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_connect(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connection_factory_create_connection(&self) -> Option<SocketConnection> {
        unsafe {
            from_glib_full(gio_sys::g_socket_connection_factory_create_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_available_bytes(&self) -> isize {
        unsafe { gio_sys::g_socket_get_available_bytes(self.as_ref().to_glib_none().0) }
    }

    fn get_blocking(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_socket_get_blocking(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_broadcast(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_socket_get_broadcast(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_credentials(&self) -> Result<Credentials, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_get_credentials(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe { from_glib(gio_sys::g_socket_get_family(self.as_ref().to_glib_none().0)) }
    }

    fn get_keepalive(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_socket_get_keepalive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_listen_backlog(&self) -> i32 {
        unsafe { gio_sys::g_socket_get_listen_backlog(self.as_ref().to_glib_none().0) }
    }

    fn get_local_address(&self) -> Result<SocketAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                gio_sys::g_socket_get_local_address(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_multicast_loopback(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_socket_get_multicast_loopback(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_multicast_ttl(&self) -> u32 {
        unsafe { gio_sys::g_socket_get_multicast_ttl(self.as_ref().to_glib_none().0) }
    }

    fn get_option(&self, level: i32, optname: i32) -> Result<i32, Error> {
        unsafe {
            let mut value = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_get_option(
                self.as_ref().to_glib_none().0,
                level,
                optname,
                &mut value,
                &mut error,
            );
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_protocol(&self) -> SocketProtocol {
        unsafe {
            from_glib(gio_sys::g_socket_get_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_remote_address(&self) -> Result<SocketAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                gio_sys::g_socket_get_remote_address(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_socket_type(&self) -> SocketType {
        unsafe {
            from_glib(gio_sys::g_socket_get_socket_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_timeout(&self) -> u32 {
        unsafe { gio_sys::g_socket_get_timeout(self.as_ref().to_glib_none().0) }
    }

    fn get_ttl(&self) -> u32 {
        unsafe { gio_sys::g_socket_get_ttl(self.as_ref().to_glib_none().0) }
    }

    fn is_closed(&self) -> bool {
        unsafe { from_glib(gio_sys::g_socket_is_closed(self.as_ref().to_glib_none().0)) }
    }

    fn is_connected(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_socket_is_connected(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn join_multicast_group<P: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_join_multicast_group(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.to_glib(),
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn join_multicast_group_ssm<P: IsA<InetAddress>, Q: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: Option<&Q>,
        iface: Option<&str>,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_join_multicast_group_ssm(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.map(|p| p.as_ref()).to_glib_none().0,
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn leave_multicast_group<P: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: bool,
        iface: Option<&str>,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_leave_multicast_group(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.to_glib(),
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn leave_multicast_group_ssm<P: IsA<InetAddress>, Q: IsA<InetAddress>>(
        &self,
        group: &P,
        source_specific: Option<&Q>,
        iface: Option<&str>,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_leave_multicast_group_ssm(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                source_specific.map(|p| p.as_ref()).to_glib_none().0,
                iface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn listen(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_listen(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_blocking(&self, blocking: bool) {
        unsafe {
            gio_sys::g_socket_set_blocking(self.as_ref().to_glib_none().0, blocking.to_glib());
        }
    }

    fn set_broadcast(&self, broadcast: bool) {
        unsafe {
            gio_sys::g_socket_set_broadcast(self.as_ref().to_glib_none().0, broadcast.to_glib());
        }
    }

    fn set_keepalive(&self, keepalive: bool) {
        unsafe {
            gio_sys::g_socket_set_keepalive(self.as_ref().to_glib_none().0, keepalive.to_glib());
        }
    }

    fn set_listen_backlog(&self, backlog: i32) {
        unsafe {
            gio_sys::g_socket_set_listen_backlog(self.as_ref().to_glib_none().0, backlog);
        }
    }

    fn set_multicast_loopback(&self, loopback: bool) {
        unsafe {
            gio_sys::g_socket_set_multicast_loopback(
                self.as_ref().to_glib_none().0,
                loopback.to_glib(),
            );
        }
    }

    fn set_multicast_ttl(&self, ttl: u32) {
        unsafe {
            gio_sys::g_socket_set_multicast_ttl(self.as_ref().to_glib_none().0, ttl);
        }
    }

    fn set_option(&self, level: i32, optname: i32, value: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_set_option(
                self.as_ref().to_glib_none().0,
                level,
                optname,
                value,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            gio_sys::g_socket_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    fn set_ttl(&self, ttl: u32) {
        unsafe {
            gio_sys::g_socket_set_ttl(self.as_ref().to_glib_none().0, ttl);
        }
    }

    fn shutdown(&self, shutdown_read: bool, shutdown_write: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_socket_shutdown(
                self.as_ref().to_glib_none().0,
                shutdown_read.to_glib(),
                shutdown_write.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn speaks_ipv4(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_socket_speaks_ipv4(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_type(&self) -> SocketType {
        unsafe {
            let mut value = Value::from_type(<SocketType as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn connect_property_blocking_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_blocking_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::blocking\0".as_ptr() as *const _,
                Some(transmute(notify_blocking_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_broadcast_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_broadcast_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::broadcast\0".as_ptr() as *const _,
                Some(transmute(notify_broadcast_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_keepalive_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_keepalive_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::keepalive\0".as_ptr() as *const _,
                Some(transmute(notify_keepalive_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_listen_backlog_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::listen-backlog\0".as_ptr() as *const _,
                Some(transmute(
                    notify_listen_backlog_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_local_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_address_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-address\0".as_ptr() as *const _,
                Some(transmute(
                    notify_local_address_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_multicast_loopback_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_multicast_loopback_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::multicast-loopback\0".as_ptr() as *const _,
                Some(transmute(
                    notify_multicast_loopback_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_multicast_ttl_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_multicast_ttl_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::multicast-ttl\0".as_ptr() as *const _,
                Some(transmute(
                    notify_multicast_ttl_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_remote_address_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_remote_address_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::remote-address\0".as_ptr() as *const _,
                Some(transmute(
                    notify_remote_address_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute(notify_timeout_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ttl_notify<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ttl_trampoline<P, F: Fn(&P) + Send + 'static>(
            this: *mut gio_sys::GSocket,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Socket>,
        {
            let f: &F = &*(f as *const F);
            f(&Socket::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ttl\0".as_ptr() as *const _,
                Some(transmute(notify_ttl_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket")
    }
}
