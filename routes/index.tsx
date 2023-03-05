import { Head } from "$fresh/runtime.ts";
import { VNode } from "preact";
import { Button } from "../components/Button.tsx";
import Canvas from "../islands/Canvas.tsx";

export default function Index(): VNode {
    return (
        <>
            <Head>
                <title>Game Of Life</title>
            </Head>
            <main class="w-screen h-screen flex">
                <Canvas />
                <div className="flex flex-col">
                    <div className="flex flex-col">
                        <label for="gap">Gap</label>
                        <input
                            type="range"
                            id="gap"
                            name="gap"
                            min="0"
                            max="10"
                            value="1"
                            step="1"
                        />
                    </div>
                    <div className="flex flex-col">
                        <label for="size">Size</label>
                        <input
                            type="range"
                            id="size"
                            name="size"
                            min="10"
                            max="100"
                            value="20"
                            step="1"
                        />
                    </div>
                    <div className="flex flex-col">
                        <label for="speed">Speed</label>
                        <input
                            type="range"
                            id="speed"
                            name="speed"
                            min="1"
                            max="100"
                            value="20"
                            step="1"
                        />
                    </div>
                    <span>
                        0
                        <label>Iteration</label>
                    </span>
                    <Button label="pause" onClick={() => {}} />
                    <Button label="iterate" onClick={() => {}} />
                </div>
            </main>
        </>
    );
}
