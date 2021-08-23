use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

pub static CACHE_I64: Lazy<Mutex<HashMap<String, i64>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});


#[cfg(test)]
mod test {
    use crate::util::local_cache::CACHE_I64;

    #[test]
    fn test() {
        if let Ok(mut cc) = CACHE_I64.lock() {

            for i in 0..10000 {
                cc.insert(format!("key{}", i), i);
            }
        }

        if let Ok(mut cc) = CACHE_I64.lock() {

            for i in 0..10000 {
                if let Some(v) = cc.get(&format!("key{}", i)) {
                    println!("{:?}", v);
                }
            }
        }


    }
}

