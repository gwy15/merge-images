use std::io::Write;

use crate::prelude::*;

/// 把图片处理成 (width, height) 大小
fn process_image(im: Mat, square: bool, width: i32, height: i32) -> Result<Mat> {
    debug!(
        "processing image into size ({}, {}), square = {}",
        width, height, square
    );
    let roi = if square {
        // 从中间选择正方形区域
        let min_size = im.rows().min(im.cols());
        if min_size == im.rows() {
            // 高度短，横向裁剪
            let x = (im.cols() - min_size) / 2;
            Rect::new(x, 0, min_size, min_size)
        } else {
            let y = (im.rows() - min_size) / 2;
            Rect::new(0, y, min_size, min_size)
        }
    } else {
        Rect::new(0, 0, im.cols(), im.rows())
    };

    let im = Mat::roi(&im, roi)?;

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

fn read_image_or_first_frame(bytes: &[u8]) -> Result<Mat> {
    match imagesize::image_type(&bytes) {
        Ok(imagesize::ImageType::Gif) => {
            // get first frame
            use tempfile::NamedTempFile;
            let mut file = NamedTempFile::new()
                .map_err(|e| Error::new(-1, format!("failed to open tempfile: {}", e)))?;

            file.write_all(bytes).unwrap();
            file.flush().unwrap();
            let path = file.path().as_os_str().to_string_lossy();
            let path = path.as_ref();
            let mut mat = Mat::default();
            let mut gif = opencv::videoio::VideoCapture::from_file(path, 0)?;
            if gif.read(&mut mat)? {
                info!("read frame from gif success");
                Ok(mat)
            } else {
                Err(Error::new(-2, "read frame from gif failed".to_string()))
            }
        }
        _ => imdecode_wrapped(bytes),
    }
}

pub(crate) fn merge_<T: AsRef<[u8]>>(
    image_bytes: &[T],
    gen_poses: impl Fn(&[T]) -> Result<((i32, i32), Vec<Rect>)>,
    square: bool,
) -> Result<Vec<u8>> {
    debug!("merging {} images", image_bytes.len());
    if image_bytes.is_empty() {
        return Err(Error::new(1, "no images".to_string()));
    }
    if image_bytes.len() == 1 {
        return Ok(image_bytes[0].as_ref().to_vec());
    }

    // 生成画布
    let ((width, height), poses) = gen_poses(image_bytes)?;
    debug!("canvas size: {} x {}", width, height);
    let canvas = Mat::new_rows_cols_with_default(
        height,
        width,
        cv_core::CV_8UC3,
        cv_core::Scalar::all(255.),
    )?;
    debug!("canvas = {:?}", canvas);

    for (idx, (bytes, pos)) in image_bytes.iter().zip(poses).enumerate() {
        let im = read_image_or_first_frame(bytes.as_ref()).map_err(|e| {
            info!("error imdecode the {}-th bytes (0 based index): {}", idx, e);
            debug!("{:?}", e);
            e
        })?;
        info!("image size: {:?}", im.size()?);

        debug!("pos = {:?}", pos);
        let im = match process_image(im, square, pos.width, pos.height) {
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
