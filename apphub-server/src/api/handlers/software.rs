use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::{software_service, category_service};

/// 软件列表项
#[derive(Debug, Serialize)]
pub struct SoftwareItem {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub file_name: String,
    pub file_size: i64,
    pub file_hash: Option<String>,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub platform: String,
    pub status: i16,
    pub download_count: i64,
    pub created_at: String,
}

/// 更新软件请求
#[derive(Debug, Deserialize)]
pub struct UpdateSoftwareRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<i16>,
}

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct SoftwareQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub category_id: Option<i64>,
    pub status: Option<i16>,
    pub keyword: Option<String>,
    pub platform: Option<String>,
}

/// 获取软件列表
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<SoftwareQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (softwares, total) = software_service::list_software_with_filters(
        &state.db,
        query.category_id,
        query.status,
        query.keyword.as_deref(),
        query.platform.as_deref(),
        page,
        page_size,
    )
    .await?;

    let mut items = Vec::new();
    for s in softwares {
        let category_name = category_service::get_category(&state.db, s.category_id)
            .await
            .ok()
            .flatten()
            .map(|c| c.name);

        items.push(SoftwareItem {
            id: s.id,
            name: s.name,
            version: s.version,
            description: s.description,
            file_name: s.file_name,
            file_size: s.file_size,
            file_hash: s.file_hash,
            category_id: s.category_id,
            category_name,
            platform: s.platform,
            status: s.status,
            download_count: s.download_count,
            created_at: s.created_at.to_rfc3339(),
        });
    }

    Ok(Json(json!({
        "code": 200,
        "message": "success",
        "data": {
            "list": items,
            "total": total,
            "page": page,
            "page_size": page_size
        }
    })))
}

/// 创建软件（multipart 文件上传）
pub async fn create(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut name: Option<String> = None;
    let mut version: Option<String> = None;
    let mut description: Option<String> = None;
    let mut category_id: Option<i64> = None;
    let mut platform: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::ValidationError(format!("解析multipart失败: {}", e)))?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "name" => {
                name = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| AppError::ValidationError(e.to_string()))?,
                );
            }
            "version" => {
                version = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| AppError::ValidationError(e.to_string()))?,
                );
            }
            "description" => {
                description = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| AppError::ValidationError(e.to_string()))?,
                );
            }
            "category_id" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| AppError::ValidationError(e.to_string()))?;
                category_id = Some(
                    text.parse()
                        .map_err(|_| AppError::ValidationError("category_id 必须是数字".to_string()))?,
                );
            }
            "platform" => {
                platform = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| AppError::ValidationError(e.to_string()))?,
                );
            }
            "file" => {
                file_name = field.file_name().map(|s| s.to_string());
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::ValidationError(format!("读取文件失败: {}", e)))?;
                file_data = Some(data.to_vec());
            }
            _ => {}
        }
    }

    let name = name.ok_or_else(|| AppError::ValidationError("缺少name字段".to_string()))?;
    let version = version.ok_or_else(|| AppError::ValidationError("缺少version字段".to_string()))?;
    let category_id = category_id.ok_or_else(|| AppError::ValidationError("缺少category_id字段".to_string()))?;
    let file_data = file_data.ok_or_else(|| AppError::ValidationError("缺少文件".to_string()))?;
    let file_name = file_name.unwrap_or_else(|| "unknown".to_string());

    // 校验平台字段
    let platform = platform.unwrap_or_else(|| "mac,linux,windows".to_string());
    let valid_platforms = ["mac", "linux", "windows"];
    for p in platform.split(',') {
        let p = p.trim();
        if !valid_platforms.contains(&p) {
            return Err(AppError::ValidationError(format!("无效的平台: {}", p)));
        }
    }

    // 检查文件大小
    if file_data.len() > state.upload_max_size {
        return Err(AppError::ValidationError("文件大小超过限制".to_string()));
    }

    let software_id = software_service::create_software(
        &state.db,
        &name,
        &version,
        description.as_deref(),
        category_id,
        &platform,
        &file_data,
        &file_name,
        &state.upload_dir,
        0, // TODO: 从认证中获取用户ID
    )
    .await?;

    Ok(Json(json!({
        "code": 200,
        "message": "创建成功",
        "data": { "id": software_id }
    })))
}

/// 获取软件详情
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let software = software_service::get_software(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("软件不存在".to_string()))?;

    let category_name = category_service::get_category(&state.db, software.category_id)
        .await
        .ok()
        .flatten()
        .map(|c| c.name);

    let item = SoftwareItem {
        id: software.id,
        name: software.name,
        version: software.version,
        description: software.description,
        file_name: software.file_name,
        file_size: software.file_size,
        file_hash: software.file_hash,
        category_id: software.category_id,
        category_name,
        platform: software.platform,
        status: software.status,
        download_count: software.download_count,
        created_at: software.created_at.to_rfc3339(),
    };

    Ok(Json(json!({
        "code": 200,
        "message": "success",
        "data": item
    })))
}

/// 更新软件
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateSoftwareRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    software_service::update_software(
        &state.db,
        id,
        req.name.as_deref(),
        req.description.as_deref(),
        req.status,
    )
    .await?;

    Ok(Json(json!({
        "code": 200,
        "message": "更新成功",
        "data": null
    })))
}

/// 删除软件
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    software_service::delete_software(&state.db, id, &state.upload_dir).await?;
    Ok(Json(json!({
        "code": 200,
        "message": "删除成功",
        "data": null
    })))
}

/// 下载软件（流式文件下载）
pub async fn download(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    let software = software_service::get_software(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("软件不存在".to_string()))?;

    if software.status != 1 {
        return Err(AppError::Forbidden("软件已下架".to_string()));
    }

    let file_path = crate::services::file_service::get_full_path(&state.upload_dir, &software.file_path);

    if !file_path.exists() {
        return Err(AppError::NotFound("文件不存在".to_string()));
    }

    // 增加下载次数
    let _ = software_service::increment_download_count(&state.db, id).await;

    // 读取文件内容
    let file_data = tokio::fs::read(&file_path)
        .await
        .map_err(|e| AppError::InternalError(format!("读取文件失败: {}", e)))?;

    // 构建响应
    // 对文件名做 RFC 5987 UTF-8 编码，避免中文/特殊字符导致浏览器丢弃下载
    let encoded_filename = utf8_percent_encode(&software.file_name, NON_ALPHANUMERIC).to_string();
    let content_disposition = format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        software.file_name, encoded_filename
    );

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_DISPOSITION, content_disposition)
        .header(header::CONTENT_LENGTH, software.file_size)
        .body(Body::from(file_data))
        .map_err(|e| AppError::InternalError(format!("构建响应失败: {}", e)))?;

    Ok(response)
}
