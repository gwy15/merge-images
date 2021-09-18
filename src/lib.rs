use log::*;
use opencv::core::{Rect, Vector};
use opencv::prelude::*;
use opencv::{Error, Result};

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
    opencv::imgproc::resize(
        &im,
        &mut resized,
        opencv::core::Size::new(width, height),
        0.,
        0.,
        opencv::imgproc::INTER_LINEAR,
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
        let im = opencv::imgcodecs::imdecode(&src, opencv::imgcodecs::IMREAD_COLOR)?;

        cv_images.push(im);
    }

    // 宽，高
    let ((width, height), poses) = match image_bytes.len() {
        2 => ((1800, 900), vec![(0, 0, 900, 900), (900, 0, 900, 900)]),
        // 1 + 2
        3 => (
            (1800, 2700),
            vec![
                (0, 0, 1800, 1800),
                (0, 1800, 900, 900),
                (900, 1800, 900, 900),
            ],
        ),
        4 => (
            (1800, 1800),
            vec![
                (0, 0, 900, 900),
                (0, 900, 900, 900),
                (900, 0, 900, 900),
                (900, 900, 900, 900),
            ],
        ),
        // 1800x600 + 1800x900
        5 => (
            (1800, 1500),
            vec![
                (0, 0, 900, 900),
                (900, 0, 900, 900),
                (0, 900, 600, 600),
                (600, 900, 600, 600),
                (1200, 900, 600, 600),
            ],
        ),
        // 3x2
        6 => (
            (1800, 1200),
            vec![
                (0, 0, 600, 600),
                (600, 0, 600, 600),
                (1200, 0, 600, 600),
                (0, 600, 600, 600),
                (600, 600, 600, 600),
                (1200, 600, 600, 600),
            ],
        ),
        // 2 + 2 + 3
        7 => (
            (1800, 2400),
            vec![
                (0, 0, 900, 900),
                (900, 0, 900, 900),
                (0, 900, 900, 900),
                (900, 900, 900, 900),
                (0, 1800, 600, 600),
                (600, 1800, 600, 600),
                (1200, 1800, 600, 600),
            ],
        ),
        // 2 + 3 + 3
        8 => (
            (1800, 2100),
            vec![
                (0, 0, 900, 900),
                (900, 0, 900, 900),
                (0, 900, 600, 600),
                (600, 900, 600, 600),
                (1200, 900, 600, 600),
                (0, 1500, 600, 600),
                (600, 1500, 600, 600),
                (1200, 1500, 600, 600),
            ],
        ),
        // 九宫图
        9 => (
            (1800, 1800),
            vec![
                (0, 0, 600, 600),
                (600, 0, 600, 600),
                (1200, 0, 600, 600),
                (0, 600, 600, 600),
                (600, 600, 600, 600),
                (1200, 600, 600, 600),
                (0, 1200, 600, 600),
                (600, 1200, 600, 600),
                (1200, 1200, 600, 600),
            ],
        ),
        _ => unreachable!(),
    };
    let canvas = Mat::zeros(height, width, opencv::core::CV_8UC3)?.to_mat()?;
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
    opencv::imgcodecs::imencode(".jpg", &canvas, &mut buf, &flags)?;

    Ok(buf.to_vec())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
