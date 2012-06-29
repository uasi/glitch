use std;

fn glitch(bytes: [mut u8]/~, table: [u8]/~, offset: uint) {
    for uint::range(offset, bytes.len()) {|i|
        bytes[i] = table[bytes[i] as uint];
    }
}

fn main(args: [str]/~) {
    if args.len() < 3u { io::println("usage: glitch src dst"); ret; }
    let src = copy args[1];
    let dst = copy args[2];
    let r = io::read_whole_file(src);
    if r.is_err() { #error("%s",copy  r.get_err()); ret; }
    let bytes = vec::to_mut(result::get(r));
    let table = vec::to_mut(vec::from_fn(256u) {|i| i as u8 });
    table[18u8] = 1u8;
    glitch(bytes, vec::from_mut(table), 1024u);
    let w = io::file_writer(dst, [io::create, io::truncate]/~);
    if w.is_err() { #error("%s", result::get_err(w)); ret; }
    result::get(w).write(bytes);
}
