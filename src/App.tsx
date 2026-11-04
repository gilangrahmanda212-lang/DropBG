import { useState, useEffect } from "react";
import { Settings, Info, Upload, Minus, X, Loader2, CheckCircle, Download } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";

interface HistoryItem {
  thumbnail_b64: string;
}

function App() {
  const appWindow = getCurrentWindow();
  const [isProcessing, setIsProcessing] = useState(false);
  const [status, setStatus] = useState<"idle" | "success" | "error">("idle");
  const [errorMsg, setErrorMsg] = useState("");
  const [history, setHistory] = useState<HistoryItem[]>([]);

  const loadHistory = async () => {
    try {
      const data = await invoke<HistoryItem[]>("get_history");
      setHistory(data);
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    loadHistory();

    let unlisten: () => void;
    const setupDragDrop = async () => {
      unlisten = await getCurrentWebview().onDragDropEvent(async (event) => {
        if (event.payload.type === 'drop') {
          const paths = event.payload.paths;
          if (paths.length > 0) {
            if (!/\.(jpg|jpeg|png|webp)$/i.test(paths[0])) {
              setErrorMsg("Format tidak didukung. Gunakan JPG, PNG, atau WebP.");
              setStatus("error");
              setTimeout(() => setStatus("idle"), 3000);
              return;
            }

            setIsProcessing(true);
            setStatus("idle");

            try {
              await invoke("process_image", { imagePath: paths[0] });
              setStatus("success");
              loadHistory();
              setTimeout(() => setStatus("idle"), 3000);
            } catch (err) {
              setErrorMsg(String(err));
              setStatus("error");
              setTimeout(() => setStatus("idle"), 5000);
            } finally {
              setIsProcessing(false);
            }
          }
        }
      });
    };
    setupDragDrop();
    return () => { if (unlisten) unlisten(); };
  }, []);

  const handleCopyHistory = async (index: number) => {
    try {
      await invoke("copy_from_history", { index });
      setStatus("success");
      setTimeout(() => setStatus("idle"), 2000);
    } catch (err) {
      alert(err);
    }
  };

  const handleSaveHistory = async (index: number) => {
    try {
      await invoke("save_from_history", { index });
    } catch (err) {
      alert(err);
    }
  };

  return (
    <div className="w-full h-screen flex flex-col bg-white overflow-hidden">
      <div data-tauri-drag-region className="h-10 flex items-center justify-between px-4 border-b border-border bg-white shrink-0">
        <div data-tauri-drag-region className="flex items-center gap-2 pointer-events-none">
          <div className="w-4 h-4 bg-accent rounded-sm pointer-events-none"></div>
          <span className="text-sm font-semibold text-text-primary pointer-events-none">DropBG</span>
        </div>
        <div className="flex items-center gap-1">
          <button onClick={() => appWindow.minimize()} className="w-7 h-7 flex items-center justify-center hover:bg-surface-muted rounded text-text-secondary"><Minus className="w-4 h-4" /></button>
          <button onClick={() => appWindow.close()} className="w-7 h-7 flex items-center justify-center hover:bg-red-500 hover:text-white rounded text-text-secondary"><X className="w-4 h-4" /></button>
        </div>
      </div>

      <div className="flex-1 flex flex-col p-4 gap-4 overflow-y-auto bg-white">
        <div className={`h-44 shrink-0 flex flex-col items-center justify-center rounded-xl border-2 border-dashed transition-all duration-150
          ${isProcessing ? "border-accent bg-accent-soft" : 
            status === "success" ? "border-green-500 bg-green-50" : 
            status === "error" ? "border-red-500 bg-red-50" : 
            "border-border bg-surface-subtle"}`}>
          
          {isProcessing ? (
            <><Loader2 className="w-8 h-8 text-accent animate-spin" /><p className="mt-3 text-sm font-medium">Memproses AI...</p></>
          ) : status === "success" ? (
            <><CheckCircle className="w-8 h-8 text-green-500" /><p className="mt-3 text-sm font-medium">Berhasil dicopy ke clipboard!</p></>
          ) : status === "error" ? (
            <><X className="w-8 h-8 text-red-500" /><p className="mt-3 text-sm font-medium px-4 text-center">{errorMsg}</p></>
          ) : (
            <><Upload className="w-8 h-8 text-text-muted" strokeWidth={1.5} /><p className="mt-3 text-sm font-medium">Drop image here</p><p className="mt-1 text-xs text-text-muted">JPG · PNG · WebP</p></>
          )}
        </div>

        <div className="flex-1">
          <p className="text-xs font-medium text-text-secondary mb-2">History (Klik untuk Copy ulang)</p>
          <div className="flex gap-2 flex-wrap">
            {history.length === 0 ? (
              <p className="text-xs text-text-muted">Belum ada riwayat.</p>
            ) : (
              history.map((item, idx) => (
                <div key={idx} className="relative group">
                  <img 
                    src={`data:image/png;base64,${item.thumbnail_b64}`} 
                    alt="history" 
                    onClick={() => handleCopyHistory(idx)}
                    className="w-16 h-16 rounded-lg border border-border object-cover cursor-pointer hover:border-accent transition-all bg-[url('https://www.transparenttextures.com/patterns/checkerboard.png')]"
                  />
                  <button 
                    onClick={() => handleSaveHistory(idx)}
                    className="absolute -bottom-1 -right-1 bg-white border border-border p-1 rounded-md opacity-0 group-hover:opacity-100 transition-opacity shadow-sm hover:text-accent"
                    title="Save to file"
                  >
                    <Download className="w-3 h-3" />
                  </button>
                </div>
              ))
            )}
          </div>
        </div>
      </div>

      <div className="h-12 border-t border-border bg-white flex items-center justify-between px-4 shrink-0">
        <button className="flex items-center gap-2 text-sm text-text-secondary hover:text-text-primary"><Settings className="w-4 h-4" />Settings</button>
        <button className="flex items-center gap-2 text-sm text-text-secondary hover:text-text-primary"><Info className="w-4 h-4" />About</button>
      </div>
    </div>
  );
}

export default App;