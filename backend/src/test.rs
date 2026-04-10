//
// use chattyApi::R;
//
// use super::*;
//
// #[tokio::test]
// async fn main_test()-> R<()> {
//     let a = register_user(&*SERVER, "happy".to_string(), "password12345".to_string()).await?;
//     dbg!(a);
//     anyhow::ensure!(register_user(&*SERVER, "happy".to_string(), "password12345".to_string()).await.is_err());
//     Ok(())
// }
// #[tokio::test]
// async fn start_test() {
//     dbg!(&*SERVER);
// }
