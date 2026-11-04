import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite"; // <--- Import ini

export default defineConfig({
  plugins: [react(), tailwindcss()], // <--- Panggil di sini
  clearScreen: false,
  server: {
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});