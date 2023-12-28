use crate::*;

pub use decoder::VarStringEncodingDecoder;
pub use encoder::VarStringEncodingEncoder;

pub const ENCODED_LENGTH: usize = 4;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct VarStringEncodingEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for VarStringEncodingEncoder<P>
    where
        P: Writer<'a> + Default,
    {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> VarStringEncodingEncoder<P>
    where
        P: Writer<'a> + Default,
    {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field 'length'
        /// - min value: 0
        /// - max value: 1073741824
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 4
        #[inline]
        pub fn length(&mut self, value: u32) {
            let offset = self.offset;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'varData'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: ISO-8859-1
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 0
        #[inline]
        pub fn var_data(&mut self, value: u8) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u8_at(offset, value);
        }
    }
} // end encoder mod

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct VarStringEncodingDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for VarStringEncodingDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> VarStringEncodingDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn length(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        /// characterEncoding: 'ISO-8859-1'
        #[inline]
        pub fn var_data(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 4)
        }
    }
} // end decoder mod
