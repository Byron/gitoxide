mod deflate {
    use crate::zlib::stream::{deflate::Deflate, Status};
    use miniz_oxide::MZFlush;

    #[test]
    fn compress_all_data_at_once() {
        let mut buf = [0u8; 16];
        assert_eq!(
            Deflate::default()
                .compress(b"hello", &mut buf, MZFlush::Finish)
                .unwrap(),
            Status::StreamEnd
        );
    }

    #[test]
    fn output_too_small() {
        let mut buf = [0u8; 6];
        let mut deflate = Deflate::default();
        let input = b"hello";
        assert_eq!(deflate.compress(input, &mut buf, MZFlush::Finish).unwrap(), Status::Ok);
        assert_eq!(deflate.total_in, 5);
        assert_eq!(deflate.total_out, 6);
        assert_eq!(
            deflate
                .compress(&input[deflate.total_in as usize..], &mut buf, MZFlush::Finish)
                .unwrap(),
            Status::Ok
        );
        assert_eq!(deflate.total_in, 5);
        assert!(deflate.total_out == 13 || deflate.total_out == 12);
        assert_eq!(
            deflate
                .compress(&input[deflate.total_in as usize..], &mut buf, MZFlush::Finish)
                .unwrap(),
            Status::StreamEnd
        );
        assert!(deflate.total_out == 13 || deflate.total_out == 12);
    }

    #[test]
    fn multiple_inputs_sufficient_output() {
        let mut buf = [0u8; 32];
        let mut deflate = Deflate::default();
        let input = b"hello";
        let step = 2;
        let mut cur = 0;
        assert_eq!(
            deflate
                .compress(&input[cur..cur + step], &mut buf, MZFlush::None)
                .unwrap(),
            Status::Ok
        );
        assert_eq!(deflate.total_in, 2);
        assert_eq!(deflate.total_out, 0);
        cur += step;
        assert_eq!(
            deflate
                .compress(&input[cur..cur + step], &mut buf, MZFlush::None)
                .unwrap(),
            Status::Ok
        );
        assert_eq!(deflate.total_in, 4);
        assert_eq!(deflate.total_out, 0);
        cur += step;
        assert_eq!(
            deflate.compress(&input[cur..], &mut buf, MZFlush::Finish).unwrap(),
            Status::StreamEnd
        );
        assert_eq!(deflate.total_in, 5);
        assert!(deflate.total_out == 13 || deflate.total_out == 12);
    }
}
