import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
    
    plugins: [
        sveltekit(),
    ],
    server: {
      fs: {
        allow: [
            '/home/joao_arthur/Programming/Games/libre_game_of_life/game_of_life_engine'
          ]
      },
    },
});
