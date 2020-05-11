//
// Various attempts at trying to write a large matrix transpose function
// in a cache friendly way.
//
//
#![feature(register_tool)]

use std::time::Instant;

const MAX: usize = 8192;
const BLOCK_SIZE: usize = 64;
const BLOCKS: usize = MAX / BLOCK_SIZE;

mod rustc_output;

use unchecked_index::unchecked_index;

use std::ops::{Index, IndexMut};

pub struct Grid<T = i32>(Box<[T]>);

impl Grid<i32> {
    pub fn new() -> Self {
        Self(vec![0; MAX * MAX].into_boxed_slice())
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T; MAX];
    
    fn index(&self, index: usize) -> &Self::Output {
        let output = &self.0[index * MAX..(index + 1) * MAX];
        unsafe { &*(output as *const [T] as *const [T; MAX]) }
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let output = &mut self.0[index * MAX..(index + 1) * MAX];
        unsafe { &mut *(output as *mut [T] as *mut [T; MAX]) }
    }
}

///////////////////////// Slow non-loop-blocking transpose /////////////////////

#[allow(clippy::all)]
fn transpose_0(a: &mut Grid, b: &Grid) {    
    for i in 0..MAX {
        for j in 0..MAX {
            a[i][j] += b[j][i];
        }
    }
}

fn transpose_1(a: &mut Grid, b: &Grid) {    
    println!("Unimlemented");
/*
    for (i, row) in a.iter_mut().enumerate().take(MAX) {
        for j in 0..MAX {
            row[j] += b[j][i];
        }
    }
*/
}

///////////////////////// Fast loop-blocking transpose /////////////////////////

#[allow(clippy::needless_range_loop)]
fn transpose_2(a: &mut Grid, b: &Grid) {    
    for i in 00..(MAX / BLOCK_SIZE) {
        for j in 00..(MAX / BLOCK_SIZE) {
            for ii in (i * BLOCK_SIZE)..((i * BLOCK_SIZE) + BLOCK_SIZE) {
                for jj in (j * BLOCK_SIZE)..((j * BLOCK_SIZE) + BLOCK_SIZE) {
                    a[ii][jj] += b[jj][ii];
                }
            }
        }
    }
}

fn transpose_3(a: &mut Grid, b: &Grid) {    
    println!("Unimlemented");
/*
    for s in 0..BLOCKS {
        for t in 0..BLOCKS {
            a.iter_mut()
                .skip(s * BLOCK_SIZE)
                .take(BLOCK_SIZE)
                .enumerate()
                .for_each(|(i, row)| {
                    row.iter_mut()
                        .skip(t * BLOCK_SIZE)
                        .take(BLOCK_SIZE)
                        .enumerate()
                        .for_each(|(j, y)| {
                            *y += b[j + t * BLOCK_SIZE][i + s * BLOCK_SIZE];
                        });
                });
        }
    }
*/
}

#[allow(clippy::needless_range_loop)]
fn transpose_4(a: &mut Grid, b: &Grid) {    
    for i in (0..MAX).step_by(BLOCK_SIZE) {
        for j in (0..MAX).step_by(BLOCK_SIZE) {
            for ii in i..i + BLOCK_SIZE {
                for jj in j..j + BLOCK_SIZE {
                    a[ii][jj] += b[jj][ii];
                }
            }
        }
    }
}

fn transpose_5(a: &mut Grid, b: &Grid) {    
    unsafe {
        let mut a = unchecked_index(a);
        let b = unchecked_index(b);
        let mut i: usize = 0;
        while i < MAX {
            let mut j: usize = 0;
            while j < MAX {
                let mut ii = i;
                while ii < i + BLOCK_SIZE {
                    let mut jj = j;
                    while jj < j + BLOCK_SIZE {
                        a[ii][jj] += b[jj][ii];
                        jj += 1
                    }
                    ii += 1
                }
                j += BLOCK_SIZE
            }
            i += BLOCK_SIZE
        }
    }
}

fn transpose_6(a: &mut Grid, b: &Grid) {    
    unsafe {
        let mut i: usize = 0;
        while i < MAX {
            let mut j: usize = 0;
            while j < MAX {
                let mut ii = i;
                while ii < i + BLOCK_SIZE {
                    let mut jj = j;
                    while jj < j + BLOCK_SIZE {
                        a[ii][jj] += b[jj][ii];
                        jj += 1
                    }
                    ii += 1
                }
                j += BLOCK_SIZE
            }
            i += BLOCK_SIZE
        }
    }
}

fn fill_arrays(a: &mut Grid, b: &mut Grid) {    
    for i in 0..MAX {
        for j in 0..MAX {
            let val = (i * 1000000 + j) as i32;
            a[i][j] = val;
            b[j][i] = -val;
        }
    }
}

#[allow(dead_code)]
fn print_array(a: &Grid) {
    for i in 0..MAX {
        for j in 0..MAX {
            print!("{}, ", a[i][j]);
        }
        println!();
    }
}

pub fn main() {
    println!("Rust functions:");
    println!("MAX:        {:?}, ", MAX);
    println!("BLOCK_SIZE: {:?}, ", BLOCK_SIZE);

    let mut a = Grid::new();
    let mut b = Grid::new();

    fill_arrays(&mut a, &mut b);
    println!("");

    let futs = [
        transpose_0,
        transpose_1,
        transpose_2,
        transpose_3,
        transpose_4,
        transpose_5,
        transpose_6,
    ]
    .to_vec();

    for (i, fut) in futs.iter().enumerate() {
        fill_arrays(&mut a, &mut b);
        let then = Instant::now();
        fut(&mut a, &b);
        let elapsed = then.elapsed().as_millis();
        println!("transpose_{}:    {}ms", i, elapsed);
    }

    println!("C functions (via rustc):");
    rustc_output::main_();
}
