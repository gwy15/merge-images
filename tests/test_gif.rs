use opencv::core::{Mat, MatTraitConstManual};
use opencv::videoio;
use opencv::videoio::VideoCaptureTrait;
use opencv::Result;

#[test]
fn test_gif_from_file() -> Result<()> {
    pretty_env_logger::try_init().ok();
    let mut cap = videoio::VideoCapture::from_file("./test-data/A.gif", 0)?;
    let mut output = Mat::default();
    if cap.read(&mut output)? {
        log::info!("read success: {:?}", output.size()?);
    } else {
        panic!("no frame read");
    }
    Ok(())
}
