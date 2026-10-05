use starter_api::api::openapi::ApiDoc;
use utoipa::OpenApi;

fn main() -> Result<(), serde_json::Error> {
    // Value 默认使用排序映射，使输出无时间戳且可确定性比对。
    let document = serde_json::to_value(ApiDoc::openapi())?;
    println!("{}", serde_json::to_string_pretty(&document)?);
    Ok(())
}
