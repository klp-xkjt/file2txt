use thiserror::Error;

#[derive(Error, Debug)]
pub enum File2txtError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("遍历目录出错: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("JSON 序列化出错: {0}")]
    Json(#[from] serde_json::Error),

    #[error("路径错误：{0}")]
    InvalidPath(String),

    #[error("未知的输出格式: '{0}'，支持: normal, meta, markdown, json")]
    UnknownFormat(String),
}
