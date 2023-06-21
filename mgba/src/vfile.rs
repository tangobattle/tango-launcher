#[repr(transparent)]
pub struct VFile(*mut mgba_sys::VFile);

pub mod flags {
    pub const O_RDONLY: u32 = mgba_sys::O_RDONLY;
    pub const O_WRONLY: u32 = mgba_sys::O_WRONLY;
    pub const O_RDWR: u32 = mgba_sys::O_RDWR;
    pub const O_APPEND: u32 = mgba_sys::O_APPEND;
    pub const O_CREAT: u32 = mgba_sys::O_CREAT;
    pub const O_TRUNC: u32 = mgba_sys::O_TRUNC;
    pub const O_EXCL: u32 = mgba_sys::O_EXCL;
}

impl VFile {
    pub fn open(path: &std::path::Path, flags: u32) -> Result<Self, crate::Error> {
        let ptr = match path.to_str() {
            Some(path) => unsafe {
                // On Windows, VFileOpenFD will call MultiByteToWideChar then _wopen, so we can just pass it a UTF-8 string.
                // On every other platform, we just use UTF-8 strings directly because they're not silly like Windows.
                let path_cstr = std::ffi::CString::new(path.as_bytes()).unwrap();
                mgba_sys::VFileOpen(path_cstr.as_ptr(), flags as i32)
            },
            None => std::ptr::null_mut(),
        };
        if ptr.is_null() {
            return Err(crate::Error::CallFailed("VFileOpen"));
        }
        Ok(VFile(ptr))
    }

    pub fn open_memory(buf: &[u8]) -> Self {
        VFile(unsafe {
            mgba_sys::VFileMemChunk(
                buf as *const _ as *const std::ffi::c_void,
                buf.len() as mgba_sys::size_t,
            )
        })
    }

    pub(super) unsafe fn release(&mut self) -> *mut mgba_sys::VFile {
        let ptr = self.0;
        self.0 = std::ptr::null_mut();
        ptr
    }
}

impl Drop for VFile {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }
        unsafe {
            (*self.0).close.unwrap()(self.0);
        }
    }
}
