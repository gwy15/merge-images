use crate::prelude::*;
use crate::utils;
use crate::PAD;
use imagesize::blob_size;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn image_poses<T: AsRef<[u8]>>(image_bytes: &[T]) -> Result<((i32, i32), Vec<Rect>)> {
    debug!("generating image poses for {} images", image_bytes.len());
    let mut ans = vec![];

    let (columns, per_size) = match image_bytes.len() {
        0..=9 => (2, 800),
        10..=16 => (3, 500),
        17..=25 => (4, 400),
        26..=36 => (5, 350),
        37..=49 => (6, 300),
        _ => (7, 300),
    };
    // 贪心, heap => (row, col(index))
    let mut heap = BinaryHeap::new();
    for i in 0..columns as usize {
        heap.push(Reverse((0, i)));
    }
    let mut image_height = 0;
    for image_byte in image_bytes {
        let size = blob_size(image_byte.as_ref()).map_err(|e| {
            info!("{:?}", e);
            Error::new(
                -1,
                format!("failed to get image size thru imagesize crate: {}", e),
            )
        })?;
        let resized_height = per_size * size.height as i32 / size.width as i32;
        // SAFETY: heap 一定不空
        let Reverse((row, col_index)) = heap.pop().unwrap();
        // 这个图占的位置：rows: (row, row+resized_height)
        let x = (per_size + PAD) * col_index as i32;
        let y = row;
        ans.push(Rect::new(x, y, per_size, resized_height));
        image_height = image_height.max(y + resized_height);
        heap.push(Reverse((y + resized_height + PAD, col_index)));
    }

    let width = per_size * columns + PAD * (columns - 1);
    let height = image_height;

    Ok(((width, height), ans))
}

pub fn merge<T: AsRef<[u8]>>(image_bytes: &[T]) -> Result<Vec<u8>> {
    debug!("merging {} images", image_bytes.len());
    if image_bytes.is_empty() {
        return Err(Error::new(1, "no images".to_string()));
    }
    if image_bytes.len() == 1 {
        return Ok(image_bytes[0].as_ref().to_vec());
    }

    utils::merge_(image_bytes, image_poses, false)
}
