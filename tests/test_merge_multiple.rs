use std::fs::File;
use std::io::*;

use merge_images::{merge, waterfall};

fn data(name: &str) -> Vec<u8> {
    let mut f = File::open(format!("./test-data/{}", name)).unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();
    buf
}

#[test]
fn test_merge_18() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let f3 = data("3.png");
    let f4 = data("4.jpg");
    let f5 = data("5.png");
    let f6 = data("6.png");
    let f7 = data("7.png");
    let f8 = data("8.jpg");
    let f9 = data("9.jpg");
    let out_im = merge(&[
        &f1, &f2, &f3, &f4, &f5, &f6, &f7, &f8, &f9, //
        &f1, &f2, &f3, &f4, &f5, &f6, &f7, &f8, &f9,
    ])
    .unwrap();

    let mut output = File::create("output-18.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_waterfall() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let f3 = data("3.png");
    let f4 = data("4.jpg");
    let f5 = data("5.png");
    let f6 = data("6.png");
    let f7 = data("7.png");
    let f8 = data("8.jpg");
    let f9 = data("9.jpg");
    let f10 = data("e09ca4a57584181ce573e45079b524ff3859b9fb.jpg");
    let out_im = waterfall(&[
        &f1, &f2, &f3, &f4, &f5, &f6, &f7, &f8, &f9, &f10, //
        &f1, &f2, &f3, &f4, &f5, &f6, &f7, &f8, &f9, &f10,
    ])
    .unwrap();

    let mut output = File::create("output-waterfall.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}
