use futures::future::join_all;
use futures::future::FutureExt;

async fn bar(i: u32){
    std::thread::sleep(std::time::Duration::from_secs(i.into()));
    // async_std::task::sleep(std::time::Duration::from_secs(i.into())).await;
    println!("{i}");
}

async fn foo(i: u32) -> u32 {
    // std::thread::sleep(std::time::Duration::from_secs(i.into()));
    // async_std::task::sleep(std::time::Duration::from_secs(i.into())).await;
    // println!("{i}");
    async_std::task::spawn(bar(i)).await;
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn it_works() {
        let futures = vec![foo(3), foo(2), foo(1)];
        assert_eq!(join_all(futures).await, [3, 2, 1]);
        // futures::join!(foo(3), foo(2), foo(1));
        let f1 = foo(1);
        let shared1 = f1.shared();
        let shared2 = shared1.clone();
        assert_eq!(1, shared1.await);
        assert_eq!(1, shared2.await);
        
        // assert_eq!(join_all(futures).await, [1, 2, 3]);
    }
}
