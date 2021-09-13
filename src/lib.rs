extern crate libappindicator_sys;
extern crate glib;
extern crate gtk;
extern crate gtk_sys;

use std::ffi::CStr;

pub use libappindicator_sys::*;
use libappindicator_sys::{AppIndicator as AppIndicatorRaw};
use glib::translate::{FromGlibPtrNone, ToGlibPtr};

#[derive(Debug)]
pub enum ErrorReason {
    NullPtr,
    Utf8Error,
}

#[derive(Debug)]
pub struct Error {
    reason: ErrorReason,
}

impl Error{
    pub fn null_ptr() -> Self {
        Self{ reason: ErrorReason::NullPtr }
    }

    pub fn utf8_error() -> Self {
        Self{ reason: ErrorReason::Utf8Error }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match self.reason {
            ErrorReason::NullPtr => "null string pointer",
            ErrorReason::Utf8Error => "utf8 conversion error",
        };
        write!(f, "{}", reason)
    }
}
impl std::error::Error for Error {}

fn cstr_to_string(ptr: *const i8) -> Result<String, Error>
{
    if ptr.is_null()
    {
        return Err(Error::null_ptr());
    }

    let cstr = unsafe{ CStr::from_ptr(ptr) };
    match cstr.to_str() {
        Ok(s) => Ok(s.to_owned()),
        Err(_) => Err(Error::utf8_error())
    }
}

pub struct AppIndicator {
    air: *mut AppIndicatorRaw
}
pub enum AppIndicatorCategory{
    ApplicationStatus = 0,
    Communications = 1,
    SystemServices = 2,
    Hardware = 3,
    Other = 4
}

impl From<u32> for AppIndicatorCategory {
    fn from(num: u32) -> Self {
        match num {
            0 => AppIndicatorCategory::ApplicationStatus,
            1 => AppIndicatorCategory::Communications,
            2 => AppIndicatorCategory::SystemServices,
            3 => AppIndicatorCategory::Hardware,
            4 => AppIndicatorCategory::Other,
            _ => panic!("invalid entry"),
        }
    }
}

pub enum AppIndicatorStatus{
    Passive = 0,
    Active = 1,
    Attention = 2
}

impl From<u32> for AppIndicatorStatus {
    fn from(num: u32) -> Self {
        match num {
            0 => AppIndicatorStatus::Passive,
            1 => AppIndicatorStatus::Active,
            2 => AppIndicatorStatus::Attention,
            _ => panic!("invalid entry"),
        }
    }
}

impl AppIndicator {
    pub fn new(title: &str, icon: &str) -> AppIndicator {
        AppIndicator {
            air: unsafe {
                app_indicator_new(title.to_glib_none().0,
                                  icon.to_glib_none().0,
                                  AppIndicatorCategory::ApplicationStatus as u32)
            }
        }
    }

    pub fn with_path(title: &str, icon: &str, theme_path: &str) -> AppIndicator {
        AppIndicator {
            air: unsafe {
                app_indicator_new_with_path(title.to_glib_none().0,
                                            icon.to_glib_none().0,
                                            AppIndicatorCategory::ApplicationStatus as u32,
                                            theme_path.to_glib_none().0)
            }
        }
    }

    pub fn set_status(&mut self, status: AppIndicatorStatus) {
        unsafe {
            app_indicator_set_status(self.air, status as u32);
        }
    }

    pub fn set_menu(&mut self, menu: &mut gtk::Menu) {
        unsafe {
            app_indicator_set_menu(self.air, menu.to_glib_none().0);
        }
    }

    pub fn set_label(&mut self, label: &str, guide: &str) {
        unsafe {
            app_indicator_set_label(self.air, label.to_glib_none().0, guide.to_glib_none().0);
        }
    }

    pub fn set_title(&mut self, title: &str) {
        unsafe {
            app_indicator_set_title(self.air, title.to_glib_none().0);
        }
    }

    pub fn set_icon(&mut self, name: &str) {
        unsafe {
            app_indicator_set_icon(self.air, name.to_glib_none().0);
        }
    }
    pub fn set_icon_theme_path(&mut self, path: &str) {
        unsafe {
            app_indicator_set_icon_theme_path(self.air, path.to_glib_none().0);
        }
    }

    pub fn set_icon_full(&mut self, name: &str, desc: &str) {
        unsafe {
            app_indicator_set_icon_full(self.air, name.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    pub fn set_attention_icon(&mut self, name: &str) {
        unsafe {
            app_indicator_set_attention_icon(self.air, name.to_glib_none().0);
        }
    }

    pub fn set_attention_icon_full(&mut self, name: &str, desc: &str) {
        unsafe {
            app_indicator_set_attention_icon_full(self.air, name.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    pub fn set_secondary_activate_target(&mut self, menu_item: &mut gtk::Widget) {
        unsafe {
            app_indicator_set_secondary_activate_target(self.air, menu_item.to_glib_none().0);
        }
    }

    pub fn get_raw(&self) -> *mut AppIndicatorRaw {
        self.air
    }

    pub fn get_id(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{ 
            app_indicator_get_id(self.air)
        })
    }

    pub fn get_category(&self) -> AppIndicatorCategory {
        unsafe{ app_indicator_get_category(self.air) }.into()
    }

    pub fn get_status(&self) -> AppIndicatorStatus {
        unsafe{ app_indicator_get_status(self.air) }.into()
    }

    pub fn get_icon(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{
            app_indicator_get_icon(self.air) 
        })
    }

    pub fn get_icon_desc(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{
            app_indicator_get_icon_desc(self.air)
        })
    }

    pub fn get_icon_theme_path(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{
            app_indicator_get_icon_theme_path(self.air)
        })
    }

    pub fn get_attention_icon(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{
            app_indicator_get_attention_icon(self.air)
        })
    }

    pub fn get_attention_icon_desc(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{
            app_indicator_get_attention_icon_desc(self.air)
        })
    }

    pub fn get_title(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{
            app_indicator_get_title(self.air)
        })
    }

    pub fn get_menu(&self) -> gtk::Menu {
        unsafe{ 
            let ptr = app_indicator_get_menu(self.air);
            gtk::Menu::from_glib_none(ptr)
        }
    }

    pub fn get_label(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{
            app_indicator_get_label(self.air)
        })
    }

    pub fn get_label_guide(&self) -> Result<String, Error> {
        cstr_to_string(unsafe{
            app_indicator_get_label_guide(self.air)
        })
    }

    pub fn get_ordering_index(&self) -> u32 {
        unsafe{ app_indicator_get_ordering_index(self.air) }
    }

    pub fn get_secondary_activate_target(&self) -> gtk::Widget {
        unsafe{
            let ptr = app_indicator_get_secondary_activate_target(self.air);
            gtk::Widget::from_glib_none(ptr)
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
