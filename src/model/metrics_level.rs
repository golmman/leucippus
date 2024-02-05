use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum MetricsLevel {
    Silent,
    Minimal,
    Reduced,
    Full,
}
