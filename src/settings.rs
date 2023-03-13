use std::{ffi::CStr, mem};

use openvr_sys as sys;
use Settings;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVRSettingsError(sys::EVRSettingsError);

impl Settings {
    pub fn set_float(
        &self,
        pch_section: &CStr,
        pch_settings_key: &CStr,
        fl_value: f32,
    ) -> Result<(), EVRSettingsError> {
        unsafe {
            let mut error: EVRSettingsError = mem::uninitialized();
            self.0.SetFloat.unwrap()(
                pch_section.as_ptr() as *mut _,
                pch_settings_key.as_ptr() as *mut _,
                fl_value,
                &mut error.0,
            );
            if error != EVRSettingsError(sys::EVRSettingsError_VRSettingsError_None) {
                return Err(error);
            }
            return Ok(());
        }
    }

    pub fn get_float(&self, pch_section: &CStr, pch_settings_key: &CStr) -> Result<f32, EVRSettingsError> {
        unsafe {
            let mut error: EVRSettingsError = mem::uninitialized();
            let result = self.0.GetFloat.unwrap()(
                pch_section.as_ptr() as *mut _,
                pch_settings_key.as_ptr() as *mut _,
                &mut error.0,
            );
            if error != EVRSettingsError(sys::EVRSettingsError_VRSettingsError_None) {
                return Err(error);
            }
            return Ok(result);
        }
    }
}
