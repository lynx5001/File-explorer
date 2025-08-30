import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// customize build process

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
})
