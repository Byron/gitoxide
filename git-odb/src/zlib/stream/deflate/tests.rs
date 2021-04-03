mod deflate_stream {
    use crate::zlib::stream::DeflateWriter;
    use crate::zlib::stream::InflateReader;
    use std::io::{Read, Write};

    #[test]
    fn all_at_once() -> Result<(), Box<dyn std::error::Error>> {
        let mut w = DeflateWriter::new(Vec::new());
        assert_eq!(w.write(b"hello")?, 5);
        w.flush()?;

        let out = w.inner;
        assert!(out.len() == 12 || out.len() == 13);

        assert_deflate_buffer(out, b"hello")
    }

    fn assert_deflate_buffer(out: Vec<u8>, expected: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut actual = Vec::new();
        InflateReader::from_read(out.as_slice()).read_to_end(&mut actual)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn big_file_small_writes() -> Result<(), Box<dyn std::error::Error>> {
        let mut w = DeflateWriter::new(Vec::new());
        let bytes = include_bytes!(
            "../../../../tests/fixtures/objects/pack/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack"
        );
        for chunk in bytes.chunks(2) {
            assert_eq!(w.write(chunk)?, chunk.len());
        }
        w.flush()?;

        assert_deflate_buffer(w.inner, bytes)
    }

    #[test]
    fn big_file_a_few_big_writes() -> Result<(), Box<dyn std::error::Error>> {
        let mut w = DeflateWriter::new(Vec::new());
        let bytes = include_bytes!(
            "../../../../tests/fixtures/objects/pack/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack"
        );
        for chunk in bytes.chunks(4096 * 9) {
            assert_eq!(w.write(chunk)?, chunk.len());
        }
        w.flush()?;

        assert_deflate_buffer(w.inner, bytes)
    }
}
