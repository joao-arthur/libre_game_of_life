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
            </main>
        </>
    );
}
