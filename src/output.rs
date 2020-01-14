#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
    non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
extern "C" {
#[no_mangle]
fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
#[no_mangle]
fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
-> libc::c_int;
#[no_mangle]
fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
pub type int32_t = libc::c_int;
pub type uint64_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn do_it_0(mut a: *mut [int32_t; 8192],
                            mut b: *mut [int32_t; 8192]) {
let mut i: size_t = 0 as libc::c_int as size_t;
while i < 8192 as libc::c_int as libc::c_ulong {
   let mut j: size_t = 0 as libc::c_int as size_t;
   while j < 8192 as libc::c_int as libc::c_ulong {
       let ref mut fresh0 = (*a.offset(i as isize))[j as usize];
       *fresh0 += (*b.offset(j as isize))[i as usize];
       j = j.wrapping_add(1)
   }
   i = i.wrapping_add(1)
};
}
#[no_mangle]
pub unsafe extern "C" fn do_it_1(mut a: *mut [int32_t; 8192],
                            mut b: *mut [int32_t; 8192]) {
let mut i: size_t = 0 as libc::c_int as size_t;
while i < 8192 as libc::c_int as libc::c_ulong {
   let mut j: size_t = 0 as libc::c_int as size_t;
   while j < 8192 as libc::c_int as libc::c_ulong {
       let mut ii: size_t = i;
       while ii < i.wrapping_add(64 as libc::c_int as libc::c_ulong) {
           let mut jj: size_t = j;
           while jj < j.wrapping_add(64 as libc::c_int as libc::c_ulong)
                 {
               let ref mut fresh1 =
                   (*a.offset(ii as isize))[jj as usize];
               *fresh1 += (*b.offset(jj as isize))[ii as usize];
               jj = jj.wrapping_add(1)
           }
           ii = ii.wrapping_add(1)
       }
       j =
           (j as
                libc::c_ulong).wrapping_add(64 as libc::c_int as
                                                libc::c_ulong) as size_t
               as size_t
   }
   i =
       (i as
            libc::c_ulong).wrapping_add(64 as libc::c_int as
                                            libc::c_ulong) as size_t as
           size_t
};
}
#[no_mangle]
pub unsafe extern "C" fn fill_arrays(mut a: *mut [int32_t; 8192],
                                mut b: *mut [int32_t; 8192]) {
let mut count: int32_t = 0.0f64 as int32_t;
let mut i: size_t = 0 as libc::c_int as size_t;
while i < 8192 as libc::c_int as libc::c_ulong {
   let mut j: size_t = 0 as libc::c_int as size_t;
   while j < 8192 as libc::c_int as libc::c_ulong {
       (*a.offset(i as isize))[j as usize] = count;
       (*b.offset(j as isize))[i as usize] = -count;
       count = (count as libc::c_double + 1.0f64) as int32_t;
       j = j.wrapping_add(1)
   }
   i = i.wrapping_add(1)
};
}
#[no_mangle]
pub unsafe extern "C" fn print_array(mut a: *mut [int32_t; 8192]) {
let mut i: size_t = 0 as libc::c_int as size_t;
while i < 32 as libc::c_int as libc::c_ulong {
   let mut j: size_t = 0 as libc::c_int as size_t;
   while j < 32 as libc::c_int as libc::c_ulong {
       printf(b"%.0f, \x00" as *const u8 as *const libc::c_char,
              (*a.offset(i as isize))[j as usize]);
       j = j.wrapping_add(1)
   }
   printf(b"\n\x00" as *const u8 as *const libc::c_char);
   i = i.wrapping_add(1)
};
}
// Return a timestamp in nano second resolution.
#[no_mangle]
pub unsafe extern "C" fn time_ns() -> uint64_t {
let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
clock_gettime(1 as libc::c_int, &mut now);
return (now.tv_sec as
           uint64_t).wrapping_mul(1000000000 as
                                      libc::c_ulong).wrapping_add(now.tv_nsec
                                                                      as
                                                                      uint64_t);
}
unsafe fn main_0() -> libc::c_int {
printf(b"MAX:        %d\n\x00" as *const u8 as *const libc::c_char,
      8192 as libc::c_int);
printf(b"BLOCK_SIZE: %d\n\x00" as *const u8 as *const libc::c_char,
      64 as libc::c_int);
let mut a: *mut [int32_t; 67108864] =
   malloc(((8192 as libc::c_int * 8192 as libc::c_int) as
               libc::c_ulong).wrapping_mul(::std::mem::size_of::<int32_t>()
                                               as libc::c_ulong)) as
       *mut [int32_t; 67108864];
let mut b: *mut [int32_t; 67108864] =
   malloc(((8192 as libc::c_int * 8192 as libc::c_int) as
               libc::c_ulong).wrapping_mul(::std::mem::size_of::<int32_t>()
                                               as libc::c_ulong)) as
       *mut [int32_t; 67108864];
fill_arrays(a as *mut [int32_t; 8192], b as *mut [int32_t; 8192]);
let mut startTime: uint64_t = time_ns();
do_it_0(a as *mut [int32_t; 8192], b as *mut [int32_t; 8192]);
let mut endTime: uint64_t = time_ns();
printf(b"do_it_0:    %luus\n\x00" as *const u8 as *const libc::c_char,
      endTime.wrapping_sub(startTime).wrapping_div(1000 as libc::c_uint
                                                       as
                                                       libc::c_ulong));
fill_arrays(a as *mut [int32_t; 8192], b as *mut [int32_t; 8192]);
let mut startTime_0: uint64_t = time_ns();
do_it_1(a as *mut [int32_t; 8192], b as *mut [int32_t; 8192]);
let mut endTime_0: uint64_t = time_ns();
printf(b"do_it_1:    %luus\n\x00" as *const u8 as *const libc::c_char,
      endTime_0.wrapping_sub(startTime_0).wrapping_div(1000 as
                                                           libc::c_uint
                                                           as
                                                           libc::c_ulong));
return 0;
}

pub fn main_() { unsafe { ::std::process::exit(main_0() as i32) } }
//print_array(a);
//print_array(a);
