import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import path from "path";

export default defineConfig({
    plugins: [
        sveltekit(),
    ],
    server: {
        fs: {
            allow: [
                path.resolve(__dirname, "../game_of_life_engine"),
            ],
        },
    },
});
