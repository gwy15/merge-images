use std::fs::File;
use std::io::*;

use merge_images::merge;

fn data(name: &str) -> Vec<u8> {
    let mut f = File::open(format!("./test-data/{}", name)).unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();
    buf
}

#[test]
fn test_merge_0() {
    pretty_env_logger::try_init().ok();
    let empty: &[&[u8]] = &[];
    assert!(merge(empty).is_err());
}

#[test]
fn test_merge_1() {
    pretty_env_logger::try_init().ok();
    let f = data("1.png");
    let out_im = merge(&[f]).unwrap();

    let mut output = File::create("output-1.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_2() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let out_im = merge(&[f1, f2]).unwrap();

    let mut output = File::create("output-2.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_3() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let f3 = data("3.png");
    let out_im = merge(&[f1, f2, f3]).unwrap();

    let mut output = File::create("output-3.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_4() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let f3 = data("3.png");
    let f4 = data("4.jpg");
    let out_im = merge(&[f1, f2, f3, f4]).unwrap();

    let mut output = File::create("output-4.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_5() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let f3 = data("3.png");
    let f4 = data("4.jpg");
    let f5 = data("5.png");
    let out_im = merge(&[f1, f2, f3, f4, f5]).unwrap();

    let mut output = File::create("output-5.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_6() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let f3 = data("3.png");
    let f4 = data("4.jpg");
    let f5 = data("5.png");
    let f6 = data("6.png");
    let out_im = merge(&[f1, f2, f3, f4, f5, f6]).unwrap();

    let mut output = File::create("output-6.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_7() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let f3 = data("3.png");
    let f4 = data("4.jpg");
    let f5 = data("5.png");
    let f6 = data("6.png");
    let f7 = data("7.png");
    let out_im = merge(&[f1, f2, f3, f4, f5, f6, f7]).unwrap();

    let mut output = File::create("output-7.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_8() {
    pretty_env_logger::try_init().ok();
    let f1 = data("1.png");
    let f2 = data("2.png");
    let f3 = data("3.png");
    let f4 = data("4.jpg");
    let f5 = data("5.png");
    let f6 = data("6.png");
    let f7 = data("7.png");
    let f8 = data("8.jpg");
    let out_im = merge(&[f1, f2, f3, f4, f5, f6, f7, f8]).unwrap();

    let mut output = File::create("output-8.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}

#[test]
fn test_merge_9() {
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
    let out_im = merge(&[f1, f2, f3, f4, f5, f6, f7, f8, f9]).unwrap();

    let mut output = File::create("output-9.jpg").unwrap();
    output.write_all(&out_im).unwrap();
}
