import { Head } from "$fresh/runtime.ts";
import { VNode } from "preact";
import Canvas from "../islands/Canvas.tsx";

export default function Index(): VNode {
    return (
        <>
            <Head>
                <title>Game Of Life</title>
            </Head>
            <main class="w-screen h-screen flex">
                <Canvas />
                <div>
                    <input
                        type="range"
                        id="gap"
                        name="gap"
                        min="0"
                        max="10"
                        value="1"
                        step="1"
                    >
                        <label for="gap">Gap</label>
                    </input>
                    <input
                        type="range"
                        id="size"
                        name="size"
                        min="10"
                        max="100"
                        value="20"
                        step="1"
                    >
                        <label for="size">Size</label>
                    </input>
                    <input
                        type="range"
                        id="speed"
                        name="speed"
                        min="1"
                        max="100"
                        value="20"
                        step="1"
                    >
                        <label for="speed">Speed</label>
                    </input>
                    <span>
                        0
                        <label>Iteration</label>
                    </span>
                    <button>pause</button>
                    <button>iterate</button>
                </div>
            </main>
        </>
    );
}
