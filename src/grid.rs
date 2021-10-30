use crate::prelude::*;
use crate::utils;
use crate::PAD;

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

    // 生成画布
    let ((width, height), poses) = image_poses(image_bytes.len());
    debug!("canvas size: {} x {}", width, height);
    let canvas = Mat::new_rows_cols_with_default(
        height,
        width,
        cv_core::CV_8UC3,
        cv_core::Scalar::all(255.),
    )?;
    debug!("canvas = {:?}", canvas);

    for (idx, (bytes, pos)) in image_bytes.iter().zip(poses).enumerate() {
        let im = utils::imdecode_wrapped(bytes.as_ref()).map_err(|e| {
            info!("error imdecode the {}-th bytes (0 based index): {}", idx, e);
            debug!("{:?}", e);
            e
        })?;
        info!("image size: {:?}", im.size()?);

        debug!("pos = {:?}", pos);
        let im = match utils::process_image(im, pos.width, pos.height) {
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
