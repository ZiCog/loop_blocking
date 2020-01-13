#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint64_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn do_it_0(
    mut a: *mut [libc::c_float; 4096],
    mut b: *mut [libc::c_float; 4096],
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 4096 as libc::c_int {
            (*a.offset(i as isize))[j as usize] += (*b.offset(j as isize))[i as usize];
            j += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_it_1(
    mut a: *mut [libc::c_float; 4096],
    mut b: *mut [libc::c_float; 4096],
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 4096 as libc::c_int {
            let mut ii: libc::c_int = i;
            while ii < i + 64 as libc::c_int {
                let mut jj: libc::c_int = j;
                while jj < j + 64 as libc::c_int {
                    (*a.offset(ii as isize))[jj as usize] += (*b.offset(jj as isize))[ii as usize];
                    jj += 1
                }
                ii += 1
            }
            j += 64 as libc::c_int
        }
        i += 64 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn fill_arrays(
    mut a: *mut [libc::c_float; 4096],
    mut b: *mut [libc::c_float; 4096],
) {
    let mut count: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 4096 as libc::c_int {
            (*a.offset(i as isize))[j as usize] = count;
            (*b.offset(j as isize))[i as usize] = -count;
            count = (count as libc::c_double + 1.0f64) as libc::c_float;
            j += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_array(mut a: *mut [libc::c_float; 4096]) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 32 as libc::c_int {
            printf(
                b"%.0f, \x00" as *const u8 as *const libc::c_char,
                (*a.offset(i as isize))[j as usize] as libc::c_double,
            );
            j += 1
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
}
// Return a timestamp in nano second resolution.
#[no_mangle]
pub unsafe extern "C" fn time_ns() -> uint64_t {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 as libc::c_int, &mut now);
    return (now.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as libc::c_ulong)
        .wrapping_add(now.tv_nsec as uint64_t);
}
unsafe fn main_0() -> libc::c_int {
    printf(
        b"MAX:        %d\n\x00" as *const u8 as *const libc::c_char,
        4096 as libc::c_int,
    );
    printf(
        b"BLOCK_SIZE: %d\n\x00" as *const u8 as *const libc::c_char,
        64 as libc::c_int,
    );
    let mut a: *mut [libc::c_float; 4096] = malloc(
        ((4096 as libc::c_int * 4096 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut [libc::c_float; 4096];
    let mut b: *mut [libc::c_float; 4096] = malloc(
        ((4096 as libc::c_int * 4096 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut [libc::c_float; 4096];
    fill_arrays(a, b);
    let mut startTime: uint64_t = time_ns();
    do_it_0(a, b);
    let mut endTime: uint64_t = time_ns();
    printf(
        b"do_it_0:    %luus\n\x00" as *const u8 as *const libc::c_char,
        endTime
            .wrapping_sub(startTime)
            .wrapping_div(1000 as libc::c_uint as libc::c_ulong),
    );
    fill_arrays(a, b);
    let mut startTime_0: uint64_t = time_ns();
    do_it_1(a, b);
    let mut endTime_0: uint64_t = time_ns();
    printf(
        b"do_it_1:    %luus\n\x00" as *const u8 as *const libc::c_char,
        endTime_0
            .wrapping_sub(startTime_0)
            .wrapping_div(1000 as libc::c_uint as libc::c_ulong),
    );
    return 0;
}

pub fn main_c2rust() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
//print_array(a);
//print_array(a);
