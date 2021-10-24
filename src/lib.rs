use log::*;
use opencv::{
    core::{self as cv_core, prelude::*, Rect, Vector},
    imgcodecs, imgproc,
    prelude::*,
    Error, Result,
};

const PAD: i32 = 10;

/// 把图片处理成 (width, height) 大小
fn process_image(im: Mat, width: i32, height: i32) -> Result<Mat> {
    debug!("processing image into size ({}, {})", width, height);
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

    let resize_result = imgproc::resize(
        &im,
        &mut resized,
        cv_core::Size::new(width, height),
        0.,
        0.,
        imgproc::INTER_LINEAR,
    );
    match resize_result {
        Ok(_) => {}
        Err(e) => {
            warn!("failed to resize image: {}", e);
            return Err(e);
        }
    }

    debug!("image resized");
    Ok(resized)
}

/// 生成大于 9 图时的略缩图位置
fn batch_image_poses(n: usize) -> ((i32, i32), Vec<Rect>) {
    debug_assert!(n > 9);
    let (columns, per_size) = match n {
        0..=9 => (3, 800),
        10..=16 => (4, 500),
        17..=25 => (5, 400),
        26..=36 => (6, 350),
        37..=49 => (7, 300),
        50..=64 => (8, 300),
        65..=81 => (9, 300),
        _ => (10, 240),
    };
    let rows = (n as i32 + columns - 1) / columns;

    let width = (columns * per_size) + PAD * (columns - 1);
    let height = (rows * per_size) + PAD * (rows - 1);

    let mut rects = vec![];
    for i in 0..n as i32 {
        let x = (i % columns) * (per_size + PAD);
        let row = i / columns;
        let y = row * (per_size + PAD);
        rects.push(Rect::new(x, y, per_size, per_size));
    }

    debug!("width = {}, height = {}", width, height);
    trace!("rects = {:?}", rects);
    ((width, height), rects)
}

/// 生成 2~9 图时的略缩图位置
/// return ((width, height), poses)
fn image_poses(n: usize) -> ((i32, i32), Vec<Rect>) {
    debug_assert!(n >= 1);

    macro_rules! rects {
        ($(
            ($x:expr, $y:expr, $width:expr, $height:expr),
        )*) => {
            vec![
                $(
                    Rect::new($x, $y, $width, $height),
                )*
            ]
        };
    }

    match n {
        1 => {
            error!("生成略缩图只有 n=1");
            ((1800, 1800), rects![(0, 0, 1800, 1800),])
        }
        2 => (
            (1800 + PAD, 900),
            rects![(0, 0, 900, 900), (900 + PAD, 0, 900, 900),],
        ),
        // 1 + 2
        3 => (
            (1800 + PAD, 2700 + PAD),
            rects![
                (0, 0, 1800, 1800),
                (0, 1800 + PAD, 900, 900),
                (900 + PAD, 1800 + PAD, 900, 900),
            ],
        ),
        4 => (
            (1800 + PAD, 1800 + PAD),
            rects![
                (0, 0, 900, 900),
                (0, 900 + PAD, 900, 900),
                (900 + PAD, 0, 900, 900),
                (900 + PAD, 900 + PAD, 900, 900),
            ],
        ),
        // 1800x600 + 1800x900
        5 => (
            (1800 + 2 * PAD, 1500 + PAD),
            rects![
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
            rects![
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
            rects![
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
            rects![
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
            rects![
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
        n => batch_image_poses(n),
    }
}

/// 返回一张拼图，格式为 jpg
pub fn merge<T: AsRef<[u8]>>(image_bytes: &[T]) -> Result<Vec<u8>> {
    debug!("merging {} images", image_bytes.len());
    if image_bytes.is_empty() {
        return Err(Error::new(1, "no images".to_string()));
    }
    if image_bytes.len() == 1 {
        return Ok(image_bytes[0].as_ref().to_vec());
    }

    let mut cv_images = vec![];
    for (idx, bytes) in image_bytes.iter().enumerate() {
        let src = Mat::from_slice(bytes.as_ref()).map_err(|e| {
            info!("Mat::from_slice error: {}", e);
            debug!("{:?}", e);
            e
        })?;
        let im_decode_result = match imagesize::blob_size(bytes.as_ref()) {
            Ok(imagesize::ImageSize { width, height }) => {
                let flag = match width.max(height) {
                    size if size > 8000 => {
                        info!("size too big: ({}x{}), shrink to 1/8", width, height);
                        imgcodecs::IMREAD_REDUCED_COLOR_8
                    }
                    size if size > 3000 => {
                        info!("size too big: ({}x{}), shrink to 1/4", width, height);
                        imgcodecs::IMREAD_REDUCED_COLOR_4
                    }
                    _ => imgcodecs::IMREAD_COLOR,
                };
                imgcodecs::imdecode(&src, flag)
            }
            Err(e) => {
                warn!("cannot get image size in advance: {:?}", e);
                imgcodecs::imdecode(&src, imgcodecs::IMREAD_COLOR)
            }
        };
        let im = im_decode_result.map_err(|e| {
            info!("error imdecode the {}-th bytes (0 based index): {}", idx, e);
            debug!("{:?}", e);
            e
        })?;
        info!("image size: {:?}", im.size()?);

        cv_images.push(im);
    }

    // 宽，高
    let ((width, height), poses) = image_poses(image_bytes.len());
    let canvas = Mat::new_rows_cols_with_default(
        height,
        width,
        cv_core::CV_8UC3,
        cv_core::Scalar::all(255.),
    )?;
    debug!("canvas = {:?}", canvas);
    // copy
    for (idx, (im, pos)) in cv_images.into_iter().zip(poses).enumerate() {
        debug!("pos = {:?}", pos);
        let im = match process_image(im, pos.width, pos.height) {
            Ok(im) => im,
            Err(e) => {
                info!("failed to process the {}-th image: {}. continue", idx, e);
                debug!("cause: {:?}", e);
                continue;
            }
        };

        let mut roi = Mat::roi(&canvas, pos)?;
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
