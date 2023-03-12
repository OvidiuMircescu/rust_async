use futures::future::join_all;
use futures::future::FutureExt;

async fn foo(i: u32) -> u32 {
    println!("{i}");
     i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn it_works() {
        let futures = vec![foo(1), foo(2), foo(3)];
        assert_eq!(join_all(futures).await, [1, 2, 3]);
        let f1 = foo(1);
        let shared1 = f1.shared();
        let shared2 = shared1.clone();
        assert_eq!(1, shared1.await);
        assert_eq!(1, shared2.await);
        
        // assert_eq!(join_all(futures).await, [1, 2, 3]);
    }
}
