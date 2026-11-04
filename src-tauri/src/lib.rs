use image::GenericImageView;
use ort::{session::Session, value::Value};
use ndarray::Array4;
use std::path::PathBuf;

#[tauri::command]
async fn test_remove_bg(image_path: String) -> Result<String, String> {
    // 1. Inisialisasi Environment ONNX Runtime (Wajib di ort 2.0)
    let env = ort::init()
        .build()
        .map_err(|e| format!("Gagal inisialisasi ONNX Runtime: {}", e))?;

    // 2. Load Model ONNX
    let model_path = PathBuf::from("models/birefnet.onnx");
     let mut session = Session::builder(&env) 
        .map_err(|e| e.to_string())?
        .commit_from_file(model_path)
        .map_err(|e| format!("Gagal load model: {}", e))?;

    // 3. Load Gambar
    let img = image::open(&image_path).map_err(|e| format!("Gagal buka gambar: {}", e))?;
    let original_size = img.dimensions();

    // 4. Preprocess: Resize ke 1024x1024 dan normalisasi (ImageNet mean/std)
    let resized = img.resize_exact(1024, 1024, image::imageops::FilterType::Triangle);
    let rgb = resized.to_rgb8();
    
    let mut input_tensor = Array4::<f32>::zeros((1, 3, 1024, 1024));
    for (x, y, pixel) in rgb.enumerate_pixels() {
        input_tensor[[0, 0, y as usize, x as usize]] = (pixel[0] as f32 / 255.0 - 0.485) / 0.229;
        input_tensor[[0, 1, y as usize, x as usize]] = (pixel[1] as f32 / 255.0 - 0.456) / 0.224;
        input_tensor[[0, 2, y as usize, x as usize]] = (pixel[2] as f32 / 255.0 - 0.406) / 0.225;
    }

    // 5. Inference
    let input_value = Value::from_array(input_tensor).map_err(|e| e.to_string())?;
    let outputs = session.run(ort::inputs!["input_image" => input_value]).map_err(|e| e.to_string())?;
    
    // 6. Postprocess: Ambil mask (output pertama)
    // Di ort 2.0, hasilnya adalah tuple (&Shape, &[f32]), bukan array biasa
    let (_shape, mask_data) = outputs[0]
        .try_extract_tensor::<f32>()
        .map_err(|e| e.to_string())?;
    
    // 7. Terapkan Mask (Alpha) ke Gambar Asli
    let mut result_img = img.to_rgba8();
    let width_ratio = 1024.0 / original_size.0 as f32;
    let height_ratio = 1024.0 / original_size.1 as f32;

    for (x, y, pixel) in result_img.enumerate_pixels_mut() {
        let mask_x = ((x as f32 * width_ratio) as usize).min(1023);
        let mask_y = ((y as f32 * height_ratio) as usize).min(1023);
        let mask_idx = mask_y * 1024 + mask_x;
        
        let mut alpha = mask_data[mask_idx];
        alpha = alpha.clamp(0.0, 1.0); // Pastikan nilai antara 0.0 - 1.0
        
        pixel[3] = (alpha * 255.0) as u8;
    }

    // 8. Simpan hasil ke file untuk testing
    let out_path = "test_output.png";
    result_img.save(out_path).map_err(|e| format!("Gagal simpan gambar: {}", e))?;

    Ok(out_path.to_string())
}

// ==========================================
// FUNGSI UTAMA TAURI
// ==========================================
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![test_remove_bg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}