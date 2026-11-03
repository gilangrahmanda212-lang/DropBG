import { Settings, Info, Upload, Minus, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

function App() {
  // Ambil referensi ke jendela aplikasi saat ini
  const appWindow = getCurrentWindow();

  const handleMinimize = () => {
    appWindow.minimize();
  };

  const handleClose = () => {
    appWindow.close();
  };

  return (
    <div className="w-full h-screen flex flex-col bg-white overflow-hidden">
      
      {/* 1. Custom Title Bar */}
      <div 
        data-tauri-drag-region 
        className="h-10 flex items-center justify-between px-4 border-b border-border bg-white shrink-0"
      >
        <div className="flex items-center gap-2 pointer-events-none">
          <div className="w-4 h-4 bg-accent rounded-sm"></div>
          <span className="text-sm font-semibold text-text-primary tracking-tight">DropBG</span>
        </div>
        
        {/* Window Controls */}
        <div className="flex items-center gap-1">
          <button 
            onClick={handleMinimize}
            className="w-7 h-7 flex items-center justify-center hover:bg-surface-muted rounded text-text-secondary transition-colors"
          >
            <Minus className="w-4 h-4" />
          </button>
          <button 
            onClick={handleClose}
            className="w-7 h-7 flex items-center justify-center hover:bg-red-500 hover:text-white rounded text-text-secondary transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* 2. Main Content */}
      <div className="flex-1 flex flex-col p-4 gap-4 overflow-y-auto bg-white">
        
        {/* DropZone */}
        <div className="h-44 flex flex-col items-center justify-center rounded-xl border-2 border-dashed border-border bg-surface-subtle hover:border-accent hover:bg-accent-soft transition-all duration-150 cursor-pointer">
          <Upload className="w-8 h-8 text-text-muted" strokeWidth={1.5} />
          <p className="mt-3 text-sm font-medium text-text-primary">Drop image here</p>
          <p className="mt-1 text-xs text-text-muted">JPG · PNG · WebP · max 20MB</p>
        </div>

        {/* History Area */}
        <div>
          <p className="text-xs font-medium text-text-secondary mb-2">History</p>
          <div className="flex gap-2">
            <div className="w-14 h-14 rounded-lg border border-border bg-surface-muted"></div>
            <div className="w-14 h-14 rounded-lg border border-border bg-surface-muted"></div>
            <div className="w-14 h-14 rounded-lg border border-border bg-surface-muted"></div>
          </div>
        </div>

      </div>

      {/* 3. Footer */}
      <div className="h-12 border-t border-border bg-white flex items-center justify-between px-4 shrink-0">
        <button className="flex items-center gap-2 text-sm text-text-secondary hover:text-text-primary transition-colors">
          <Settings className="w-4 h-4" />
          Settings
        </button>
        <button className="flex items-center gap-2 text-sm text-text-secondary hover:text-text-primary transition-colors">
          <Info className="w-4 h-4" />
          About
        </button>
      </div>

    </div>
  );
}

export default App;
