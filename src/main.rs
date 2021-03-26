mod ffi {
    use std::os::raw::{c_char, c_int, c_void};

    pub const LUA_GLOBALSINDEX: c_int = -10002;

    extern "C" {
        pub fn luaL_newstate() -> *mut c_void;
        pub fn luaL_openlibs(state: *mut c_void);
        pub fn luaL_loadstring(L: *mut c_void, s: *const c_char) -> c_int;
        pub fn lua_pcall(L: *mut c_void, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
        pub fn lua_getfield(state: *mut c_void, index: c_int, k: *const c_char);
        pub fn lua_gettable(L: *mut c_void, idx: c_int);
        pub fn lua_pushcclosure(
            L: *mut c_void,
            f: unsafe extern "C" fn(L: *mut c_void) -> c_int,
            n: c_int,
        );
        pub fn lua_pushinteger(L: *mut c_void, n: c_int);
        pub fn lua_pushvalue(L: *mut c_void, idx: c_int);
    }

    // #[inline(always)]
    pub unsafe fn lua_gettable2(state: *mut c_void, idx: c_int) {
        lua_gettable(state, idx);
    }
}

macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0") as *const str as *const [::std::os::raw::c_char]
            as *const ::std::os::raw::c_char
    };
}

pub unsafe fn test_me() {
    let state = ffi::luaL_newstate();
    assert!(!state.is_null());
    ffi::luaL_openlibs(state);

    let status = ffi::luaL_loadstring(
        state,
        cstr!(
            r#"
        t = setmetatable({}, {__index = function() error("lua error") end})
    "#
        ),
    );
    assert!(status == 0);
    assert!(ffi::lua_pcall(state, 0, -1, 0) == 0);

    ffi::lua_getfield(state, ffi::LUA_GLOBALSINDEX, cstr!("t"));

    unsafe extern "C" fn get_table(state: *mut std::os::raw::c_void) -> std::os::raw::c_int {
        ffi::lua_pushvalue(state, -2);
        ffi::lua_pushinteger(state, 1);
        ffi::lua_gettable2(state, -2); // Changing this to `ffi::lua_gettable` solves the problem
        1
    }
    ffi::lua_pushcclosure(state, get_table, 0);
    ffi::lua_pcall(state, 0, -1, 0);
}

fn main() {
    unsafe { test_me() }
}
