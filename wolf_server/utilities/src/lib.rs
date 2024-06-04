#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_export]
macro_rules! ret_opt {
    ($e: expr) => {
        match $e {
            Some(e) => e,
            None => return,
        }
    };
}
