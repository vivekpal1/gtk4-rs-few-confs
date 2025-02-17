// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{MemoryFormat, Texture};
use glib::{prelude::*, translate::*};
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureDownloader(Boxed<ffi::GdkTextureDownloader>);

    match fn {
        copy => |ptr| ffi::gdk_texture_downloader_copy(ptr),
        free => |ptr| ffi::gdk_texture_downloader_free(ptr),
        type_ => || ffi::gdk_texture_downloader_get_type(),
    }
}

impl TextureDownloader {
    #[doc(alias = "gdk_texture_downloader_new")]
    pub fn new(texture: &impl IsA<Texture>) -> TextureDownloader {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_texture_downloader_new(
                texture.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_texture_downloader_download_bytes")]
    pub fn download_bytes(&self) -> (glib::Bytes, usize) {
        unsafe {
            let mut out_stride = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::gdk_texture_downloader_download_bytes(
                self.to_glib_none().0,
                out_stride.as_mut_ptr(),
            ));
            (ret, out_stride.assume_init())
        }
    }

    //#[doc(alias = "gdk_texture_downloader_download_into")]
    //pub fn download_into(&self, data: &[u8], stride: usize) {
    //    unsafe { TODO: call ffi:gdk_texture_downloader_download_into() }
    //}

    #[doc(alias = "gdk_texture_downloader_get_format")]
    #[doc(alias = "get_format")]
    pub fn format(&self) -> MemoryFormat {
        unsafe {
            from_glib(ffi::gdk_texture_downloader_get_format(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_texture_downloader_get_texture")]
    #[doc(alias = "get_texture")]
    pub fn texture(&self) -> Texture {
        unsafe {
            from_glib_none(ffi::gdk_texture_downloader_get_texture(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_texture_downloader_set_format")]
    pub fn set_format(&mut self, format: MemoryFormat) {
        unsafe {
            ffi::gdk_texture_downloader_set_format(self.to_glib_none_mut().0, format.into_glib());
        }
    }

    #[doc(alias = "gdk_texture_downloader_set_texture")]
    pub fn set_texture(&mut self, texture: &impl IsA<Texture>) {
        unsafe {
            ffi::gdk_texture_downloader_set_texture(
                self.to_glib_none_mut().0,
                texture.as_ref().to_glib_none().0,
            );
        }
    }
}
