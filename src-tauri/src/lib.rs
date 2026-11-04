use base64::{engine::general_purpose, Engine as _};
use image::GenericImageView;
use ndarray::Array4;
use ort::{session::Session, value::Value};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

// 1. Struktur untuk menyimpan data di memori (agar AI tidak di-load berulang kali)
struct AppState {
    session: Mutex<Session>,
    history: Mutex<Vec<HistoryItem>>,
}

#[derive(Clone, serde::Serialize)]
struct HistoryItem {
    thumbnail_b64: String, // Thumbnail base64 untuk UI
    png_bytes: Vec<u8>,    // File PNG asli untuk disimpan/disalin
}

#[tauri::command]
fn get_history(state: State<'_, AppState>) -> Vec<HistoryItem> {
    let history = state.history.lock().unwrap();
    // Kirim hanya thumbnail ke frontend agar ringan
    history.iter().map(|item| HistoryItem {
        thumbnail_b64: item.thumbnail_b64.clone(),
        png_bytes: vec![], // Kosongkan bytes saat dikirim ke UI
    }).collect()
}

#[tauri::command]
fn copy_from_history(index: usize, state: State<'_, AppState>) -> Result<(), String> {
    let history = state.history.lock().unwrap();
    if let Some(item) = history.get(index) {
        let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
        let img = image::load_from_memory(&item.png_bytes).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        clipboard.set_image(arboard::ImageData {
            width: rgba.width() as usize,
            height: rgba.height() as usize,
            bytes: rgba.into_raw().into(),
        }).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("History tidak ditemukan".into())
    }
}

#[tauri::command]
async fn save_from_history(index: usize, app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    use tauri_plugin_dialog::DialogExt;
    let history = state.history.lock().unwrap();
    
    if let Some(item) = history.get(index) {
        let file_path = app.dialog()
            .file()
            .set_file_name("dropbg_result.png")
            .add_filter("Image", &["png"])
            .blocking_save_file();

        if let Some(path) = file_path {
            std::fs::write(path.into_path().unwrap(), &item.png_bytes)
                .map_err(|e| format!("Gagal simpan: {}", e))?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn process_image(image_path: String, state: State<'_, AppState>) -> Result<usize, String> {
    // 1. Load Gambar
    let img = image::open(&image_path).map_err(|e| format!("Gagal buka gambar: {}", e))?;
    let original_size = img.dimensions();

    // 2. Preprocess (Resize ke 1024x1024 dan Normalisasi)
    let resized = img.resize_exact(1024, 1024, image::imageops::FilterType::Triangle);
    let rgb = resized.to_rgb8();
    
    let mut input_tensor = Array4::<f32>::zeros((1, 3, 1024, 1024));
    for (x, y, pixel) in rgb.enumerate_pixels() {
        input_tensor[[0, 0, y as usize, x as usize]] = (pixel[0] as f32 / 255.0 - 0.485) / 0.229;
        input_tensor[[0, 1, y as usize, x as usize]] = (pixel[1] as f32 / 255.0 - 0.456) / 0.224;
        input_tensor[[0, 2, y as usize, x as usize]] = (pixel[2] as f32 / 255.0 - 0.406) / 0.225;
    }

    // 3. Inference (Kunci session hanya saat proses AI)
    let mut session = state.session.lock().unwrap();
    let input_value = Value::from_array(input_tensor).map_err(|e| e.to_string())?;
    let outputs = session.run(ort::inputs!["input_image" => input_value]).map_err(|e| e.to_string())?;
    

    // 4. Postprocess (Ambil Mask)
    let (_shape, mask_data) = outputs[0].try_extract_tensor::<f32>().map_err(|e| e.to_string())?;
    
    // 5. Terapkan Mask ke Alpha Channel
    let mut result_img = img.to_rgba8();
    let width_ratio = 1024.0 / original_size.0 as f32;
    let height_ratio = 1024.0 / original_size.1 as f32;

    for (x, y, pixel) in result_img.enumerate_pixels_mut() {
        let mask_x = ((x as f32 * width_ratio) as usize).min(1023);
        let mask_y = ((y as f32 * height_ratio) as usize).min(1023);
        let mask_idx = mask_y * 1024 + mask_x;
        
        let mut alpha = mask_data[mask_idx];
        alpha = alpha.clamp(0.0, 1.0); 
        
        pixel[3] = (alpha * 255.0) as u8;
    }

    // 6. Buat Thumbnail (Base64) untuk UI
    let thumb = image::DynamicImage::ImageRgba8(result_img.clone()).thumbnail(200, 200);
    let mut thumb_png = std::io::Cursor::new(Vec::new());
    // Perbaikan: Menggunakan ImageFormat::Png
    thumb.write_to(&mut thumb_png, image::ImageFormat::Png).unwrap();
    let thumbnail_b64 = general_purpose::STANDARD.encode(thumb_png.into_inner());

    // 7. Simpan PNG penuh ke dalam memori (Bytes)
    let mut full_png = std::io::Cursor::new(Vec::new());
    // Perbaikan: Menggunakan ImageFormat::Png
    result_img.write_to(&mut full_png, image::ImageFormat::Png).unwrap();
    let png_bytes = full_png.into_inner();

    // 8. Masukkan ke History (Maksimal 5)
    let mut history = state.history.lock().unwrap();
    history.insert(0, HistoryItem { thumbnail_b64, png_bytes: png_bytes.clone() });
    if history.len() > 5 { history.pop(); }

    // 9. Copy ke Clipboard Otomatis
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_image(arboard::ImageData {
        width: result_img.width() as usize,
        height: result_img.height() as usize,
        bytes: result_img.into_raw().into(),
    }).map_err(|e| e.to_string())?;

    Ok(0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // LOAD MODEL DI AWAL (Caching)
            let env = ort::init().build().expect("Gagal init ONNX");
            let model_path = PathBuf::from("models/birefnet.onnx");
            let session = Session::builder(&env)
                .unwrap()
                .commit_from_file(model_path)
                .expect("Gagal load model AI");
            
            // Simpan session ke State global
            app.manage(AppState {
                session: Mutex::new(session),
                history: Mutex::new(Vec::new()),
            });
            Ok(())
        })
        // Daftarkan semua commands
        .invoke_handler(tauri::generate_handler![process_image, get_history, copy_from_history, save_from_history])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}