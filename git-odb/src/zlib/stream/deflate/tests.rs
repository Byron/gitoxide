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
        let bytes =
            include_bytes!("../../../../tests/fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack");
        for chunk in bytes.chunks(2) {
            assert_eq!(w.write(chunk)?, chunk.len());
        }
        w.flush()?;

        assert_deflate_buffer(w.inner, bytes)
    }

    #[test]
    fn big_file_a_few_big_writes() -> Result<(), Box<dyn std::error::Error>> {
        let mut w = DeflateWriter::new(Vec::new());
        let bytes =
            include_bytes!("../../../../tests/fixtures/packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack");
        for chunk in bytes.chunks(4096 * 9) {
            assert_eq!(w.write(chunk)?, chunk.len());
        }
        w.flush()?;

        assert_deflate_buffer(w.inner, bytes)
    }
}

// mod deflate {
//     use flate2::FlushCompress;

//     #[test]
//     fn compress_all_data_at_once() {
//         let mut buf = [0u8; 16];
//         assert_eq!(
//             Deflate::default()
//                 .compress(b"hello", &mut buf, MZFlush::Finish)
//                 .expect("compression to memory to work"),
//             Status::StreamEnd
//         );
//     }

//     #[test]
//     fn output_too_small_try_finish() -> Result<(), Box<dyn std::error::Error>> {
//         let mut buf = [0u8; 6];
//         let mut deflate = Deflate::default();
//         let input = b"hello";
//         assert_eq!(deflate.compress(input, &mut buf, MZFlush::Finish)?, Status::Ok);
//         assert_eq!(deflate.total_in, 5);
//         assert_eq!(deflate.total_out, 6);
//         assert_eq!(
//             deflate.compress(&input[deflate.total_in as usize..], &mut buf, MZFlush::Finish)?,
//             Status::Ok
//         );
//         assert_eq!(deflate.total_in, 5);
//         assert!(deflate.total_out == 13 || deflate.total_out == 12);
//         assert_eq!(
//             deflate.compress(&input[deflate.total_in as usize..], &mut buf, MZFlush::Finish)?,
//             Status::StreamEnd
//         );
//         assert!(deflate.total_out == 13 || deflate.total_out == 12);
//         Ok(())
//     }

//     #[test]
//     fn output_too_small_do_not_try_to_finish() -> Result<(), Box<dyn std::error::Error>> {
//         let mut buf = [0u8; 6];
//         let mut deflate = Deflate::default();
//         let input = b"hellohellohellohellohellohellohellohellhellohellohellohellohellohellohellohellhellohellohellohellohellohellohellohellooohellohellohellohellohellohellohellohello";
//         assert_eq!(deflate.compress(input, &mut buf, MZFlush::None)?, Status::Ok);
//         assert_eq!(deflate.total_in, 160);
//         assert_eq!(deflate.total_out, 0);
//         assert_eq!(
//             deflate.compress(&input[deflate.total_in as usize..], &mut buf, MZFlush::None)?,
//             Status::BufError,
//             "the output buffer is too small to drop any information"
//         );
//         let mut buf = [0u8; 32];
//         assert_eq!(
//             deflate.compress(&input[deflate.total_in as usize..], &mut buf, MZFlush::None)?,
//             Status::BufError,
//             "after the first buf error, unless providing more input, probably nothing can be done"
//         );
//         assert_eq!(deflate.total_out, 0);
//         assert_eq!(
//             deflate.compress(&input[deflate.total_in as usize..], &mut buf, MZFlush::Finish)?,
//             Status::Ok,
//             "it wrote some data, but not all"
//         );
//         assert!(deflate.total_out == 31 || deflate.total_out == 32);
//         assert_eq!(
//             deflate.compress(&input[deflate.total_in as usize..], &mut buf, MZFlush::Finish)?,
//             Status::StreamEnd,
//         );
//         assert_eq!(deflate.total_out, 35);
//         Ok(())
//     }

//     #[test]
//     fn multiple_inputs_sufficient_output() -> Result<(), Box<dyn std::error::Error>> {
//         let mut buf = [0u8; 32];
//         let mut deflate = Deflate::default();
//         let input = b"hello";
//         let step = 2;
//         let mut cur = 0;
//         assert_eq!(
//             deflate.compress(&input[cur..cur + step], &mut buf, MZFlush::None)?,
//             Status::Ok
//         );
//         assert_eq!(deflate.total_in, 2);
//         assert_eq!(deflate.total_out, 0);
//         cur += step;
//         assert_eq!(
//             deflate.compress(&input[cur..cur + step], &mut buf, MZFlush::None)?,
//             Status::Ok
//         );
//         assert_eq!(deflate.total_in, 4);
//         assert_eq!(deflate.total_out, 0);
//         cur += step;
//         assert_eq!(
//             deflate.compress(&input[cur..], &mut buf, MZFlush::Finish)?,
//             Status::StreamEnd
//         );
//         assert_eq!(deflate.total_in, 5);
//         assert!(deflate.total_out == 13 || deflate.total_out == 12);
//         Ok(())
//     }
// }
