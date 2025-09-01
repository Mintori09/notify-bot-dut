use notify_bot_dut::entity::NoticeSent;
use notify_bot_dut::fetch::http_client;
use notify_bot_dut::fetch::{self};

#[tokio::test]
async fn test_fetch_class_notice() {
    let client = http_client();

    let result = fetch::fetch_class_notice(&client).await;

    match result {
        Ok(notices) => {
            println!("Got {} notices", notices.len());
            for n in notices.iter().take(5) {
                let notice = NoticeSent::try_from(n.clone()).unwrap();
                println!("Notice ready: {}", notice);
            }
            assert!(!notices.is_empty(), "Should fetch at least 1 notice");
        }
        Err(e) => panic!("fetch_class_notice failed: {e:?}"),
    }
}

#[tokio::test]
async fn test_fetch_training() {
    let client = http_client();

    let result = fetch::fetch_training(&client).await;

    match result {
        Ok(notices) => {
            println!("Got {} notices: ", notices.len());
            for n in notices.iter().take(5) {
                let notice = NoticeSent::try_from(n.clone()).unwrap();
                println!("Notice ready: {}", notice);
            }
        }
        Err(e) => panic!("fetch_training failed: {e:?}"),
    }
}

#[tokio::test]
async fn test_fetch_student_affairs() {
    let client = http_client();

    let result = fetch::fetch_student_affairs(&client).await;

    match result {
        Ok(notices) => {
            println!("Got {} notices: ", notices.len());
            for n in notices.iter().take(5) {
                let notice = NoticeSent::try_from(n.clone()).unwrap();
                println!("Notice ready: {}", notice);
            }
        }
        Err(e) => panic!("fetch_student_affairs failed: {e:?}"),
    }
}
#[tokio::test]
async fn test_fetch_tuition() {
    let client = http_client();

    let result = fetch::fetch_tuition(&client).await;

    match result {
        Ok(notices) => {
            println!("Got {} notices: ", notices.len());
            for n in notices.iter().take(5) {
                let notice = NoticeSent::try_from(n.clone()).unwrap();
                println!("Notice ready: {}", notice);
            }
        }
        Err(e) => panic!("fetch_tuition failed: {e:?}"),
    }
}
