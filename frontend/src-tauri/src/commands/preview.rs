use std::io::Read as IoRead;
use tauri::State;

use crate::db::Database;
use crate::models::{ApiResponse, FilePreviewInfo};
use crate::services::auth_service::AuthService;
use crate::services::file_service::FileService;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewData {
    pub preview_type: String,
    pub content: String, // base64 for binary, raw text for text
    pub mime_type: String,
}

#[tauri::command]
pub fn get_file_preview_info(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<FilePreviewInfo>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    let file = FileService::get_file_by_id(&db, id).map_err(|e| e.to_string())?;

    let file_type = file.file_type.to_lowercase();
    let preview_type = get_preview_type(&file_type);
    let can_preview = preview_type != "unknown";

    let info = FilePreviewInfo {
        id: file.id,
        title: file.title,
        file_name: file.original_name,
        file_type: file.file_type,
        preview_type,
        file_size: file.file_size,
        can_preview,
        preview_url: String::new(), // 桌面应用不使用 URL
        download_url: String::new(),
    };

    Ok(ApiResponse::success(info))
}

#[tauri::command]
pub fn preview_file(
    db: State<Database>,
    auth: State<AuthService>,
    token: String,
    id: i64,
) -> Result<ApiResponse<PreviewData>, String> {
    auth.validate_token(&token).map_err(|e| e.to_string())?;

    let file = FileService::get_file_by_id(&db, id).map_err(|e| e.to_string())?;
    let file_type = file.file_type.to_lowercase();
    let preview_type = get_preview_type(&file_type);

    match preview_type.as_str() {
        "text" => {
            let content = FileService::get_file_content(&db, id).map_err(|e| e.to_string())?;
            Ok(ApiResponse::success(PreviewData {
                preview_type: "text".to_string(),
                content,
                mime_type: "text/plain".to_string(),
            }))
        }
        "image" | "pdf" => {
            let (b64, mime) = FileService::get_file_base64(&db, id).map_err(|e| e.to_string())?;
            Ok(ApiResponse::success(PreviewData {
                preview_type: preview_type.clone(),
                content: b64,
                mime_type: mime,
            }))
        }
        "office_word" => {
            let path = FileService::get_file_path(&db, id).map_err(|e| e.to_string())?;
            let html = convert_docx_to_html(&path).map_err(|e| e.to_string())?;
            Ok(ApiResponse::success(PreviewData {
                preview_type: "html".to_string(),
                content: html,
                mime_type: "text/html".to_string(),
            }))
        }
        "office_excel" => {
            let path = FileService::get_file_path(&db, id).map_err(|e| e.to_string())?;
            let html = convert_xlsx_to_html(&path).map_err(|e| e.to_string())?;
            Ok(ApiResponse::success(PreviewData {
                preview_type: "html".to_string(),
                content: html,
                mime_type: "text/html".to_string(),
            }))
        }
        "video" | "audio" => {
            let path = FileService::get_file_path(&db, id).map_err(|e| e.to_string())?;
            let mime = file.mime_type.unwrap_or_else(|| {
                if preview_type == "video" { "video/mp4".to_string() } else { "audio/mpeg".to_string() }
            });
            Ok(ApiResponse::success(PreviewData {
                preview_type: preview_type.clone(),
                content: path,
                mime_type: mime,
            }))
        }
        _ => {
            // 不支持的类型，返回文件路径让前端用系统应用打开
            let path = FileService::get_file_path(&db, id).map_err(|e| e.to_string())?;
            Ok(ApiResponse::success(PreviewData {
                preview_type: "external".to_string(),
                content: path,
                mime_type: file.mime_type.unwrap_or_default(),
            }))
        }
    }
}

fn get_preview_type(file_type: &str) -> String {
    match file_type {
        "pdf" => "pdf".to_string(),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" => "image".to_string(),
        "mp4" | "avi" | "mov" | "wmv" | "flv" | "mkv" | "webm" => "video".to_string(),
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => "audio".to_string(),
        "txt" | "md" | "json" | "xml" | "html" | "htm" | "css" | "js" | "ts" | "log" | "csv" => "text".to_string(),
        "doc" | "docx" => "office_word".to_string(),
        "xls" | "xlsx" => "office_excel".to_string(),
        "ppt" | "pptx" => "office".to_string(),
        _ => "unknown".to_string(),
    }
}

/// 将 xlsx 文件转换为 HTML 表格
fn convert_xlsx_to_html(file_path: &str) -> Result<String, String> {
    let file = std::fs::File::open(file_path)
        .map_err(|e| format!("无法打开文件: {}", e))?;

    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("无法解压 xlsx 文件: {}", e))?;

    // 1. 解析共享字符串表
    let shared_strings = parse_shared_strings(&mut archive)?;

    // 2. 解析 sheet1 工作表
    let table_html = parse_sheet(&mut archive, &shared_strings)?;

    let html = format!(
        r#"<div class="xlsx-wrapper">
<style>
.xlsx-wrapper {{
    font-family: 'Microsoft YaHei', Arial, sans-serif;
    font-size: 13px;
    overflow-x: auto;
}}
.xlsx-wrapper table {{
    border-collapse: collapse;
    width: auto;
    min-width: 100%;
}}
.xlsx-wrapper th, .xlsx-wrapper td {{
    border: 1px solid #d0d0d0;
    padding: 6px 10px;
    text-align: left;
    white-space: nowrap;
}}
.xlsx-wrapper tr:first-child td,
.xlsx-wrapper tr:first-child th {{
    background-color: #f0f0f0;
    font-weight: bold;
}}
.xlsx-wrapper tr:nth-child(even) {{
    background-color: #fafafa;
}}
.xlsx-wrapper tr:hover {{
    background-color: #e8f4fd;
}}
</style>
<table>{}</table></div>"#,
        table_html
    );

    Ok(html)
}

/// 解析 xlsx 的 sharedStrings.xml
fn parse_shared_strings(archive: &mut zip::ZipArchive<std::fs::File>) -> Result<Vec<String>, String> {
    let mut strings = Vec::new();

    let shared_strings_file = match archive.by_name("xl/sharedStrings.xml") {
        Ok(f) => f,
        Err(_) => return Ok(strings), // 没有共享字符串表，可能所有值都是数值
    };

    let mut xml_content = String::new();
    std::io::BufReader::new(shared_strings_file)
        .read_to_string(&mut xml_content)
        .map_err(|e| format!("读取 sharedStrings.xml 失败: {}", e))?;

    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(&xml_content);
    let mut buf = Vec::new();
    let mut in_si = false;
    let mut in_t = false;
    let mut current_string = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => {
                let local = e.local_name();
                let name = std::str::from_utf8(local.as_ref()).unwrap_or("");
                match name {
                    "si" => {
                        in_si = true;
                        current_string.clear();
                    }
                    "t" if in_si => {
                        in_t = true;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_t && in_si {
                    current_string.push_str(&e.unescape().unwrap_or_default());
                }
            }
            Ok(Event::End(ref e)) => {
                let local = e.local_name();
                let name = std::str::from_utf8(local.as_ref()).unwrap_or("");
                match name {
                    "si" => {
                        in_si = false;
                        strings.push(current_string.clone());
                    }
                    "t" => {
                        in_t = false;
                    }
                    _ => {}
                }
            }
            Err(e) => return Err(format!("解析 sharedStrings.xml 错误: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(strings)
}

/// 解析 xlsx 的 sheet1.xml，生成 HTML 表格行
fn parse_sheet(archive: &mut zip::ZipArchive<std::fs::File>, shared_strings: &[String]) -> Result<String, String> {
    let sheet_file = archive.by_name("xl/worksheets/sheet1.xml")
        .map_err(|e| format!("无法读取 sheet1.xml: {}", e))?;

    let mut xml_content = String::new();
    std::io::BufReader::new(sheet_file)
        .read_to_string(&mut xml_content)
        .map_err(|e| format!("读取 sheet1.xml 失败: {}", e))?;

    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(&xml_content);
    let mut buf = Vec::new();
    let mut html = String::new();

    let mut in_row = false;
    let mut in_cell = false;
    let mut in_value = false;
    let mut cell_type = String::new(); // "s" = shared string, "" or "n" = number
    let mut cell_value = String::new();
    let mut row_cells: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => {
                let local = e.local_name();
                let name = std::str::from_utf8(local.as_ref()).unwrap_or("");
                match name {
                    "row" => {
                        in_row = true;
                        row_cells.clear();
                    }
                    "c" if in_row => {
                        in_cell = true;
                        cell_type.clear();
                        cell_value.clear();
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "t" {
                                cell_type = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    "v" if in_cell => {
                        in_value = true;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_value {
                    cell_value.push_str(&e.unescape().unwrap_or_default());
                }
            }
            Ok(Event::End(ref e)) => {
                let local = e.local_name();
                let name = std::str::from_utf8(local.as_ref()).unwrap_or("");
                match name {
                    "v" => {
                        in_value = false;
                    }
                    "c" => {
                        in_cell = false;
                        let display_value = if cell_type == "s" {
                            // 共享字符串引用
                            if let Ok(idx) = cell_value.parse::<usize>() {
                                shared_strings.get(idx).cloned().unwrap_or_default()
                            } else {
                                cell_value.clone()
                            }
                        } else {
                            cell_value.clone()
                        };
                        row_cells.push(html_escape(&display_value));
                    }
                    "row" => {
                        in_row = false;
                        if !row_cells.is_empty() {
                            html.push_str("<tr>");
                            for cell in &row_cells {
                                html.push_str(&format!("<td>{}</td>", cell));
                            }
                            html.push_str("</tr>");
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => return Err(format!("解析 sheet1.xml 错误: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(html)
}

/// 将 docx 文件转换为 HTML
fn convert_docx_to_html(file_path: &str) -> Result<String, String> {
    let file = std::fs::File::open(file_path)
        .map_err(|e| format!("无法打开文件: {}", e))?;

    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("无法解压 docx 文件: {}", e))?;

    let mut xml_content = String::new();
    {
        let mut document_xml = archive.by_name("word/document.xml")
            .map_err(|e| format!("无法读取 document.xml: {}", e))?;
        document_xml.read_to_string(&mut xml_content)
            .map_err(|e| format!("读取 XML 内容失败: {}", e))?;
    }

    let html_body = parse_docx_xml(&xml_content)?;

    let html = format!(
        r#"<div class="docx-wrapper">
<style>
.docx-wrapper {{
    font-family: 'Microsoft YaHei', 'SimSun', Arial, sans-serif;
    font-size: 14px;
    line-height: 1.8;
    color: #333;
    word-wrap: break-word;
}}
.docx-wrapper p {{
    margin: 0 0 8px 0;
}}
.docx-wrapper h1 {{ font-size: 24px; font-weight: bold; margin: 16px 0 8px; }}
.docx-wrapper h2 {{ font-size: 20px; font-weight: bold; margin: 14px 0 8px; }}
.docx-wrapper h3 {{ font-size: 18px; font-weight: bold; margin: 12px 0 8px; }}
.docx-wrapper h4 {{ font-size: 16px; font-weight: bold; margin: 10px 0 8px; }}
.docx-wrapper h5 {{ font-size: 15px; font-weight: bold; margin: 8px 0 6px; }}
.docx-wrapper h6 {{ font-size: 14px; font-weight: bold; margin: 8px 0 6px; }}
.docx-wrapper table {{
    border-collapse: collapse;
    width: 100%;
    margin: 12px 0;
}}
.docx-wrapper td, .docx-wrapper th {{
    border: 1px solid #ccc;
    padding: 6px 10px;
    text-align: left;
    vertical-align: top;
}}
.docx-wrapper ul, .docx-wrapper ol {{
    margin: 8px 0;
    padding-left: 24px;
}}
</style>
{}</div>"#,
        html_body
    );

    Ok(html)
}

/// 解析 docx 的 XML 内容，生成 HTML
fn parse_docx_xml(xml: &str) -> Result<String, String> {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(xml);
    let mut html = String::new();
    let mut buf = Vec::new();

    // 状态跟踪
    let mut in_body = false;
    let mut in_paragraph = false;
    let mut in_run = false;
    let mut in_text = false;
    let mut in_table = false;
    let mut in_row = false;
    let mut in_cell = false;
    let mut in_hyperlink = false;

    // 段落属性
    let mut paragraph_style = String::new();
    let mut paragraph_align = String::new();
    let mut in_paragraph_props = false;

    // Run 属性
    let mut run_bold = false;
    let mut run_italic = false;
    let mut run_underline = false;
    let mut run_strike = false;
    let mut in_run_props = false;

    // 嵌套深度跟踪
    let mut p_pr_depth: u32 = 0;
    let mut r_pr_depth: u32 = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => {
                let local_name = e.local_name();
                let name = std::str::from_utf8(local_name.as_ref()).unwrap_or("");

                match name {
                    "body" => {
                        in_body = true;
                    }
                    "p" if in_body => {
                        in_paragraph = true;
                        paragraph_style.clear();
                        paragraph_align.clear();
                    }
                    "pPr" if in_paragraph => {
                        in_paragraph_props = true;
                        p_pr_depth = 1;
                    }
                    "jc" if in_paragraph_props => {
                        // 段落对齐
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                paragraph_align = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    "pStyle" if in_paragraph_props => {
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                paragraph_style = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    "r" if in_paragraph => {
                        in_run = true;
                        run_bold = false;
                        run_italic = false;
                        run_underline = false;
                        run_strike = false;
                    }
                    "rPr" if in_run => {
                        in_run_props = true;
                        r_pr_depth = 1;
                    }
                    "b" if in_run_props => {
                        // 检查是否有 val="0" 或 val="false"
                        let mut disabled = false;
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                let val = String::from_utf8_lossy(&attr.value).to_string();
                                if val == "0" || val == "false" {
                                    disabled = true;
                                }
                            }
                        }
                        if !disabled {
                            run_bold = true;
                        }
                    }
                    "i" if in_run_props => {
                        let mut disabled = false;
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                let val = String::from_utf8_lossy(&attr.value).to_string();
                                if val == "0" || val == "false" {
                                    disabled = true;
                                }
                            }
                        }
                        if !disabled {
                            run_italic = true;
                        }
                    }
                    "u" if in_run_props => {
                        run_underline = true;
                    }
                    "strike" if in_run_props => {
                        let mut disabled = false;
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                let val = String::from_utf8_lossy(&attr.value).to_string();
                                if val == "0" || val == "false" {
                                    disabled = true;
                                }
                            }
                        }
                        if !disabled {
                            run_strike = true;
                        }
                    }
                    "t" if in_run => {
                        in_text = true;
                    }
                    "tbl" if in_body => {
                        in_table = true;
                        html.push_str("<table>");
                    }
                    "tr" if in_table => {
                        in_row = true;
                        html.push_str("<tr>");
                    }
                    "tc" if in_row => {
                        in_cell = true;
                        html.push_str("<td>");
                    }
                    "hyperlink" if in_paragraph => {
                        in_hyperlink = true;
                    }
                    _ => {
                        // 跟踪嵌套深度
                        if in_paragraph_props {
                            p_pr_depth += 1;
                        }
                        if in_run_props {
                            r_pr_depth += 1;
                        }
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                let local_name = e.local_name();
                let name = std::str::from_utf8(local_name.as_ref()).unwrap_or("");

                match name {
                    "br" if in_paragraph => {
                        html.push_str("<br>");
                    }
                    "b" if in_run_props => {
                        let mut disabled = false;
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                let val = String::from_utf8_lossy(&attr.value).to_string();
                                if val == "0" || val == "false" {
                                    disabled = true;
                                }
                            }
                        }
                        if !disabled {
                            run_bold = true;
                        }
                    }
                    "i" if in_run_props => {
                        let mut disabled = false;
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                let val = String::from_utf8_lossy(&attr.value).to_string();
                                if val == "0" || val == "false" {
                                    disabled = true;
                                }
                            }
                        }
                        if !disabled {
                            run_italic = true;
                        }
                    }
                    "u" if in_run_props => {
                        run_underline = true;
                    }
                    "strike" if in_run_props => {
                        let mut disabled = false;
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                let val = String::from_utf8_lossy(&attr.value).to_string();
                                if val == "0" || val == "false" {
                                    disabled = true;
                                }
                            }
                        }
                        if !disabled {
                            run_strike = true;
                        }
                    }
                    "jc" if in_paragraph_props => {
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                paragraph_align = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    "pStyle" if in_paragraph_props => {
                        for attr in e.attributes().flatten() {
                            if std::str::from_utf8(attr.key.local_name().as_ref()).unwrap_or("") == "val" {
                                paragraph_style = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_text && in_run {
                    let text = e.unescape().unwrap_or_default().to_string();
                    let escaped = html_escape(&text);

                    // 构建样式
                    let mut styles = Vec::new();
                    if run_bold { styles.push("font-weight:bold"); }
                    if run_italic { styles.push("font-style:italic"); }
                    if run_underline { styles.push("text-decoration:underline"); }
                    if run_strike { styles.push("text-decoration:line-through"); }

                    if styles.is_empty() {
                        html.push_str(&escaped);
                    } else {
                        html.push_str(&format!(
                            "<span style=\"{}\">{}</span>",
                            styles.join(";"),
                            escaped
                        ));
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let local_name = e.local_name();
                let name = std::str::from_utf8(local_name.as_ref()).unwrap_or("");

                match name {
                    "body" => {
                        in_body = false;
                    }
                    "p" if in_paragraph => {
                        in_paragraph = false;
                        // 不在表格中时才输出段落标签
                        // 在表格中的段落内容已直接写入 <td>
                    }
                    "pPr" if in_paragraph_props => {
                        in_paragraph_props = false;
                        p_pr_depth = 0;

                        // 段落属性读取完毕，输出段落开始标签
                        let tag = get_heading_tag(&paragraph_style);
                        let align_style = get_align_style(&paragraph_align);

                        if !in_cell {
                            if align_style.is_empty() {
                                html.push_str(&format!("<{}>", tag));
                            } else {
                                html.push_str(&format!("<{} style=\"{}\">", tag, align_style));
                            }
                        }
                    }
                    "r" if in_run => {
                        in_run = false;
                        run_bold = false;
                        run_italic = false;
                        run_underline = false;
                        run_strike = false;
                    }
                    "rPr" if in_run_props => {
                        in_run_props = false;
                        r_pr_depth = 0;
                    }
                    "t" if in_text => {
                        in_text = false;
                    }
                    "tbl" if in_table => {
                        in_table = false;
                        html.push_str("</table>");
                    }
                    "tr" if in_row => {
                        in_row = false;
                        html.push_str("</tr>");
                    }
                    "tc" if in_cell => {
                        in_cell = false;
                        html.push_str("</td>");
                    }
                    "hyperlink" if in_hyperlink => {
                        in_hyperlink = false;
                    }
                    _ => {
                        if in_paragraph_props && p_pr_depth > 0 {
                            p_pr_depth -= 1;
                        }
                        if in_run_props && r_pr_depth > 0 {
                            r_pr_depth -= 1;
                        }
                    }
                }

                // 段落结束（没有 pPr 的情况也要处理）
                if name == "p" && !in_cell {
                    let tag = get_heading_tag(&paragraph_style);
                    // 如果没有 pPr，段落标签还未输出，这里补上
                    if !html.ends_with(&format!("</{}>", tag)) && !html.ends_with(&format!("<{}>", tag)) {
                        // 检查是否已经输出了开始标签
                        let open_tag_prefix = format!("<{}", tag);
                        let has_open_tag = html.contains(&open_tag_prefix);
                        if !has_open_tag {
                            html.push_str(&format!("<{}>", tag));
                        }
                    }
                    html.push_str(&format!("</{}>", tag));
                }
            }
            Err(e) => {
                return Err(format!("XML 解析错误: {}", e));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(html)
}

/// 根据段落样式名返回对应的 HTML 标签
fn get_heading_tag(style: &str) -> &str {
    let style_lower = style.to_lowercase();
    if style_lower.contains("heading1") || style_lower == "1" || style_lower.contains("标题 1") || style_lower.contains("标题1") {
        "h1"
    } else if style_lower.contains("heading2") || style_lower == "2" || style_lower.contains("标题 2") || style_lower.contains("标题2") {
        "h2"
    } else if style_lower.contains("heading3") || style_lower == "3" || style_lower.contains("标题 3") || style_lower.contains("标题3") {
        "h3"
    } else if style_lower.contains("heading4") || style_lower == "4" || style_lower.contains("标题 4") || style_lower.contains("标题4") {
        "h4"
    } else if style_lower.contains("heading5") || style_lower == "5" || style_lower.contains("标题 5") || style_lower.contains("标题5") {
        "h5"
    } else if style_lower.contains("heading6") || style_lower == "6" || style_lower.contains("标题 6") || style_lower.contains("标题6") {
        "h6"
    } else {
        "p"
    }
}

/// 根据对齐方式返回 CSS 样式
fn get_align_style(align: &str) -> String {
    match align {
        "center" => "text-align:center".to_string(),
        "right" | "end" => "text-align:right".to_string(),
        "both" | "distribute" => "text-align:justify".to_string(),
        _ => String::new(),
    }
}

/// HTML 特殊字符转义
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
