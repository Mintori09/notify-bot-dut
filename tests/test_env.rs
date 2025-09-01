use notify_bot_dut::database::Config;

#[test]
fn test_env() {
    let config = Config::init();
    println!("{:?}", config);
}
