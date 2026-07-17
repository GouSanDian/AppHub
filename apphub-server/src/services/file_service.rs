//! 文件服务

use crate::error::AppError;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

/// 文件信息
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub file_hash: String,
}

/// 保存上传文件
pub async fn save_file(
    file_data: &[u8],
    filename: &str,
    upload_dir: &Path,
) -> Result<FileInfo, AppError> {
    // 生成唯一文件名
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let unique_name = format!("{}_{}.{}", Uuid::new_v4(), timestamp(), ext);

    // 创建上传目录
    let file_path = upload_dir.join(&unique_name);
    fs::create_dir_all(upload_dir)
        .await
        .map_err(|e| AppError::InternalError(format!("创建上传目录失败: {}", e)))?;

    // 写入文件
    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| AppError::InternalError(format!("创建文件失败: {}", e)))?;

    file.write_all(file_data)
        .await
        .map_err(|e| AppError::InternalError(format!("写入文件失败: {}", e)))?;

    file.flush()
        .await
        .map_err(|e| AppError::InternalError(format!("刷新文件失败: {}", e)))?;

    // 计算文件哈希
    let file_hash = calculate_hash(file_data);

    let file_info = FileInfo {
        file_name: filename.to_string(),
        file_path: unique_name,
        file_size: file_data.len() as i64,
        file_hash,
    };

    tracing::info!("文件保存成功: {} -> {}", filename, file_info.file_path);
    Ok(file_info)
}

/// 计算文件哈希（SHA256）
pub fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// 计算文件MD5
pub fn calculate_md5(file_path: &Path) -> Result<String, AppError> {
    // 同步读取文件计算MD5（用于小文件）
    let data = std::fs::read(file_path)
        .map_err(|e| AppError::InternalError(format!("读取文件失败: {}", e)))?;

    let mut hasher = md5::Context::new();
    hasher.consume(&data);
    Ok(format!("{:x}", hasher.compute()))
}

/// 删除文件
pub async fn delete_file(file_path: &Path) -> Result<(), AppError> {
    if file_path.exists() {
        fs::remove_file(file_path)
            .await
            .map_err(|e| AppError::InternalError(format!("删除文件失败: {}", e)))?;
        tracing::info!("文件删除成功: {:?}", file_path);
    }
    Ok(())
}

/// 获取文件完整路径
pub fn get_full_path(upload_dir: &Path, file_path: &str) -> PathBuf {
    upload_dir.join(file_path)
}

/// 检查文件是否存在
pub async fn file_exists(file_path: &Path) -> bool {
    file_path.exists()
}

/// 获取文件大小
pub async fn get_file_size(file_path: &Path) -> Result<i64, AppError> {
    let metadata = fs::metadata(file_path)
        .await
        .map_err(|e| AppError::InternalError(format!("获取文件元数据失败: {}", e)))?;
    Ok(metadata.len() as i64)
}

/// 生成时间戳
fn timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// 分片上传初始化
pub async fn init_multipart_upload(
    upload_dir: &Path,
    filename: &str,
    total_chunks: i32,
) -> Result<String, AppError> {
    let upload_id = Uuid::new_v4().to_string();
    let temp_dir = upload_dir.join("temp").join(&upload_id);

    fs::create_dir_all(&temp_dir)
        .await
        .map_err(|e| AppError::InternalError(format!("创建临时目录失败: {}", e)))?;

    // 保存上传元信息
    let meta = serde_json::json!({
        "upload_id": upload_id,
        "filename": filename,
        "total_chunks": total_chunks,
        "uploaded_chunks": [],
        "created_at": timestamp()
    });

    let meta_path = temp_dir.join("meta.json");
    let meta_data = serde_json::to_string_pretty(&meta)
        .map_err(|e| AppError::InternalError(format!("序列化元数据失败: {}", e)))?;

    fs::write(&meta_path, meta_data)
        .await
        .map_err(|e| AppError::InternalError(format!("保存元数据失败: {}", e)))?;

    tracing::info!("分片上传初始化: {} -> {}", filename, upload_id);
    Ok(upload_id)
}

/// 上传分片
pub async fn upload_chunk(
    upload_dir: &Path,
    upload_id: &str,
    chunk_index: i32,
    chunk_data: &[u8],
) -> Result<(), AppError> {
    let temp_dir = upload_dir.join("temp").join(upload_id);
    let chunk_path = temp_dir.join(format!("chunk_{}", chunk_index));

    let mut file = fs::File::create(&chunk_path)
        .await
        .map_err(|e| AppError::InternalError(format!("创建分片文件失败: {}", e)))?;

    file.write_all(chunk_data)
        .await
        .map_err(|e| AppError::InternalError(format!("写入分片失败: {}", e)))?;

    file.flush()
        .await
        .map_err(|e| AppError::InternalError(format!("刷新分片失败: {}", e)))?;

    // 更新元数据
    let meta_path = temp_dir.join("meta.json");
    if let Ok(meta_data) = fs::read_to_string(&meta_path).await {
        if let Ok(mut meta) = serde_json::from_str::<serde_json::Value>(&meta_data) {
            if let Some(chunks) = meta["uploaded_chunks"].as_array_mut() {
                chunks.push(serde_json::json!(chunk_index));
                let updated_meta = serde_json::to_string_pretty(&meta)
                    .map_err(|e| AppError::InternalError(format!("序列化元数据失败: {}", e)))?;
                let _ = fs::write(&meta_path, updated_meta).await;
            }
        }
    }

    tracing::info!("分片上传成功: {} chunk {}", upload_id, chunk_index);
    Ok(())
}

/// 完成分片上传（合并文件）
pub async fn complete_multipart_upload(
    upload_dir: &Path,
    upload_id: &str,
) -> Result<FileInfo, AppError> {
    let temp_dir = upload_dir.join("temp").join(upload_id);
    let meta_path = temp_dir.join("meta.json");

    // 读取元数据
    let meta_data = fs::read_to_string(&meta_path)
        .await
        .map_err(|e| AppError::InternalError(format!("读取元数据失败: {}", e)))?;

    let meta: serde_json::Value = serde_json::from_str(&meta_data)
        .map_err(|e| AppError::InternalError(format!("解析元数据失败: {}", e)))?;

    let filename = meta["filename"]
        .as_str()
        .ok_or_else(|| AppError::InternalError("元数据缺少filename".to_string()))?;

    let total_chunks = meta["total_chunks"]
        .as_i64()
        .ok_or_else(|| AppError::InternalError("元数据缺少total_chunks".to_string()))? as i32;

    let uploaded_chunks = meta["uploaded_chunks"]
        .as_array()
        .ok_or_else(|| AppError::InternalError("元数据缺少uploaded_chunks".to_string()))?;

    // 检查所有分片是否已上传
    if uploaded_chunks.len() != total_chunks as usize {
        return Err(AppError::InternalError(format!(
            "分片不完整: 已上传 {}/{}",
            uploaded_chunks.len(),
            total_chunks
        )));
    }

    // 生成最终文件名
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let unique_name = format!("{}_{}.{}", Uuid::new_v4(), timestamp(), ext);
    let final_path = upload_dir.join(&unique_name);

    // 合并分片
    let mut final_file = fs::File::create(&final_path)
        .await
        .map_err(|e| AppError::InternalError(format!("创建最终文件失败: {}", e)))?;

    let mut total_size: i64 = 0;
    let mut hasher = Sha256::new();

    for i in 0..total_chunks {
        let chunk_path = temp_dir.join(format!("chunk_{}", i));
        let mut chunk_file = fs::File::open(&chunk_path)
            .await
            .map_err(|e| AppError::InternalError(format!("打开分片失败: {}", e)))?;

        let mut chunk_data = Vec::new();
        chunk_file
            .read_to_end(&mut chunk_data)
            .await
            .map_err(|e| AppError::InternalError(format!("读取分片失败: {}", e)))?;

        total_size += chunk_data.len() as i64;
        hasher.update(&chunk_data);

        final_file
            .write_all(&chunk_data)
            .await
            .map_err(|e| AppError::InternalError(format!("写入最终文件失败: {}", e)))?;
    }

    final_file
        .flush()
        .await
        .map_err(|e| AppError::InternalError(format!("刷新最终文件失败: {}", e)))?;

    // 删除临时目录
    fs::remove_dir_all(&temp_dir)
        .await
        .map_err(|e| AppError::InternalError(format!("删除临时目录失败: {}", e)))?;

    let file_hash = format!("{:x}", hasher.finalize());

    let file_info = FileInfo {
        file_name: filename.to_string(),
        file_path: unique_name,
        file_size: total_size,
        file_hash,
    };

    tracing::info!(
        "分片上传完成: {} -> {} ({} bytes)",
        filename,
        file_info.file_path,
        total_size
    );
    Ok(file_info)
}
