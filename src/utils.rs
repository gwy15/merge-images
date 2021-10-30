use crate::prelude::*;

/// 把图片处理成 (width, height) 大小
pub fn process_image(im: Mat, width: i32, height: i32) -> Result<Mat> {
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

/// 对 imdecode 简单地包装了一下，避免在遇到尺寸过大的图像时内存溢出。
pub fn imdecode_wrapped(bytes: &[u8]) -> Result<Mat> {
    let src = Mat::from_slice(bytes).map_err(|e| {
        info!("Mat::from_slice error: {}", e);
        debug!("{:?}", e);
        e
    })?;
    let im_decode_result = match imagesize::blob_size(bytes) {
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
    let im = im_decode_result?;

    let (width, height) = (im.cols(), im.rows());
    let im = match width.max(height) {
        #[cfg(debug_assertions)]
        size if size > 8000 => {
            panic!("表现不一致：大小还是超过 8000");
        }
        #[cfg(not(debug_assertions))]
        size if size > 8000 => {
            error!("表现不一致：大小还是超过 8000；继续缩放为 1/8");
            let mut output = Mat::default();
            imgproc::resize(
                &im,
                &mut output,
                cv_core::Size::new(0, 0),
                1. / 8.,
                1. / 8.,
                imgproc::INTER_LINEAR,
            )?;
            std::mem::drop(im);
            output
        }
        _ => im,
    };

    Ok(im)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const F: &str = "./test-data/e09ca4a57584181ce573e45079b524ff3859b9fb.jpg";

    #[test]
    fn test_imdecode_wrapped() {
        let im = opencv::imgcodecs::imread(F, opencv::imgcodecs::IMREAD_REDUCED_COLOR_8).unwrap();
        assert_eq!(im.cols(), 1440);
        assert_eq!(im.rows(), 2048);

        let mut f = std::fs::File::open(F).unwrap();
        let mut buf = vec![];
        f.read_to_end(&mut buf).unwrap();

        let im = imdecode_wrapped(&buf).unwrap();
        assert_eq!(im.cols(), 1440);
        assert_eq!(im.rows(), 2048);
    }
}
