use std::io;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init() {
    // 创建 logs 目录
    std::fs::create_dir_all("logs").expect("无法创建 logs 目录");

    // 创建主日志文件 appender
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 将 guard 泄露以保持文件写入器活跃
    // 这是必要的，因为 guard 被 drop 时会关闭文件
    std::mem::forget(_guard);

    // 配置日志格式
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true);

    // 控制台日志层
    let console_layer = fmt::layer().with_writer(io::stdout).with_target(true);

    // 环境过滤器，默认为 info 级别
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 组合所有层
    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    tracing::info!("日志系统初始化完成");
    tracing::info!("日志文件将保存在 logs/app.log.<date> 中");
}
