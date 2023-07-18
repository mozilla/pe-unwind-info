#![no_std]
#![feature(start, lang_items)]

#[link(name = "vcruntime")]
extern {}

#[link(name = "ucrt")]
extern {}

#[link(name = "msvcrt")]
extern {}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[inline(never)]
fn funca(a: u64, b: u64, c: u64, d: u64) -> u64 {
    let mut acc = 0;
    for x in d..100 {
        acc += funcc(funcb(a, d), funcb(b, c), x);
    }
    funcd(acc, c, c, d, acc, a)
}

#[inline(never)]
fn funcb(a: u64, b: u64) -> u64 {
    a * 3 + b
}

#[inline(never)]
fn funcc(a: u64, b: u64, c: u64) -> u64 {
    if c == 0 {
        a + b
    } else {
        funcc(b, a + b, c - 1)
    }
}

#[inline(never)]
fn funcd(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) -> u64 {
    let x = if a > b { (a + c) / d } else { (b + c) / d };
    x * (x % e) - f + funcc(b, c, d)
}

#[start]
fn main(c: isize, _v: *const *const u8) -> isize {
    funca(10, 12, 2, c as u64) as isize
}
