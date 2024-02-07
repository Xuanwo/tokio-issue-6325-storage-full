use rand::prelude::*;
use tokio::io::AsyncWriteExt;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut f = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("/tmp/test_dir/test")
        .await
        .unwrap();


    let size = 512 * 1024 + 1;
    let mut bs = vec![0; size];
    thread_rng().fill_bytes(&mut bs);

    let n = f.write(&bs).await?;
    dbg!(&n);
    assert_eq!(n, size, "tokio file always write data into buffer first");

    let res = f.flush().await;
    dbg!(&res);
    assert!(res.is_err(), "the first flush will return the correct error");

    // After some operations, we retry the file flush.
    let res = f.flush().await;
    dbg!(&res);
    assert!(res.is_ok(), "but the second flush return ok");

    let res = f.sync_all().await;
    dbg!(&res);
    assert!(res.is_ok(), "sync_all will return ok too");

    let res = f.shutdown().await;
    dbg!(&res);
    assert!(res.is_ok(), "shutdown will return ok too");

    assert!(false, "data lost while reaching this line.");
    Ok(())
}
