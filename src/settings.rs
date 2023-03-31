use std::{ffi::CStr, mem::MaybeUninit};

use openvr_sys as sys;
use Settings;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVRSettingsError(sys::EVRSettingsError);

impl Settings {
    pub fn set_bool(
        &self,
        pch_section: &CStr,
        pch_settings_key: &CStr,
        b_value: bool,
    ) -> Result<(), EVRSettingsError> {
        unsafe {
            let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
            self.0.SetBool.unwrap()(
                pch_section.as_ptr() as *mut _,
                pch_settings_key.as_ptr() as *mut _,
                b_value,
                error.as_mut_ptr() as *mut sys::EVRSettingsError,
            );
            let error = error.assume_init();
            if error != EVRSettingsError(sys::EVRSettingsError_VRSettingsError_None) {
                return Err(error);
            }
            return Ok(());
        }
    }

    pub fn get_bool(
        &self,
        pch_section: &CStr,
        pch_settings_key: &CStr,
    ) -> Result<bool, EVRSettingsError> {
        unsafe {
            let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
            let result = self.0.GetBool.unwrap()(
                pch_section.as_ptr() as *mut _,
                pch_settings_key.as_ptr() as *mut _,
                error.as_mut_ptr() as *mut sys::EVRSettingsError,
            );
            let error = error.assume_init();
            if error != EVRSettingsError(sys::EVRSettingsError_VRSettingsError_None) {
                return Err(error);
            }
            return Ok(result);
        }
    }

    pub fn set_float(
        &self,
        pch_section: &CStr,
        pch_settings_key: &CStr,
        fl_value: f32,
    ) -> Result<(), EVRSettingsError> {
        unsafe {
            let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
            self.0.SetFloat.unwrap()(
                pch_section.as_ptr() as *mut _,
                pch_settings_key.as_ptr() as *mut _,
                fl_value,
                error.as_mut_ptr() as *mut sys::EVRSettingsError,
            );
            let error = error.assume_init();
            if error != EVRSettingsError(sys::EVRSettingsError_VRSettingsError_None) {
                return Err(error);
            }
            return Ok(());
        }
    }

    pub fn get_float(
        &self,
        pch_section: &CStr,
        pch_settings_key: &CStr,
    ) -> Result<f32, EVRSettingsError> {
        unsafe {
            let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
            let result = self.0.GetFloat.unwrap()(
                pch_section.as_ptr() as *mut _,
                pch_settings_key.as_ptr() as *mut _,
                error.as_mut_ptr() as *mut sys::EVRSettingsError,
            );
            let error = error.assume_init();
            if error != EVRSettingsError(sys::EVRSettingsError_VRSettingsError_None) {
                return Err(error);
            }
            return Ok(result);
        }
    }
}
