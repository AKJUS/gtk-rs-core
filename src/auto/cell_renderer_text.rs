// This file was generated by gir (81f9b8c) from gir-files (11e0e6d)
// DO NOT EDIT

use CellRenderer;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct CellRendererText(Object<ffi::GtkCellRendererText>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_text_get_type(),
    }
}

impl CellRendererText {
    pub fn new() -> CellRendererText {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_text_new()).downcast_unchecked()
        }
    }
}

pub trait CellRendererTextExt {
    fn set_fixed_height_from_font(&self, number_of_rows: i32);
}

impl<O: IsA<CellRendererText>> CellRendererTextExt for O {
    fn set_fixed_height_from_font(&self, number_of_rows: i32) {
        unsafe {
            ffi::gtk_cell_renderer_text_set_fixed_height_from_font(self.to_glib_none().0, number_of_rows);
        }
    }
}
