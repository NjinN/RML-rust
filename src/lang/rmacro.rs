#[macro_export]
macro_rules! ptr_raw {
    ($token: expr) => {
        $token.0.read().unwrap()
    }
}


#[macro_export]
macro_rules! ptr_raw_mut {
    ($token: expr) => {
        $token.0.write().unwrap()
    }
}









