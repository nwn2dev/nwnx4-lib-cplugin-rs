#![allow(dead_code, non_camel_case_types, non_snake_case)]

use std::error::Error;
// use std::ffi;
// use std::ffi::{c_void, CStr, CString};
// use std::ffi::CStr;
use std::fmt;
// use std::fs::File;
// use std::io::Read;
// use std::os::raw::c_char;
// use std::path::PathBuf;
// use std::slice;

#[allow(unused_imports)]
use log::{error, info, trace, warn};

pub struct InitInfo<'a> {
    pub dll_path: &'a str,
    pub nwnx_user_path: &'a str,
    pub nwn2_install_path: &'a str,
    pub nwn2_home_path: &'a str,
    pub nwn2_module_path: &'a str,
    pub nwnx_install_path: &'a str,
}

#[derive(Debug)]
struct UnimplementedError {
    func_name: &'static str,
}
impl std::error::Error for UnimplementedError {}
impl fmt::Display for UnimplementedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function is not implemented in CPlugin trait")
    }
}

pub enum COptStr<'a> {
    CStr(&'a std::ffi::CStr),
    Str(&'a str),
}

#[allow(unused_variables)]
pub trait CPlugin<'a>: Sized {
    fn new(info: InitInfo) -> Result<Self, Box<dyn Error>>;
    fn get_id(&mut self) -> &'a std::ffi::CStr {
        &std::ffi::CStr::from_bytes_with_nul("\0".as_bytes()).unwrap()
    }
    fn get_info() -> &'static std::ffi::CStr {
        &std::ffi::CStr::from_bytes_with_nul("\0".as_bytes()).unwrap()
    }
    fn get_version() -> &'static std::ffi::CStr {
        &std::ffi::CStr::from_bytes_with_nul("\0".as_bytes()).unwrap()
    }
    fn get_int(
        &mut self,
        function: &str,
        param1: &str,
        param2: i32,
    ) -> Result<i32, Box<dyn Error>> {
        Err(Box::new(UnimplementedError {
            func_name: "get_int",
        }))
    }
    fn set_int(
        &mut self,
        function: &str,
        param1: &str,
        param2: i32,
        value: i32,
    ) -> Result<(), Box<dyn Error>> {
        Err(Box::new(UnimplementedError {
            func_name: "set_int",
        }))
    }
    fn get_float(
        &mut self,
        function: &str,
        param1: &str,
        param2: i32,
    ) -> Result<f32, Box<dyn Error>> {
        Err(Box::new(UnimplementedError {
            func_name: "get_float",
        }))
    }
    fn set_float(
        &mut self,
        function: &str,
        param1: &str,
        param2: i32,
        value: f32,
    ) -> Result<(), Box<dyn Error>> {
        Err(Box::new(UnimplementedError {
            func_name: "set_float",
        }))
    }
    fn get_str(
        &mut self,
        function: &str,
        param1: &str,
        param2: i32,
    ) -> Result<&str, Box<dyn Error>> {
        Err(Box::new(UnimplementedError {
            func_name: "get_str",
        }))
    }
    fn set_str(
        &mut self,
        function: &str,
        param1: &str,
        param2: i32,
        value: &str,
    ) -> Result<(), Box<dyn Error>> {
        Err(Box::new(UnimplementedError {
            func_name: "set_str",
        }))
    }
    fn get_gff_size(&mut self, var_name: &str) -> Result<usize, Box<dyn Error>> {
        Err(Box::new(UnimplementedError {
            func_name: "get_gff_size",
        }))
    }
    fn get_gff(&mut self, var_name: &str, buffer: &mut [u8]) {}
    fn set_gff(&mut self, var_name: &str, value: &[u8]) -> Result<(), Box<dyn Error>> {
        Err(Box::new(UnimplementedError {
            func_name: "set_gff",
        }))
    }
}

#[derive(Debug)]
pub enum CPluginEndpoints {
    get_id,
    get_info,
    get_version,
    get_int,
    set_int,
    get_float,
    set_float,
    get_str,
    set_str,
    get_gff_size,
    get_gff,
    set_gff,
}

#[macro_export]
macro_rules! cplugin_hook {
    ($plugin_class:ty, [$($endpoints:ident),*]) => {

        #[repr(C)]
        #[derive(Debug)]
        struct NWNXCPlugin_InitInfo {
            pub dll_path: *const std::os::raw::c_char,
            pub nwnx_user_path: *const std::os::raw::c_char,
            pub nwn2_install_path: *const std::os::raw::c_char,
            pub nwn2_home_path: *const std::os::raw::c_char,
            pub nwn2_module_path: *const std::os::raw::c_char,
            pub nwnx_install_path: *const std::os::raw::c_char,
        }

        #[allow(non_upper_case_globals)]
        #[no_mangle]
        static nwnxcplugin_abi_version: u32 = 1;

        #[no_mangle]
        extern "C" fn NWNXCPlugin_New(info: NWNXCPlugin_InitInfo) -> *mut std::ffi::c_void {
            use nwnx4_lib_cplugin_rs::{InitInfo, CPlugin};

            let dll_path = unsafe { std::ffi::CStr::from_ptr(info.dll_path).to_str().unwrap_or("") };
            let nwnx_user_path = unsafe { std::ffi::CStr::from_ptr(info.nwnx_user_path).to_str().unwrap_or("") };
            let nwn2_install_path = unsafe {
                std::ffi::CStr::from_ptr(info.nwn2_install_path)
                    .to_str()
                    .unwrap_or("")
            };
            let nwn2_home_path =
                unsafe { std::ffi::CStr::from_ptr(info.nwn2_home_path).to_str().unwrap_or("") };
            let nwn2_module_path =
                unsafe { std::ffi::CStr::from_ptr(info.nwn2_module_path).to_str().unwrap_or("") };
            let nwnx_install_path =
                unsafe { std::ffi::CStr::from_ptr(info.nwnx_install_path).to_str().unwrap_or("") };
            let init_info = InitInfo {
                dll_path,
                nwnx_user_path,
                nwn2_install_path,
                nwn2_home_path,
                nwn2_module_path,
                nwnx_install_path,
            };

            match <$plugin_class>::new(init_info) {
                Ok(plugin) => {
                    let plugin = std::boxed::Box::new(plugin);
                    log::trace!(
                        "NWNXCPlugin_New(dll_path={:?}, nwnx_user_path={:?}, nwn2_install_path={:?}, nwn2_home_path={:?}, nwn2_module_path={:?}, nwnx_install_path={:?}) -> {:p}",
                        dll_path,
                        nwnx_user_path,
                        nwn2_install_path,
                        nwn2_home_path,
                        nwn2_module_path,
                        nwnx_install_path,
                        plugin,
                    );
                    std::boxed::Box::into_raw(plugin) as *mut _ as *mut std::ffi::c_void
                },
                Err(e) => {
                    log::error!("NWNXCPlugin_New(dll_path={:?}, nwnx_user_path={:?}, nwn2_install_path={:?}, nwn2_home_path={:?}, nwn2_module_path={:?}, nwnx_install_path={:?}) -> Error {}",
                        dll_path,
                        nwnx_user_path,
                        nwn2_install_path,
                        nwn2_home_path,
                        nwn2_module_path,
                        nwnx_install_path,
                        e,
                    );
                    std::ptr::null_mut()
                },
            }
        }

        #[no_mangle]
        #[allow(unused_attributes)]
        pub extern "C" fn NWNXCPlugin_Delete(cplugin: *mut std::ffi::c_void) {
            unsafe { Box::from_raw(cplugin) };
        }

        // Implement functions
        $(
             cplugin_hook!($plugin_class, implement $endpoints);
        )*
    };

    ($plugin_class:ty, implement new) => {
        compile_error!("New and Delete are always implemented");
    };

    ($plugin_class:ty, implement delete) => {
        compile_error!("New and Delete are always implemented");
    };

    ($plugin_class:ty, implement get_id) => {
        #[no_mangle]
        pub extern "C" fn NWNXCPlugin_GetID(cplugin: *mut std::ffi::c_void) -> *const std::os::raw::c_char {
            use nwnx4_lib_cplugin_rs::CPlugin;
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            plugin.get_id().as_ptr()
        }
    };

    ($plugin_class:ty, implement get_version) => {
        #[no_mangle]
        pub extern "C" fn NWNXCPlugin_GetVersion() -> *const std::os::raw::c_char {
            use nwnx4_lib_cplugin_rs::CPlugin;
            <$plugin_class>::get_version().as_ptr()
        }
    };

    ($plugin_class:ty, implement get_info) => {
        #[no_mangle]
        pub extern "C" fn NWNXCPlugin_GetInfo() -> *const std::os::raw::c_char {
            use nwnx4_lib_cplugin_rs::CPlugin;
            <$plugin_class>::get_info().as_ptr()
        }
    };

    ($plugin_class:ty, implement get_int) => {
        #[no_mangle]
        #[allow(unused_attributes)]
        pub fn NWNXCPlugin_GetInt(
            cplugin: *mut std::ffi::c_void,
            sFunction: *const std::os::raw::c_char,
            sParam1: *const std::os::raw::c_char,
            nParam2: i32,
        ) -> i32 {
            use nwnx4_lib_cplugin_rs::CPlugin;
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let function: &str = unsafe { std::ffi::CStr::from_ptr(sFunction).to_str().unwrap_or("") };
            let param1: &str = unsafe { std::ffi::CStr::from_ptr(sParam1).to_str().unwrap_or("") };
            log::trace!(
                "NWNXCPlugin_GetInt({:p}, {:?}, {:?}, {})",
                plugin,
                function,
                param1,
                nParam2,
            );
            match plugin.get_int(function, param1, nParam2) {
                Ok(i) => i,
                Err(e) => {
                    log::error!("NWNXCPlugin_GetInt -> Error {}", e);
                    0
                }
            }
        }
    };

    ($plugin_class:ty, implement set_int) => {
        #[no_mangle]
        #[allow(unused_attributes)]
        pub fn NWNXCPlugin_SetInt(
            cplugin: *mut std::ffi::c_void,
            sFunction: *const std::os::raw::c_char,
            sParam1: *const std::os::raw::c_char,
            nParam2: i32,
            nValue: i32,
        ) {
            use nwnx4_lib_cplugin_rs::CPlugin;
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let function: &str = unsafe { std::ffi::CStr::from_ptr(sFunction).to_str().unwrap_or("") };
            let param1: &str = unsafe { std::ffi::CStr::from_ptr(sParam1).to_str().unwrap_or("") };
            log::trace!(
                "NWNXCPlugin_SetInt({:p}, {:?}, {:?}, {}, {})",
                plugin,
                function,
                param1,
                nParam2,
                nValue,
            );
            match plugin.set_int(function, param1, nParam2, nValue) {
                Ok(()) => {},
                Err(e) => {
                    log::error!("NWNXCPlugin_SetInt -> Error {}", e);
                }
            }
        }
    };

    ($plugin_class:ty, implement get_float) => {
        #[no_mangle]
        #[allow(unused_attributes)]
        pub fn NWNXCPlugin_GetFloat(
            cplugin: *mut std::ffi::c_void,
            sFunction: *const std::os::raw::c_char,
            sParam1: *const std::os::raw::c_char,
            nParam2: i32,
        ) -> f32 {
            use nwnx4_lib_cplugin_rs::CPlugin;
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let function: &str = unsafe { std::ffi::CStr::from_ptr(sFunction).to_str().unwrap_or("") };
            let param1: &str = unsafe { std::ffi::CStr::from_ptr(sParam1).to_str().unwrap_or("") };
            log::trace!(
                "NWNXCPlugin_GetFloat({:p}, {:?}, {:?}, {})",
                plugin,
                function,
                param1,
                nParam2,
            );
            match plugin.get_float(function, param1, nParam2) {
                Ok(f) => f,
                Err(e) => {
                    log::error!("NWNXCPlugin_GetFloat -> Error {}", e);
                    0.0
                }
            }
        }
    };

    ($plugin_class:ty, implement set_float) => {
        #[no_mangle]
        #[allow(unused_attributes)]
        pub fn NWNXCPlugin_SetFloat(
            cplugin: *mut std::ffi::c_void,
            sFunction: *const std::os::raw::c_char,
            sParam1: *const std::os::raw::c_char,
            nParam2: i32,
            fValue: f32,
        ) {
            use nwnx4_lib_cplugin_rs::CPlugin;
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let function: &str = unsafe { std::ffi::CStr::from_ptr(sFunction).to_str().unwrap_or("") };
            let param1: &str = unsafe { std::ffi::CStr::from_ptr(sParam1).to_str().unwrap_or("") };
            log::trace!(
                "NWNXCPlugin_SetFloat({:p}, {:?}, {:?}, {}, {})",
                plugin,
                function,
                param1,
                nParam2,
                fValue,
            );
            match plugin.set_float(function, param1, nParam2, fValue) {
                Ok(()) => {},
                Err(e) => {
                    log::error!("NWNXCPlugin_SetFloat -> Error {}", e);
                }
            }
        }
    };

    ($plugin_class:ty, implement get_str) => {

        #[no_mangle]
        #[allow(unused_attributes)]
        pub extern "C" fn NWNXCPlugin_GetString(
            cplugin: *mut std::ffi::c_void,
            sFunction: *const std::os::raw::c_char,
            sParam1: *const std::os::raw::c_char,
            nParam2: i32,
            result: *mut std::os::raw::c_char,
            resultSize: usize,
        ) {
            use nwnx4_lib_cplugin_rs::{CPlugin, COptStr};
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let function: &str = unsafe { std::ffi::CStr::from_ptr(sFunction).to_str().unwrap_or("") };
            let param1: &str = unsafe { std::ffi::CStr::from_ptr(sParam1).to_str().unwrap_or("") };
            let result_bytes = unsafe { slice::from_raw_parts_mut(result as *mut u8, resultSize) };
            log::trace!(
                "NWNXCPlugin_GetString({:p}, {:?}, {:?}, {})",
                plugin,
                function,
                param1,
                nParam2
            );

            match plugin.get_str(function, param1, nParam2) {
                Ok(s) => {
                    log::error!("plugin.get_str returned {:?}", s);

                    let bytes = s.as_bytes();
                    if bytes.len() + 1 < resultSize {
                        result_bytes[..bytes.len()].copy_from_slice(bytes);
                        result_bytes[bytes.len()] = 0;
                    } else{
                        log::error!(
                            "NWNXCPlugin_GetString -> Data is too long to fit result buffer. buffer_length={} result_length={}",
                            resultSize,
                            bytes.len() + 1
                        );
                    }
                },
                Err(e) => {
                    log::error!("NWNXCPlugin_GetString -> Error {}", e);
                    result_bytes[0] = 0;
                }
            }
        }
    };

    ($plugin_class:ty, implement set_str) => {
        #[no_mangle]
        #[allow(unused_attributes)]
        pub fn NWNXCPlugin_SetString(
            cplugin: *mut std::ffi::c_void,
            sFunction: *const std::os::raw::c_char,
            sParam1: *const std::os::raw::c_char,
            nParam2: i32,
            sValue: *const std::os::raw::c_char,
        ) {
            use nwnx4_lib_cplugin_rs::CPlugin;
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let function: &str = unsafe { std::ffi::CStr::from_ptr(sFunction).to_str().unwrap_or("") };
            let param1: &str = unsafe { std::ffi::CStr::from_ptr(sParam1).to_str().unwrap_or("") };
            let value: &str = unsafe { std::ffi::CStr::from_ptr(sValue).to_str().unwrap_or("") };
            log::trace!(
                "NWNXCPlugin_SetString({:p}, {:?}, {:?}, {}, {})",
                plugin,
                function,
                param1,
                nParam2,
                value,
            );
            match plugin.set_str(function, param1, nParam2, value) {
                Ok(()) => {},
                Err(e) => {
                    log::error!("NWNXCPlugin_SetString -> Error {}", e);
                }
            }
        }
    };

    ($plugin_class:ty, implement get_gff_size) => {
        #[no_mangle]
        #[allow(unused_attributes)]
        pub fn NWNXCPlugin_GetGFFSize(
            cplugin: *mut std::ffi::c_void,
            sVarName: *const std::os::raw::c_char,
        ) -> usize {
            use nwnx4_lib_cplugin_rs::CPlugin;
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let varname: &str = unsafe { std::ffi::CStr::from_ptr(sVarName).to_str().unwrap_or("") };
            log::trace!(
                "NWNXCPlugin_GetGFFSize({:p}, {:?})",
                plugin,
                varname,
            );
            match plugin.get_gff_size(varname) {
                Ok(len) => { len },
                Err(e) => {
                    log::error!("NWNXCPlugin_GetGFFSize -> Error {}", e);
                    0
                }
            }
        }
    };

    ($plugin_class:ty, implement get_gff) => {
        #[no_mangle]
        #[allow(unused_attributes)]
        pub extern "C" fn NWNXCPlugin_GetGFF(
            cplugin: *mut std::ffi::c_void,
            sVarName: *const std::os::raw::c_char,
            gffData: *mut u8,
            gffDataSize: usize,
        ) {
            use nwnx4_lib_cplugin_rs::{CPlugin, COptStr};
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let varname: &str = unsafe { std::ffi::CStr::from_ptr(sVarName).to_str().unwrap_or("") };
            let gff_data = unsafe { slice::from_raw_parts_mut(gffData as *mut u8, gffDataSize) };
            log::trace!(
                "NWNXCPlugin_GetGFF({:p}, {:?})",
                plugin,
                varname,
            );

            plugin.get_gff(varname, gff_data);
        }
    };

    ($plugin_class:ty, implement set_gff) => {
        #[no_mangle]
        #[allow(unused_attributes)]
        pub fn NWNXCPlugin_SetGFF(
            cplugin: *mut std::ffi::c_void,
            sVarName: *const std::os::raw::c_char,
            gffData: *const u8,
            gffDataSize: usize,
        ) {
            use nwnx4_lib_cplugin_rs::CPlugin;
            let plugin: &mut $plugin_class = unsafe { &mut *(cplugin as *mut _ as *mut $plugin_class) };
            let varname: &str = unsafe { std::ffi::CStr::from_ptr(sVarName).to_str().unwrap_or("") };
            let gff_data = unsafe { slice::from_raw_parts(gffData as *const u8, gffDataSize) };
            log::trace!(
                "NWNXCPlugin_SetGFF({:p}, {:?}, {:?})",
                plugin,
                varname,
                std::str::from_utf8(gff_data.get(0..8).unwrap_or(&[])).unwrap_or(""),
            );
            match plugin.set_gff(varname, gff_data) {
                Ok(()) => {},
                Err(e) => {
                    log::error!("NWNXCPlugin_SetGFF -> Error {}", e);
                }
            }
        }
    };

    ($plugin_class:ty, inject $endpoint:ident) => {
        compile_error!(stringify!(Unknown hook: $endpoint));
    };
}
