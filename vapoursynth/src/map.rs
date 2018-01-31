use std::ffi::CStr;
use std::marker::PhantomData;
use vapoursynth_sys as ffi;

use api::API;

// TODO: impl Eq on this stuff, impl Clone on Map, impl From for/to HashMap.

/// A non-owned non-mutable VapourSynth map.
#[derive(Debug, Clone, Copy)]
pub struct MapRef<'a> {
    api: API,
    handle: *const ffi::VSMap,
    owner: PhantomData<&'a ()>,
}

unsafe impl<'a> Send for MapRef<'a> {}
unsafe impl<'a> Sync for MapRef<'a> {}

impl<'a> MapRef<'a> {
    /// Wraps `handle` in a `MapRef`.
    ///
    /// # Safety
    /// The caller must ensure `handle` is valid and the provided owner's lifetime is correct for
    /// the given `handle`.
    pub(crate) unsafe fn from_ptr<T>(api: API, _owner: &'a T, handle: *const ffi::VSMap) -> Self {
        Self {
            api,
            handle,
            owner: PhantomData,
        }
    }
}

/// A non-owned mutable VapourSynth map.
#[derive(Debug)]
pub struct MapRefMut<'a> {
    api: API,
    handle: *mut ffi::VSMap,
    owner: PhantomData<&'a mut ()>,
}

unsafe impl<'a> Send for MapRefMut<'a> {}
unsafe impl<'a> Sync for MapRefMut<'a> {}

/// An owned VapourSynth map.
#[derive(Debug)]
pub struct Map {
    api: API,
    handle: *mut ffi::VSMap,
}

unsafe impl Send for Map {}
unsafe impl Sync for Map {}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            self.api.free_map(self.handle);
        }
    }
}

impl Map {
    /// Creates a new `Map`.
    pub fn new(api: API) -> Self {
        let handle = api.create_map();

        Self { api, handle }
    }
}

/// A non-mutable VapourSynth map.
pub trait VSMap: sealed::VSMapInterface {
    /// Returns the number of keys contained in a map.
    fn key_count(&self) -> usize {
        let count = unsafe { self.api().prop_num_keys(self.handle()) };
        assert!(count >= 0);
        count as usize
    }

    /// Returns a key from a map.
    ///
    /// # Panics
    /// Panics if `index >= self.key_count()`.
    fn key(&self, index: usize) -> &CStr {
        assert!(index < self.key_count());
        let index = index as i32;

        unsafe { CStr::from_ptr(self.api().prop_get_key(self.handle(), index)) }
    }
}

/// A mutable VapourSynth map.
pub trait VSMapMut: VSMap + sealed::VSMapMutInterface {
    fn clear(&mut self) {
        unsafe {
            self.api().clear_map(self.handle_mut());
        }
    }
}

impl<T> VSMap for T
where
    T: sealed::VSMapInterface,
{
}
impl<T> VSMapMut for T
where
    T: VSMap + sealed::VSMapMutInterface,
{
}

mod sealed {
    use super::*;

    /// An interface for a non-mutable VapourSynth map.
    pub trait VSMapInterface {
        fn api(&self) -> API;
        fn handle(&self) -> *const ffi::VSMap;
    }

    /// An interface for a mutable VapourSynth map.
    pub trait VSMapMutInterface: VSMapInterface {
        fn handle_mut(&mut self) -> *mut ffi::VSMap;
    }

    impl<'a> VSMapInterface for MapRef<'a> {
        fn api(&self) -> API {
            self.api
        }

        fn handle(&self) -> *const ffi::VSMap {
            self.handle
        }
    }

    impl<'a> VSMapInterface for MapRefMut<'a> {
        fn api(&self) -> API {
            self.api
        }

        fn handle(&self) -> *const ffi::VSMap {
            self.handle
        }
    }

    impl<'a> VSMapMutInterface for MapRefMut<'a> {
        fn handle_mut(&mut self) -> *mut ffi::VSMap {
            self.handle
        }
    }

    impl VSMapInterface for Map {
        fn api(&self) -> API {
            self.api
        }

        fn handle(&self) -> *const ffi::VSMap {
            self.handle
        }
    }

    impl VSMapMutInterface for Map {
        fn handle_mut(&mut self) -> *mut ffi::VSMap {
            self.handle
        }
    }
}
