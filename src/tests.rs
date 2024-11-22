#![cfg(test)]

use crate::option;

#[test]
fn test_option0() {
    assert_eq!(option!(), None::<()>);
}

#[test]
fn test_option1() {
    assert_eq!(option!(1), Some(1));
}

#[test]
fn test_option2() {
    let res = option!({
        fn fact(n: u32) -> u32 {
            if n < 1 {
                1
            } else {
                n * fact(n - 1)
            }
        }

        fact(10)
    });

    assert_eq!(res, Some(3628800));
}

macro_rules! test_macro {
    ($($input:literal)?) => {
        option!($($input)?)
    }
}

#[test]
fn test_option_with_macro_repetition1() {
    let res = test_macro!();
    assert_eq!(res, None::<()>)
}

#[test]
fn test_option_with_macro_repetition2() {
    let res = test_macro!("hello");
    assert_eq!(res, Some("hello"))
}

macro_rules! readme {
    ($($input1:literal)?, $($input2:literal)?, $($input3:literal)?, $($input4:literal)?, $($input5:literal)?) => {
        (option!($($input1)?), option!($($input2)?), option!($($input3)?), option!($($input4)?), option!($($input5)?))
    }
}

#[test]
#[rustfmt::skip]
fn test_readme() {
    assert_eq!(readme!(0,0,0,0, ), (Some(0),    Some(0),    Some(0),    Some(0),    None::<()>, ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0,0,0,0, ), (Some(0),    Some(0),    Some(0),    Some(0),    None::<()>, ));
    assert_eq!(readme!(0, , , , ), (Some(0),    None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!(0, , , , ), (Some(0),    None::<()>, None::<()>, None::<()>, None::<()>, ));
    
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!( ,0,0,0, ), (None::<()>, Some(0),    Some(0),    Some(0),    None::<()>, ));
    
    assert_eq!(readme!(0,0,0,0,0), (Some(0),    Some(0),    Some(0),    Some(0),    Some(0),    ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0,0, , ,0), (Some(0),    Some(0),    None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, ,0, ,0), (Some(0),    None::<()>, Some(0),    None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , ,0,0), (Some(0),    None::<()>, None::<()>, Some(0),    Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    
    assert_eq!(readme!( , , , , ), (None::<()>, None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!( , , , , ), (None::<()>, None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!( , , , , ), (None::<()>, None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!( , , , , ), (None::<()>, None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!( , , , , ), (None::<()>, None::<()>, None::<()>, None::<()>, None::<()>, ));
    
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0,0,0,0,0), (Some(0),    Some(0),    Some(0),    Some(0),    Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!( ,0,0,0, ), (None::<()>, Some(0),    Some(0),    Some(0),    None::<()>, ));
    
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    assert_eq!(readme!( , ,0, , ), (None::<()>, None::<()>, Some(0),    None::<()>, None::<()>, ));
    
    assert_eq!(readme!(0, , , , ), (Some(0),    None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!(0, , , , ), (Some(0),    None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!(0, , , , ), (Some(0),    None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!(0, , , , ), (Some(0),    None::<()>, None::<()>, None::<()>, None::<()>, ));
    assert_eq!(readme!(0,0,0,0,0), (Some(0),    Some(0),    Some(0),    Some(0),    Some(0),    ));
    
    assert_eq!(readme!( ,0,0,0, ), (None::<()>, Some(0),    Some(0),    Some(0),    None::<()>, ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!(0, , , ,0), (Some(0),    None::<()>, None::<()>, None::<()>, Some(0),    ));
    assert_eq!(readme!( ,0,0,0, ), (None::<()>, Some(0),    Some(0),    Some(0),    None::<()>, ));
}
