#![feature(register_tool)]

use std::time::Instant;

const MAX: usize = 8192;
const BLOCK_SIZE: usize = 64;
const BLOCKS: usize = MAX / BLOCK_SIZE;

mod output;

use unchecked_index::unchecked_index;

type T = [[i32; MAX]];

#[allow(clippy::all)]
fn do_it_0(a: &mut T, b: &T) {
    for i in 0..MAX {
        for j in 0..MAX {
            a[i][j] += b[j][i];
        }
    }
}

fn do_it_1(a: &mut T, b: &T) {
    for (i, row) in a.iter_mut().enumerate().take(MAX) {
        for j in 0..MAX {
            row[j] += b[j][i];
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn do_it_2(a: &mut T, b: &T) {
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

fn do_it_3(a: &mut T, b: &T) {
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
}

#[allow(clippy::needless_range_loop)]
fn do_it_4(a: &mut T, b: &T) {
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

fn do_it_5 (a: &mut T, b: &T) {
    unsafe {
        let mut a = unchecked_index(a);
        let mut b = unchecked_index(b);
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

fn do_it_6 (a: &mut T, b: &T) {
    unsafe {
        let mut i: usize = 0;
        while i < MAX {
            let mut j: usize = 0;
            while j < MAX {
                let mut ii = i;
                while ii < i + BLOCK_SIZE {
                    let mut jj = j;
                    while jj < j + BLOCK_SIZE {
                        *a.get_unchecked_mut(ii).get_unchecked_mut(jj) += 
                        b.get_unchecked(jj).get_unchecked(ii);
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

fn fill_arrays(a: &mut T, b: &mut T) {
    for (i, row) in a.iter_mut().enumerate().take(MAX) {
        let mut val = 0i32;
        for j in 0..MAX {
            row[j] = val;
            b[j][i] = -val;
            val += 1i32;
        }
    }
}

#[allow(dead_code)]
fn print_array(a: &T) {
    for row in a.iter().take(32) {
        for elem in row.iter().take(32) {
            print!("{}, ", elem);
        }
        println!();
    }
}

fn main() {
    println!("MAX:        {:?}, ", MAX);
    println!("BLOCK_SIZE: {:?}, ", BLOCK_SIZE);

    let mut a = vec![[9i32; MAX]; MAX];
    let mut b = vec![[10i32; MAX]; MAX];

    let futs = [do_it_0, do_it_1, do_it_2, do_it_3, do_it_4, do_it_5, do_it_6].to_vec();

    for (i, fut) in futs.iter().enumerate() {
        fill_arrays(&mut a, &mut b);
        let then = Instant::now();
        fut(&mut a, &b);
        let elapsed = then.elapsed().as_micros();
        println!("do_it_{}:    {}us", i, elapsed);
        //print_array(&a);
    }
    output::main_();
}
