use oh_my_rust::ext::*;

mod io {
    use super::*;

    const SAMPLE_TEXT: &[u8] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod.".as_bytes();

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

mod uninit {
    use super::*;

    #[test]
    fn set_len_uninit_primitive() {
        let mut a = vec![1, 2, 3];
        a.set_len_uninit_primitive(4);
        println!("{:?}", a);
    }
}

mod pointer {
    use std::{cell::Cell, rc::Rc};

    use super::*;

    #[test]
    fn leak_and_reclaim() {
        let x = Rc::new(Cell::new(3));
        
        struct A(Rc<Cell<u8>>);
        impl Drop for A {
            fn drop(&mut self) {
                self.0.set(4)
            }
        }

        let a = A(x.clone()).box_and_into_raw();
        assert_eq!(x.get(), 3);
        unsafe { a.reclaim_box() };
        assert_eq!(x.get(), 4)
    }
}

mod other {
    use super::*;

    #[test]
    fn ignore() {
        let _ = ().ignore();
    }

    #[test]
    fn apply() {
        let x = 4.apply(|x| *x += 1);
        assert_eq!(x, 5)
    }
}