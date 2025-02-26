use opentelemetry::global::shutdown_tracer_provider;
use structured::init_test_subscriber;

#[tokio::test]
async fn failure() {
    init_test_subscriber();
    let order_numbers = vec![3, 4, 5];

    structured::get_total(&order_numbers).unwrap_err();

    // Ensure all spans are exported
    tokio::task::spawn_blocking(|| shutdown_tracer_provider())
        .await
        .unwrap();
}
