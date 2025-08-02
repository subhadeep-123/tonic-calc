use crate::generated::service::calculator_server::CalculatorServer;
use crate::server::handler::CalculatorServiceImpl;
use tonic_health::server::HealthReporter;

pub async fn monitor_health(mut health_reporter: HealthReporter) {
    tokio::spawn(async move {
        loop {
            // Example condition: replace with your real checks
            let is_healthy = true;

            if is_healthy {
                health_reporter
                    .set_serving::<CalculatorServer<CalculatorServiceImpl>>()
                    .await;
            } else {
                health_reporter
                    .set_not_serving::<CalculatorServer<CalculatorServiceImpl>>()
                    .await;
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });
}
