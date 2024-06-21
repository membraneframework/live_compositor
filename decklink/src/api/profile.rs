use log::warn;

use crate::DeckLinkError;

use super::ffi::{self, HResult};

pub struct ProfileAttributes(pub(super) *mut ffi::IDeckLinkProfileAttributes);

impl ProfileAttributes {
    pub fn get_flag(&self, id: ffi::FlagAttributeId) -> Result<Option<bool>, DeckLinkError> {
        let mut value = false;
        match unsafe { ffi::profile_attributes_flag(self.0, id, &mut value)? } {
            HResult::Ok => Ok(Some(value)),
            HResult::NotImplementedError => Ok(None),
            hresult => Err(DeckLinkError::ProfileAttributeError(format!(
                "Failed to get {id:?} (error_code: {:#x})",
                hresult.repr,
            ))),
        }
    }

    pub fn get_integer(&self, id: ffi::IntegerAttributeId) -> Result<Option<i64>, DeckLinkError> {
        let mut value: i64 = 0;
        match unsafe { ffi::profile_attributes_integer(self.0, id, &mut value)? } {
            HResult::Ok => Ok(Some(value)),
            HResult::NotImplementedError => Ok(None),
            hresult => Err(DeckLinkError::ProfileAttributeError(format!(
                "Failed to get {id:?} (error_code: {:#x})",
                hresult.repr,
            ))),
        }
    }

    pub fn get_float(&self, id: ffi::FloatAttributeId) -> Result<Option<f64>, DeckLinkError> {
        let mut value: f64 = 0.0;
        match unsafe { ffi::profile_attributes_float(self.0, id, &mut value)? } {
            HResult::Ok => Ok(Some(value)),
            HResult::NotImplementedError => Ok(None),
            hresult => Err(DeckLinkError::ProfileAttributeError(format!(
                "Failed to get {id:?} (error_code: {:#x})",
                hresult.repr,
            ))),
        }
    }

    pub fn get_string(&self, id: ffi::StringAttributeId) -> Result<Option<String>, DeckLinkError> {
        let mut value = String::new();
        match unsafe { ffi::profile_attributes_string(self.0, id, &mut value, false)? } {
            HResult::Ok => Ok(Some(value)),
            HResult::NotImplementedError => Ok(None),
            hresult => Err(DeckLinkError::ProfileAttributeError(format!(
                "Failed to get {id:?} (error_code: {:#x})",
                hresult.repr,
            ))),
        }
    }
}

impl Drop for ProfileAttributes {
    fn drop(&mut self) {
        let result = unsafe { ffi::profile_attributes_release(self.0) };
        if result != HResult::Ok {
            warn!("Error when releasing ProfileAttributes ({result:?}).")
        }
    }
}

pub struct ProfileManager(pub(super) *mut ffi::IDeckLinkProfileManager);

impl ProfileManager {
    pub fn profiles(&self) -> Result<Vec<Profile>, DeckLinkError> {
        let ptrs = unsafe { ffi::profile_manager_profiles(self.0) }?;
        Ok(ptrs
            .into_iter()
            .map(|wrapper| Profile(wrapper.ptr))
            .collect())
    }
}

impl Drop for ProfileManager {
    fn drop(&mut self) {
        unsafe { ffi::profile_manager_release(self.0) }
    }
}

pub struct Profile(*mut ffi::IDeckLinkProfile);

impl Profile {
    pub fn attributes(&self) -> Result<ProfileAttributes, DeckLinkError> {
        let attributes = unsafe { ffi::profile_profile_attributes(self.0) }?;
        Ok(ProfileAttributes(attributes))
    }

    pub fn is_active(&self) -> Result<bool, DeckLinkError> {
        Ok(unsafe { ffi::profile_is_active(self.0) }?)
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        unsafe { ffi::profile_release(self.0) }
    }
}
