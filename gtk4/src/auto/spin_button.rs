// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::CellEditable;
use crate::ConstraintTarget;
use crate::Editable;
use crate::LayoutManager;
use crate::Orientable;
use crate::Orientation;
use crate::Overflow;
use crate::ScrollType;
use crate::SpinButtonUpdatePolicy;
use crate::SpinType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkSpinButton")]
    pub struct SpinButton(Object<ffi::GtkSpinButton>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, CellEditable, Editable, Orientable;

    match fn {
        type_ => || ffi::gtk_spin_button_get_type(),
    }
}

impl SpinButton {
    #[doc(alias = "gtk_spin_button_new")]
    pub fn new(
        adjustment: Option<&impl IsA<Adjustment>>,
        climb_rate: f64,
        digits: u32,
    ) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spin_button_new(
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
                climb_rate,
                digits,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_spin_button_new_with_range")]
    #[doc(alias = "new_with_range")]
    pub fn with_range(min: f64, max: f64, step: f64) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spin_button_new_with_range(min, max, step))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SpinButton`] objects.
    ///
    /// This method returns an instance of [`SpinButtonBuilder`](crate::builders::SpinButtonBuilder) which can be used to create [`SpinButton`] objects.
    pub fn builder() -> SpinButtonBuilder {
        SpinButtonBuilder::default()
    }

    #[doc(alias = "gtk_spin_button_configure")]
    pub fn configure(
        &self,
        adjustment: Option<&impl IsA<Adjustment>>,
        climb_rate: f64,
        digits: u32,
    ) {
        unsafe {
            ffi::gtk_spin_button_configure(
                self.to_glib_none().0,
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
                climb_rate,
                digits,
            );
        }
    }

    #[doc(alias = "gtk_spin_button_get_adjustment")]
    #[doc(alias = "get_adjustment")]
    pub fn adjustment(&self) -> Adjustment {
        unsafe { from_glib_none(ffi::gtk_spin_button_get_adjustment(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_spin_button_get_climb_rate")]
    #[doc(alias = "get_climb_rate")]
    pub fn climb_rate(&self) -> f64 {
        unsafe { ffi::gtk_spin_button_get_climb_rate(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_spin_button_get_digits")]
    #[doc(alias = "get_digits")]
    pub fn digits(&self) -> u32 {
        unsafe { ffi::gtk_spin_button_get_digits(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_spin_button_get_increments")]
    #[doc(alias = "get_increments")]
    pub fn increments(&self) -> (f64, f64) {
        unsafe {
            let mut step = mem::MaybeUninit::uninit();
            let mut page = mem::MaybeUninit::uninit();
            ffi::gtk_spin_button_get_increments(
                self.to_glib_none().0,
                step.as_mut_ptr(),
                page.as_mut_ptr(),
            );
            let step = step.assume_init();
            let page = page.assume_init();
            (step, page)
        }
    }

    #[doc(alias = "gtk_spin_button_get_numeric")]
    #[doc(alias = "get_numeric")]
    pub fn is_numeric(&self) -> bool {
        unsafe { from_glib(ffi::gtk_spin_button_get_numeric(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_spin_button_get_range")]
    #[doc(alias = "get_range")]
    pub fn range(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            ffi::gtk_spin_button_get_range(
                self.to_glib_none().0,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            let min = min.assume_init();
            let max = max.assume_init();
            (min, max)
        }
    }

    #[doc(alias = "gtk_spin_button_get_snap_to_ticks")]
    #[doc(alias = "get_snap_to_ticks")]
    pub fn snaps_to_ticks(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_snap_to_ticks(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_spin_button_get_update_policy")]
    #[doc(alias = "get_update_policy")]
    pub fn update_policy(&self) -> SpinButtonUpdatePolicy {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_update_policy(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_spin_button_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self) -> f64 {
        unsafe { ffi::gtk_spin_button_get_value(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_spin_button_get_value_as_int")]
    #[doc(alias = "get_value_as_int")]
    pub fn value_as_int(&self) -> i32 {
        unsafe { ffi::gtk_spin_button_get_value_as_int(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_spin_button_get_wrap")]
    #[doc(alias = "get_wrap")]
    pub fn wraps(&self) -> bool {
        unsafe { from_glib(ffi::gtk_spin_button_get_wrap(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_spin_button_set_adjustment")]
    pub fn set_adjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_spin_button_set_adjustment(
                self.to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_spin_button_set_climb_rate")]
    pub fn set_climb_rate(&self, climb_rate: f64) {
        unsafe {
            ffi::gtk_spin_button_set_climb_rate(self.to_glib_none().0, climb_rate);
        }
    }

    #[doc(alias = "gtk_spin_button_set_digits")]
    pub fn set_digits(&self, digits: u32) {
        unsafe {
            ffi::gtk_spin_button_set_digits(self.to_glib_none().0, digits);
        }
    }

    #[doc(alias = "gtk_spin_button_set_increments")]
    pub fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            ffi::gtk_spin_button_set_increments(self.to_glib_none().0, step, page);
        }
    }

    #[doc(alias = "gtk_spin_button_set_numeric")]
    pub fn set_numeric(&self, numeric: bool) {
        unsafe {
            ffi::gtk_spin_button_set_numeric(self.to_glib_none().0, numeric.into_glib());
        }
    }

    #[doc(alias = "gtk_spin_button_set_range")]
    pub fn set_range(&self, min: f64, max: f64) {
        unsafe {
            ffi::gtk_spin_button_set_range(self.to_glib_none().0, min, max);
        }
    }

    #[doc(alias = "gtk_spin_button_set_snap_to_ticks")]
    pub fn set_snap_to_ticks(&self, snap_to_ticks: bool) {
        unsafe {
            ffi::gtk_spin_button_set_snap_to_ticks(
                self.to_glib_none().0,
                snap_to_ticks.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_spin_button_set_update_policy")]
    pub fn set_update_policy(&self, policy: SpinButtonUpdatePolicy) {
        unsafe {
            ffi::gtk_spin_button_set_update_policy(self.to_glib_none().0, policy.into_glib());
        }
    }

    #[doc(alias = "gtk_spin_button_set_value")]
    pub fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_spin_button_set_value(self.to_glib_none().0, value);
        }
    }

    #[doc(alias = "gtk_spin_button_set_wrap")]
    pub fn set_wrap(&self, wrap: bool) {
        unsafe {
            ffi::gtk_spin_button_set_wrap(self.to_glib_none().0, wrap.into_glib());
        }
    }

    #[doc(alias = "gtk_spin_button_spin")]
    pub fn spin(&self, direction: SpinType, increment: f64) {
        unsafe {
            ffi::gtk_spin_button_spin(self.to_glib_none().0, direction.into_glib(), increment);
        }
    }

    #[doc(alias = "gtk_spin_button_update")]
    pub fn update(&self) {
        unsafe {
            ffi::gtk_spin_button_update(self.to_glib_none().0);
        }
    }

    #[doc(alias = "change-value")]
    pub fn connect_change_value<F: Fn(&Self, ScrollType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn change_value_trampoline<F: Fn(&SpinButton, ScrollType) + 'static>(
            this: *mut ffi::GtkSpinButton,
            scroll: ffi::GtkScrollType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(scroll))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    change_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_change_value(&self, scroll: ScrollType) {
        self.emit_by_name::<()>("change-value", &[&scroll]);
    }

    //#[doc(alias = "input")]
    //pub fn connect_input<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Out new_value: *.Double
    //}

    #[doc(alias = "output")]
    pub fn connect_output<F: Fn(&Self) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn output_trampoline<
            F: Fn(&SpinButton) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkSpinButton,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"output\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    output_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value-changed")]
    pub fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    value_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wrapped")]
    pub fn connect_wrapped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn wrapped_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"wrapped\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    wrapped_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "adjustment")]
    pub fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_adjustment_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "climb-rate")]
    pub fn connect_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_climb_rate_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::climb-rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_climb_rate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "digits")]
    pub fn connect_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_digits_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::digits\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_digits_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "numeric")]
    pub fn connect_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_numeric_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::numeric\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_numeric_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "snap-to-ticks")]
    pub fn connect_snap_to_ticks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_snap_to_ticks_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::snap-to-ticks\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_snap_to_ticks_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "update-policy")]
    pub fn connect_update_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_update_policy_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::update-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_update_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value")]
    pub fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wrap")]
    pub fn connect_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_trampoline<F: Fn(&SpinButton) + 'static>(
            this: *mut ffi::GtkSpinButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wrap\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wrap_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SpinButton {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct SpinButton object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SpinButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SpinButtonBuilder {
    adjustment: Option<Adjustment>,
    climb_rate: Option<f64>,
    digits: Option<u32>,
    numeric: Option<bool>,
    snap_to_ticks: Option<bool>,
    update_policy: Option<SpinButtonUpdatePolicy>,
    value: Option<f64>,
    wrap: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
    editing_canceled: Option<bool>,
    editable: Option<bool>,
    enable_undo: Option<bool>,
    max_width_chars: Option<i32>,
    text: Option<String>,
    width_chars: Option<i32>,
    xalign: Option<f32>,
    orientation: Option<Orientation>,
}

impl SpinButtonBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SpinButtonBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SpinButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SpinButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref adjustment) = self.adjustment {
            properties.push(("adjustment", adjustment));
        }
        if let Some(ref climb_rate) = self.climb_rate {
            properties.push(("climb-rate", climb_rate));
        }
        if let Some(ref digits) = self.digits {
            properties.push(("digits", digits));
        }
        if let Some(ref numeric) = self.numeric {
            properties.push(("numeric", numeric));
        }
        if let Some(ref snap_to_ticks) = self.snap_to_ticks {
            properties.push(("snap-to-ticks", snap_to_ticks));
        }
        if let Some(ref update_policy) = self.update_policy {
            properties.push(("update-policy", update_policy));
        }
        if let Some(ref value) = self.value {
            properties.push(("value", value));
        }
        if let Some(ref wrap) = self.wrap {
            properties.push(("wrap", wrap));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref editing_canceled) = self.editing_canceled {
            properties.push(("editing-canceled", editing_canceled));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref enable_undo) = self.enable_undo {
            properties.push(("enable-undo", enable_undo));
        }
        if let Some(ref max_width_chars) = self.max_width_chars {
            properties.push(("max-width-chars", max_width_chars));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        if let Some(ref width_chars) = self.width_chars {
            properties.push(("width-chars", width_chars));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<SpinButton>(&properties)
            .expect("Failed to create an instance of SpinButton")
    }

    pub fn adjustment(mut self, adjustment: &impl IsA<Adjustment>) -> Self {
        self.adjustment = Some(adjustment.clone().upcast());
        self
    }

    pub fn climb_rate(mut self, climb_rate: f64) -> Self {
        self.climb_rate = Some(climb_rate);
        self
    }

    pub fn digits(mut self, digits: u32) -> Self {
        self.digits = Some(digits);
        self
    }

    pub fn numeric(mut self, numeric: bool) -> Self {
        self.numeric = Some(numeric);
        self
    }

    pub fn snap_to_ticks(mut self, snap_to_ticks: bool) -> Self {
        self.snap_to_ticks = Some(snap_to_ticks);
        self
    }

    pub fn update_policy(mut self, update_policy: SpinButtonUpdatePolicy) -> Self {
        self.update_policy = Some(update_policy);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn editing_canceled(mut self, editing_canceled: bool) -> Self {
        self.editing_canceled = Some(editing_canceled);
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn enable_undo(mut self, enable_undo: bool) -> Self {
        self.enable_undo = Some(enable_undo);
        self
    }

    pub fn max_width_chars(mut self, max_width_chars: i32) -> Self {
        self.max_width_chars = Some(max_width_chars);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn width_chars(mut self, width_chars: i32) -> Self {
        self.width_chars = Some(width_chars);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for SpinButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SpinButton")
    }
}
