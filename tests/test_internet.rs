use notify_bot_dut::utils::has_internet;

#[tokio::test]
async fn test_internet() {
    let connected = has_internet().await;
    assert!(connected, "Expected internet, but none is available");
}
