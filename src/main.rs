use clap::Parser;

/// Phlodia is a Kubernetes and Cloud Native Kafka distribution.
#[derive(Parser)]
pub struct Cli {

}

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();

    phlodia::run().await;
}
