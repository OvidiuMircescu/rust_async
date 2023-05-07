use async_process::Command;
use futures::future::{self, Shared};
use futures::future::FutureExt;

pub async fn thread_sleep(i: u32){
    std::thread::sleep(std::time::Duration::from_secs(i.into()));
    // async_std::task::sleep(std::time::Duration::from_secs(i.into())).await;
    // println!("{i}");
}

pub async fn foo(i: u32) -> u32 {
    // std::thread::sleep(std::time::Duration::from_secs(i.into()));
    // async_std::task::sleep(std::time::Duration::from_secs(i.into())).await;
    // println!("{i}");
    async_std::task::spawn(thread_sleep(i)).await;
    i
}

pub async fn command_sleep(delay: u32)->Result<u32, async_std::io::Error>{
    Command::new("sleep").arg(delay.to_string()).status().await?;
    Result::Ok(delay)
}

pub async fn s_command_sleep(delay: u32)->u32{
    Command::new("sleep").arg(delay.to_string()).status().await;
    delay
}

// TODO: deal with errors. async_std::io::Error not clonable.
// pub async fn run_command<Fut>(args:Fut) -> Result<u32, async_std::io::Error>
pub async fn run_command<Fut>(args:Fut) -> u32
where
//   Fut: future::Future<Output = Result<u32, async_std::io::Error>>
  Fut: future::Future<Output = u32>
{
    let v = args.await;
    let ret = s_command_sleep(v).await;
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[async_std::test]
    async fn mini_graph() {
        let start_time = Instant::now();
        let r1 = s_command_sleep(1).shared();
        let r2 = run_command(r1.clone()).shared();
        let r3 = run_command(r1.clone()).shared();
        let r4 = run_command(r2.clone()).shared();
        future::join(future::join(future::join(r1, r2), r3), r4).await;
        let run_time = start_time.elapsed().as_secs();
        // println!("run_time:{run_time}s");
        assert!(run_time == 3);
    }
}
