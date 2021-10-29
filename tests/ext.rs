use oh_my_rust::ext::*;

mod io {
    use super::*;

    const SAMPLE_TEXT: &'static [u8] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".as_bytes();

    #[test]
    fn read_to_end_alloc() {
        let mut source = SAMPLE_TEXT;
        let result = source.read_to_end_alloc().unwrap();
        assert_eq!(&result, SAMPLE_TEXT)
    }

    #[test]
    fn read_to_string_alloc() {
        let mut source = SAMPLE_TEXT;
        let result = source.read_to_string_alloc().unwrap();
        assert_eq!(result.as_bytes(), SAMPLE_TEXT)
    }

    #[test]
    fn read_exact_alloc() {
        let mut source = SAMPLE_TEXT;
        let result = source.read_exact_alloc(32).unwrap();
        assert_eq!(result, SAMPLE_TEXT[..32]);
        assert_eq!(source[..], SAMPLE_TEXT[32..])
    }
}

