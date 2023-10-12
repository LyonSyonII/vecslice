macro_rules! test {
    ( $( $(#[ $meta:meta ])* $name:ident => $code:expr )* ) => {
        $(
            #[test]
            $( #[$meta] )*
            fn $name() {
                $code;
            }
        )*
    }
}

use vecslice::Slice;

test! {
    new_tail0 => assert_eq!(vec![0; 0].vecslice_at_tail(), [])
    new_tail1 => assert_eq!(vec![1].vecslice_at_tail(), [])
    new_tail2 => assert_eq!(vec![1, 2].vecslice_at_tail(), [])
    new_tail3 => assert_eq!(vec![1, 2, 3].vecslice_at_tail(), [])

    new_range_unbound0 => assert_eq!(vec![0; 0].vecslice(..), [])
    new_range_unbound1 => assert_eq!(vec![1].vecslice(..), [1])
    new_range_unbound2 => assert_eq!(vec![1, 2].vecslice(..), [1, 2])
    new_range_unbound3 => assert_eq!(vec![1, 2, 3].vecslice(..), [1, 2, 3])

    new_range_unbound_left0 => assert_eq!(vec![0; 0].vecslice(..0), [])
    new_range_unbound_left1 => assert_eq!(vec![1].vecslice(..1), [1])
    new_range_unbound_left2 => assert_eq!(vec![1, 2].vecslice(..2), [1, 2])
    new_range_unbound_left3 => assert_eq!(vec![1, 2, 3].vecslice(..3), [1, 2, 3])

    new_range_unbound_right0 => assert_eq!(vec![0; 0].vecslice(0..), [])
    new_range_unbound_right1 => assert_eq!(vec![1].vecslice(0..), [1])
    new_range_unbound_right2 => assert_eq!(vec![1, 2].vecslice(0..), [1, 2])
    new_range_unbound_right3 => assert_eq!(vec![1, 2, 3].vecslice(0..), [1, 2, 3])

    new_range0_excluded0 => assert_eq!(vec![0; 0].vecslice(0..0), [])
    new_range0_excluded1 => assert_eq!(vec![1].vecslice(0..0), [])
    new_range0_excluded2 => assert_eq!(vec![1, 2].vecslice(0..0), [])
    new_range0_excluded3 => assert_eq!(vec![1, 2, 3].vecslice(0..0), [])

    #[should_panic]
    new_range1_included0 => assert_eq!(vec![0; 0].vecslice(0..=0), [])
    new_range1_included1 => assert_eq!(vec![1].vecslice(0..=0), [1])
    new_range1_included2 => assert_eq!(vec![1, 2].vecslice(0..=0), [1])
    new_range1_included3 => assert_eq!(vec![1, 2, 3].vecslice(0..=0), [1])

    #[should_panic]
    new_range1_excluded0 => assert_eq!(vec![0; 0].vecslice(0..1), [])
    new_range1_excluded1 => assert_eq!(vec![1].vecslice(0..1), [1])
    new_range1_excluded2 => assert_eq!(vec![1, 2].vecslice(0..1), [1])
    new_range1_excluded3 => assert_eq!(vec![1, 2, 3].vecslice(0..1), [1])

    #[should_panic]
    new_range2_included0 => assert_eq!(vec![0; 0].vecslice(0..=1), [])
    #[should_panic]
    new_range2_included1 => assert_eq!(vec![1].vecslice(0..=1), [])
    new_range2_included2 => assert_eq!(vec![1, 2].vecslice(0..=1), [1, 2])
    new_range2_included3 => assert_eq!(vec![1, 2, 3].vecslice(0..=1), [1, 2])

    #[should_panic]
    new_range2_excluded0 => assert_eq!(vec![0; 0].vecslice(0..2), [])
    #[should_panic]
    new_range2_excluded1 => assert_eq!(vec![1].vecslice(0..2), [])
    new_range2_excluded2 => assert_eq!(vec![1, 2].vecslice(0..2), [1, 2])
    new_range2_excluded3 => assert_eq!(vec![1, 2, 3].vecslice(0..2), [1, 2])

    len0 => assert_eq!(vec![0; 0].vecslice(..).len(), 0)
    len1 => assert_eq!(vec![1].vecslice(..).len(), 1)
    len2 => assert_eq!(vec![1, 2].vecslice(..).len(), 2)
    len3 => assert_eq!(vec![1, 2, 3].vecslice(..).len(), 3)

    empty_true => assert_eq!(vec![0; 0].vecslice(..).is_empty(), true)
    empty_false => assert_eq!(vec![1].vecslice(..).is_empty(), false)

    push_back_tail_0 => {
        let mut v = Vec::new();
        let mut s = v.vecslice_at_tail();
        s.push_back(1);
        s.push_back(2);
        s.push_back(3);
        assert_eq!(s, [1, 2, 3]);
        assert_eq!(v, [1, 2, 3]);
    }
    push_back_tail_1 => {
        let mut v = vec![1];
        let mut s = v.vecslice_at_tail();
        s.push_back(2);
        s.push_back(3);
        assert_eq!(s, [2, 3]);
        assert_eq!(v, [1, 2, 3]);
    }
    push_back_tail_2 => {
        let mut v = vec![1, 2];
        let mut s = v.vecslice_at_tail();
        s.push_back(3);
        assert_eq!(s, [3]);
        assert_eq!(v, [1, 2, 3]);
    }

    push_back_front_0 => {
        let mut v = Vec::new();
        let mut s = v.vecslice(0..0);
        s.push_back(1);
        s.push_back(2);
        s.push_back(3);
        assert_eq!(s, [1, 2, 3]);
        assert_eq!(v, [1, 2, 3]);
    }
    push_back_front_1 => {
        let mut v = vec![1];
        let mut s = v.vecslice(0..0);
    }
}
