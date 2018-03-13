extern crate zimg_sys as ffi;
extern crate av_data as data;

use data::frame::*;
use ffi::zimg::*;

// TODO move the trait somewhere?
trait Rescale {
    /// Reconfigure the rescaler
    ///
    /// Optional call to avoid the lazy setup in `rescale`
    fn setup(&mut self, dst: &mut VideoInfo, src: &VideoInfo) -> Result<(), FrameError>;

    /// Rescale the `src` in `dst`
    ///
    /// It will lazily configure the rescaler if the source
    /// or the destination VideoInfo are different
    fn rescale(&mut self, dst: &mut ArcFrame, src: ArcFrame) -> Result<(), FrameError>;
}

struct ZImgImageFormat(zimg_image_format);

impl<'a> From<&'a VideoInfo> for ZImgImageFormat {
    fn from(v: &VideoInfo) -> Self {
        unimplemented!()
    }
}

struct ZImgImageBuffer(zimg_image_buffer);

impl<'a> From<&'a FrameBuffer> for ZImgImageBuffer {
    fn from(f: &FrameBuffer) -> Self {
        unimplemented!()
    }
}

pub struct ZImg {
    src_vinfo: Option<VideoInfo>,
    dst_vinfo: Option<VideoInfo>,
    graph: Option<*mut zimg_filter_graph>
}

impl ZImg {
    fn new() -> Self {
        unimplemented!()
    }
}
