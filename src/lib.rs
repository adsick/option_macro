#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! option {
    ($value:expr) => {
        Some($value)
    };
    () => {
        None
    };
}
