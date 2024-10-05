pub mod first;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn it_works() {
        println!("{}", size_of::<first::List>());
    }
}
