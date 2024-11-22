# Option macro
Sometimes you just need one little thing...

Intended to be used as a building block for other macros, specifically with macro repetitions.
All it does is turning "nothing" into `Option::None`, and "something" (expr) into `Option::Some(expr)`

## Example
```rust
macro_rules! readme {
    ($($input1:literal)?, $($input2:literal)?, $($input3:literal)?, $($input4:literal)?) => {
        (option!($($input1)?), option!($($input2)?), option!($($input3)?), option!($($input4)?))
    }
}

readme!(1, 2,  , 4) // (Some(1), Some(2), None, Some(4))
```