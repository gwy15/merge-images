use log::*;
use opencv::{
    core::{self as cv_core, prelude::*, Rect, Vector},
    imgcodecs, imgproc,
    prelude::*,
    Error, Result,
};

const PAD: i32 = 10;

fn process_image(im: Mat, width: i32, height: i32) -> Result<Mat> {
    // select from center
    let min_size = im.rows().min(im.cols());
    let (x, y) = if min_size == im.rows() {
        // 高度短，横向裁剪
        let x = (im.cols() - min_size) / 2;
        (x, 0)
    } else {
        let y = (im.rows() - min_size) / 2;
        (0, y)
    };
    let im = Mat::roi(&im, Rect::new(x, y, min_size, min_size))?;

    let mut resized = Mat::default();
    imgproc::resize(
        &im,
        &mut resized,
        cv_core::Size::new(width, height),
        0.,
        0.,
        imgproc::INTER_LINEAR,
    )?;
    debug!("image resized");
    Ok(resized)
}

/// 返回一张拼图，格式为 jpg
pub fn merge(image_bytes: &[Vec<u8>]) -> Result<Vec<u8>> {
    if image_bytes.is_empty() {
        return Err(Error::new(1, "no images".to_string()));
    }
    if image_bytes.len() > 9 {
        return Err(Error::new(
            1,
            format!("too many images: {}", image_bytes.len()),
        ));
    }
    if image_bytes.len() == 1 {
        return Ok(image_bytes[0].clone());
    }

    let mut cv_images = vec![];
    for bytes in image_bytes {
        let src = Mat::from_slice(bytes)?;
        let im = imgcodecs::imdecode(&src, imgcodecs::IMREAD_COLOR)?;

        cv_images.push(im);
    }

    // 宽，高
    let ((width, height), poses) = match image_bytes.len() {
        2 => (
            (1800 + PAD, 900),
            vec![(0, 0, 900, 900), (900 + PAD, 0, 900, 900)],
        ),
        // 1 + 2
        3 => (
            (1800 + PAD, 2700 + PAD),
            vec![
                (0, 0, 1800, 1800),
                (0, 1800 + PAD, 900, 900),
                (900 + PAD, 1800 + PAD, 900, 900),
            ],
        ),
        4 => (
            (1800 + PAD, 1800 + PAD),
            vec![
                (0, 0, 900, 900),
                (0, 900 + PAD, 900, 900),
                (900 + PAD, 0, 900, 900),
                (900 + PAD, 900 + PAD, 900, 900),
            ],
        ),
        // 1800x600 + 1800x900
        5 => (
            (1800 + 2 * PAD, 1500 + PAD),
            vec![
                (0, 0, 900, 900),
                (900 + PAD, 0, 900, 900),
                (0, 900 + PAD, 600, 600),
                (600 + PAD, 900 + PAD, 600, 600),
                (1200 + 2 * PAD, 900 + PAD, 600, 600),
            ],
        ),
        // 3x2
        6 => (
            (1800 + 2 * PAD, 1200 + PAD),
            vec![
                (0, 0, 600, 600),
                (600 + PAD, 0, 600, 600),
                (1200 + 2 * PAD, 0, 600, 600),
                (0, 600 + PAD, 600, 600),
                (600 + PAD, 600 + PAD, 600, 600),
                (1200 + 2 * PAD, 600 + PAD, 600, 600),
            ],
        ),
        // 2 + 2 + 3
        7 => (
            (1800 + 2 * PAD, 2400 + 2 * PAD),
            vec![
                (0, 0, 900, 900),
                (900 + PAD, 0, 900, 900),
                (0, 900 + PAD, 900, 900),
                (900 + PAD, 900 + PAD, 900, 900),
                (0, 1800 + 2 * PAD, 600, 600),
                (600 + PAD, 1800 + 2 * PAD, 600, 600),
                (1200 + 2 * PAD, 1800 + 2 * PAD, 600, 600),
            ],
        ),
        // 2 + 3 + 3
        8 => (
            (1800 + 2 * PAD, 2100 + 2 * PAD),
            vec![
                (0, 0, 900, 900),
                (900 + PAD, 0, 900, 900),
                (0, 900 + PAD, 600, 600),
                (600 + PAD, 900 + PAD, 600, 600),
                (1200 + 2 * PAD, 900 + PAD, 600, 600),
                (0, 1500 + 2 * PAD, 600, 600),
                (600 + PAD, 1500 + 2 * PAD, 600, 600),
                (1200 + 2 * PAD, 1500 + 2 * PAD, 600, 600),
            ],
        ),
        // 九宫图
        9 => (
            (1800 + 2 * PAD, 1800 + 2 * PAD),
            vec![
                (0, 0, 600, 600),
                (600 + PAD, 0, 600, 600),
                (1200 + 2 * PAD, 0, 600, 600),
                (0, 600 + PAD, 600, 600),
                (600 + PAD, 600 + PAD, 600, 600),
                (1200 + 2 * PAD, 600 + PAD, 600, 600),
                (0, 1200 + 2 * PAD, 600, 600),
                (600 + PAD, 1200 + 2 * PAD, 600, 600),
                (1200 + 2 * PAD, 1200 + 2 * PAD, 600, 600),
            ],
        ),
        _ => unreachable!(),
    };
    let canvas = Mat::new_rows_cols_with_default(
        height,
        width,
        cv_core::CV_8UC3,
        cv_core::Scalar::all(255.),
    )?;
    debug!("canvas = {:?}", canvas);
    // copy
    for (im, pos) in cv_images.into_iter().zip(poses) {
        debug!("pos = {:?}", pos);
        let (x, y, dx, dy) = pos;

        let im = process_image(im, dx, dy)?;

        let mut roi = Mat::roi(&canvas, Rect::new(x, y, dx, dy))?;
        debug!("image copy: src = {:?}, roi = {:?}", im, roi);

        im.copy_to(&mut roi)?;
    }

    let mut buf = Vector::new();
    let flags = Vector::new();
    imgcodecs::imencode(".jpg", &canvas, &mut buf, &flags)?;

    Ok(buf.to_vec())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
