mod integration_tests {
    use lib;

    #[test]
    fn _swallow() {
        let mut l = lib::_get_larger();
        let l_width = l._get_width();
        let l_height = l._get_height();
        let s = lib::_get_smaller();

        l._swallow(s);

        assert_ne!(l_width, l._get_width());
        assert_ne!(l_height, l._get_height());
    }
}