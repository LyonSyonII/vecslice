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
    new0 => assert_eq!(vec![0; 0].vecslice(..), [])
    new1 => assert_eq!(vec![1].vecslice(..), [1])
    new2 => assert_eq!(vec![1, 2].vecslice(..), [1, 2])
    new3 => assert_eq!(vec![1, 2, 3].vecslice(..), [1, 2, 3])
    
    new_tail0 => assert_eq!(vec![0; 0].vecslice_at_tail(), [])
    new_tail1 => assert_eq!(vec![1].vecslice_at_tail(), [])
    new_tail2 => assert_eq!(vec![1, 2].vecslice_at_tail(), [])
    new_tail3 => assert_eq!(vec![1, 2, 3].vecslice_at_tail(), [])

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


}