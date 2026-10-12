//! 이미지 정규화 유틸리티
//!
//! CLI(main.rs)와 WASM 내보내기 경로가 공통으로 사용한다.

/// BMP 임베디드 이미지는 마크다운/웹 뷰어에서 렌더되지 않고 용량도 크므로
/// (HWP 임베딩은 흔히 수 MB BMP) PNG로 변환한다.
///
/// MIME 감지가 `application/octet-stream`으로 떨어지는 경우가 있어
/// 데이터 매직바이트("BM")로도 BMP를 판정한다. BMP가 아니면 입력 그대로 반환.
pub fn normalize_export_image(mime: String, data: Vec<u8>) -> (String, Vec<u8>) {
    let is_bmp = mime == "image/bmp" || data.starts_with(&[0x42, 0x4D]);
    if is_bmp {
        use image::{load_from_memory_with_format, ImageFormat};
        use std::io::Cursor;
        if let Ok(img) = load_from_memory_with_format(&data, ImageFormat::Bmp) {
            let mut out = Vec::new();
            if img
                .write_to(&mut Cursor::new(&mut out), ImageFormat::Png)
                .is_ok()
            {
                return ("image/png".to_string(), out);
            }
        }
        // 변환 실패 시 원본 유지 (최소한 데이터는 보존)
    }
    (mime, data)
}
