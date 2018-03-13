// TODO do w/out the unions?
#![feature(untagged_unions)]

pub mod zimg;

#[cfg(test)]
mod tests {
    use super::zimg::*;
    use std::ffi::*;
    #[test]
    fn version() {
        let mut major = 0;
        let mut minor = 0;

        unsafe {
            zimg_get_api_version(&mut major, &mut minor);
        }

        assert_eq!(major, ZIMG_API_VERSION_MAJOR);
        assert_eq!(minor, ZIMG_API_VERSION_MINOR);
    }
}
