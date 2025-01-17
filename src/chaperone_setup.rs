use crate::{sys, Context};

use std::ffi::CString;
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr::null_mut;

pub struct ChaperoneSetupManager<'c> {
    ctx: PhantomData<&'c Context>,
    inner: Pin<&'c mut sys::IVRChaperoneSetup>,
}

impl<'c> ChaperoneSetupManager<'c> {
    pub(super) fn new(_ctx: &'c Context) -> Self {
        let inner = unsafe { Pin::new_unchecked(sys::VRChaperoneSetup().as_mut::<'c>().unwrap()) };
        Self {
            ctx: Default::default(),
            inner,
        }
    }

    // TODO: this outputs json, could we pass it directly to something that does json?
    pub fn export_live_to_buffer(&mut self) -> Option<CString> {
        let mut len = 0u32;
        // Passing null pointer here means it will merely write to the length parameter.
        let res = unsafe { self.inner.as_mut().ExportLiveToBuffer(null_mut(), &mut len) };
        if !res || len == 0 {
            return None;
        }

        let mut data = vec![0u8; len as usize];
        let res = unsafe {
            self.inner
                .as_mut()
                .ExportLiveToBuffer(data.as_mut_ptr() as *mut i8, &mut len)
        };
        if res {
            CString::from_vec_with_nul(data).ok()
        } else {
            None
        }
    }
}
