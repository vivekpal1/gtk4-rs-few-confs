// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::Constraint;
use crate::ConstraintGuide;
use crate::LayoutManager;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkConstraintLayout")]
    pub struct ConstraintLayout(Object<ffi::GtkConstraintLayout, ffi::GtkConstraintLayoutClass>) @extends LayoutManager, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_constraint_layout_get_type(),
    }
}

impl ConstraintLayout {
    #[doc(alias = "gtk_constraint_layout_new")]
    pub fn new() -> ConstraintLayout {
        assert_initialized_main_thread!();
        unsafe { LayoutManager::from_glib_full(ffi::gtk_constraint_layout_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_constraint_layout_add_constraint")]
    pub fn add_constraint(&self, constraint: Constraint) {
        unsafe {
            ffi::gtk_constraint_layout_add_constraint(
                self.to_glib_none().0,
                constraint.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "gtk_constraint_layout_add_guide")]
    pub fn add_guide(&self, guide: ConstraintGuide) {
        unsafe {
            ffi::gtk_constraint_layout_add_guide(self.to_glib_none().0, guide.into_glib_ptr());
        }
    }

    #[doc(alias = "gtk_constraint_layout_observe_constraints")]
    pub fn observe_constraints(&self) -> gio::ListModel {
        unsafe {
            from_glib_full(ffi::gtk_constraint_layout_observe_constraints(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_layout_observe_guides")]
    pub fn observe_guides(&self) -> gio::ListModel {
        unsafe {
            from_glib_full(ffi::gtk_constraint_layout_observe_guides(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_layout_remove_all_constraints")]
    pub fn remove_all_constraints(&self) {
        unsafe {
            ffi::gtk_constraint_layout_remove_all_constraints(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_constraint_layout_remove_constraint")]
    pub fn remove_constraint(&self, constraint: &Constraint) {
        unsafe {
            ffi::gtk_constraint_layout_remove_constraint(
                self.to_glib_none().0,
                constraint.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_constraint_layout_remove_guide")]
    pub fn remove_guide(&self, guide: &ConstraintGuide) {
        unsafe {
            ffi::gtk_constraint_layout_remove_guide(self.to_glib_none().0, guide.to_glib_none().0);
        }
    }
}

impl Default for ConstraintLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ConstraintLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConstraintLayout")
    }
}
