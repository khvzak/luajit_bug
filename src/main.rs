mod ffi {
    use std::os::raw::{c_int, c_void};

    type LuaCFn = unsafe extern "C" fn(L: *mut c_void) -> c_int;

    extern "C" {
        pub fn luaL_newstate() -> *mut c_void;
        pub fn lua_pcall(L: *mut c_void, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
        pub fn lua_gettable(L: *mut c_void, idx: c_int);
        pub fn lua_pushcclosure(L: *mut c_void, f: LuaCFn, n: c_int);
        pub fn lua_pushnil(L: *mut c_void);
    }

    // #[inline(always)]
    pub unsafe fn lua_gettable2(state: *mut c_void, idx: c_int) {
        lua_gettable(state, idx);
    }
}

pub unsafe fn test_me() {
    let state = ffi::luaL_newstate();
    assert!(!state.is_null());

    unsafe extern "C" fn run_me(state: *mut std::os::raw::c_void) -> std::os::raw::c_int {
        ffi::lua_pushnil(state);
        ffi::lua_pushnil(state);
        ffi::lua_gettable2(state, -2); // Changing this to `ffi::lua_gettable` solves the problem
        1
    }

    ffi::lua_pushcclosure(state, run_me, 0);
    assert!(ffi::lua_pcall(state, 0, -1, 0) != 0);
}

fn main() {
    unsafe { test_me() }
}
