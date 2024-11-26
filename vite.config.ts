import { defineConfig } from 'vite'
//import { PreRenderedAsset } from 'rollup'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    //publicDir: "world/assets",
    //build: {
    //    rollupOptions: {
    //        output: {
    //            assetFileNames(_assetInfo: PreRenderedAsset) {
    //                return "assets";
    //            },
    //        }
    //    },
    //},
})
