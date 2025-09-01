use notify_bot_dut::fetch::fetch_all_notices;

#[tokio::test]
async fn test_fetch_real_data() {
    let client = reqwest::Client::new();
    let notices = fetch_all_notices(&client).await.unwrap();

    assert!(!notices.is_empty(), "Should fetch at least one notice");
    println!("Got {} notices", notices.len());
    for n in notices {
        println!("- {}", n);
    }
}
