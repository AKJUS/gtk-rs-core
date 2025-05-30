// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, AsyncResult, Cancellable, FilterInputStream, InputStream, Seekable};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GBufferedInputStream")]
    pub struct BufferedInputStream(Object<ffi::GBufferedInputStream, ffi::GBufferedInputStreamClass>) @extends FilterInputStream, InputStream, @implements Seekable;

    match fn {
        type_ => || ffi::g_buffered_input_stream_get_type(),
    }
}

impl BufferedInputStream {
    pub const NONE: Option<&'static BufferedInputStream> = None;

    #[doc(alias = "g_buffered_input_stream_new")]
    pub fn new(base_stream: &impl IsA<InputStream>) -> BufferedInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_buffered_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "g_buffered_input_stream_new_sized")]
    pub fn new_sized(base_stream: &impl IsA<InputStream>, size: usize) -> BufferedInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_buffered_input_stream_new_sized(
                base_stream.as_ref().to_glib_none().0,
                size,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BufferedInputStream`] objects.
    ///
    /// This method returns an instance of [`BufferedInputStreamBuilder`](crate::builders::BufferedInputStreamBuilder) which can be used to create [`BufferedInputStream`] objects.
    pub fn builder() -> BufferedInputStreamBuilder {
        BufferedInputStreamBuilder::new()
    }
}

impl Default for BufferedInputStream {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BufferedInputStream`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BufferedInputStreamBuilder {
    builder: glib::object::ObjectBuilder<'static, BufferedInputStream>,
}

impl BufferedInputStreamBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn buffer_size(self, buffer_size: u32) -> Self {
        Self {
            builder: self.builder.property("buffer-size", buffer_size),
        }
    }

    pub fn base_stream(self, base_stream: &impl IsA<InputStream>) -> Self {
        Self {
            builder: self
                .builder
                .property("base-stream", base_stream.clone().upcast()),
        }
    }

    pub fn close_base_stream(self, close_base_stream: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("close-base-stream", close_base_stream),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`BufferedInputStream`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BufferedInputStream {
        self.builder.build()
    }
}

pub trait BufferedInputStreamExt: IsA<BufferedInputStream> + 'static {
    #[doc(alias = "g_buffered_input_stream_fill")]
    fn fill(
        &self,
        count: isize,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_buffered_input_stream_fill(
                self.as_ref().to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_buffered_input_stream_fill_async")]
    fn fill_async<P: FnOnce(Result<isize, glib::Error>) + 'static>(
        &self,
        count: isize,
        io_priority: glib::Priority,
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
        unsafe extern "C" fn fill_async_trampoline<
            P: FnOnce(Result<isize, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::g_buffered_input_stream_fill_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = fill_async_trampoline::<P>;
        unsafe {
            ffi::g_buffered_input_stream_fill_async(
                self.as_ref().to_glib_none().0,
                count,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn fill_future(
        &self,
        count: isize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.fill_async(count, io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_buffered_input_stream_get_available")]
    #[doc(alias = "get_available")]
    fn available(&self) -> usize {
        unsafe { ffi::g_buffered_input_stream_get_available(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_buffered_input_stream_get_buffer_size")]
    #[doc(alias = "get_buffer_size")]
    #[doc(alias = "buffer-size")]
    fn buffer_size(&self) -> usize {
        unsafe { ffi::g_buffered_input_stream_get_buffer_size(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_buffered_input_stream_peek_buffer")]
    fn peek_buffer(&self) -> Vec<u8> {
        unsafe {
            let mut count = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::g_buffered_input_stream_peek_buffer(
                    self.as_ref().to_glib_none().0,
                    count.as_mut_ptr(),
                ),
                count.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "g_buffered_input_stream_read_byte")]
    fn read_byte(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_buffered_input_stream_read_byte(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_buffered_input_stream_set_buffer_size")]
    #[doc(alias = "buffer-size")]
    fn set_buffer_size(&self, size: usize) {
        unsafe {
            ffi::g_buffered_input_stream_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    #[doc(alias = "buffer-size")]
    fn connect_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_size_trampoline<
            P: IsA<BufferedInputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GBufferedInputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BufferedInputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::buffer-size".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_buffer_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<BufferedInputStream>> BufferedInputStreamExt for O {}
