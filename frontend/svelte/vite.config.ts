import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import path from "node:path";

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	resolve: {
		alias: {
			"@breadcrumbs": path.resolve(__dirname, "../../backend/bindings/output/breadcrumbs.js"),
		},
	},
	server: {
		fs: {
			allow: [
				"../../backend/bindings/output/breadcrumbs_bg.wasm",
			],
		},
	},
});
