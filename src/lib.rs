use async_process::Command;
#[allow(dead_code)]
async fn thread_sleep(i: u32){
    std::thread::sleep(std::time::Duration::from_secs(i.into()));
    // async_std::task::sleep(std::time::Duration::from_secs(i.into())).await;
    // println!("{i}");
}

#[allow(dead_code)]
async fn foo(i: u32) -> u32 {
    // std::thread::sleep(std::time::Duration::from_secs(i.into()));
    // async_std::task::sleep(std::time::Duration::from_secs(i.into())).await;
    // println!("{i}");
    async_std::task::spawn(thread_sleep(i)).await;
    i
}

#[allow(dead_code)]
async fn command_sleep(delay: u32)->Result<(), async_std::io::Error>{
    Command::new("sleep").arg(delay.to_string()).status().await?;
    Result::Ok(())
}

#[allow(dead_code)]
async fn run_command(){
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;
    use futures::future::FutureExt;
    use std::time::Instant;

    #[async_std::test]
    async fn it_works() {
        let lst_futures = vec![foo(3), foo(2), foo(1)];
        assert_eq!(future::join_all(lst_futures).await, [3, 2, 1]);
        // futures::join!(foo(3), foo(2), foo(1));
        let f1 = foo(1);
        let shared1 = f1.shared();
        let shared2 = shared1.clone();
        assert_eq!(1, shared1.await);
        assert_eq!(1, shared2.await);
        
        // assert_eq!(join_all(futures).await, [1, 2, 3]);
    }

    #[async_std::test]
    async fn concurrent_run() {
        let start_time = Instant::now();
        let f1 = future::join(command_sleep(3), command_sleep(3));
        let f2 = future::join(f1, command_sleep(3));
        let ((res1,res2), res3) = f2.await;
        res1.map_err(|x| println!("Error: {x}")).ok();
        res2.map_err(|x| println!("Error: {x}")).ok();
        res3.map_err(|x| println!("Error: {x}")).ok();
        // match res1{
        //     Result::Err(x) => println!("Error: {x}"),
        //     _ =>()
        // }
        // match res2{
        //     Result::Err(x) => println!("Error: {x}"),
        //     _ =>()
        // }
        let run_time = start_time.elapsed().as_secs();
        assert!(run_time < 4);
    }

    #[async_std::test]
    async fn foreach_run() {
        let start_time = Instant::now();
        let mut lst_futures = Vec::new();
        for _ in 1..5{
            lst_futures.push(command_sleep(3));
        }
        future::join_all(lst_futures).await; // no error check
        let run_time = start_time.elapsed().as_secs();
        assert!(run_time < 4);
    }
}
