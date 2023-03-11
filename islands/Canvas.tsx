import { useEffect, useRef } from "preact/hooks";
import { JSX, VNode } from "preact";
import { modelFns } from "../src/game/model/mod.ts";
import { useWindowDimensions } from "../hooks/useWindowDimensions.ts";
import { CanvasDrawContext } from "../src/adapters/canvasDrawContext.ts";
import { Button } from "../components/Button.tsx";
import { RangeInput } from "../components/RangeInput.tsx";
import { useGameStore } from "../integrations/useGameStore.ts";
import { drawContextType } from "../src/ports/drawContext.ts";
import { stateType } from "../src/game/cell/mod.ts";

let timeoutId = 0;
let drawContext: drawContextType;

function getSquareColor(state: stateType): string {
    switch (state) {
        case stateType.DEAD:
            return "#2e2e2e";
        case stateType.ALIVE:
            return "#dbdbdb";
    }
}

export default function Canvas(): VNode {
    const dimensions = useWindowDimensions();
    const dimension = Math.min(dimensions.height, dimensions.width);
    const canvasRef = useRef<HTMLCanvasElement>(null);

    const {
        store,
        actions,
    } = useGameStore();

    useEffect(() => {
        if (!canvasRef.current) {
            return;
        }
        const context = canvasRef.current.getContext("2d");
        if (!context) {
            return;
        }
        drawContext = new CanvasDrawContext(context);
    }, []);

    useEffect(() => {
        const { iterate } = modelFns;
        console.log("render", 1000 / store.fps);

        if (timeoutId) {
            globalThis.clearInterval(timeoutId);
        }
        if (store.status === "paused") return;
        timeoutId = globalThis.setInterval(() => {
            console.log("render");
            //    model = iterate(model);
            const dimensionSize = Math.min(
                dimensions.width,
                dimensions.height,
            );
            const modelSize = Math.min(
                store.model.width,
                store.model.height,
            );
            const size = dimensionSize / modelSize;

            modelFns.forEach(
                store.model,
                ({ column, row }, state) => {
                    drawContext.drawSquare({
                        x: column * size + store.gap,
                        y: row * size + store.gap,
                        size: size - store.gap * 2,
                    }, getSquareColor(state));
                },
            );
        }, 1000 / store.fps);
    }, [store]);

    function onClick(
        e: JSX.TargetedMouseEvent<HTMLCanvasElement>,
    ): void {
        const x = e.pageX - e.currentTarget.offsetLeft;
        const y = e.pageY - e.currentTarget.offsetTop;
    }

    return (
        <>
            <canvas
                onClick={onClick}
                className="m-auto"
                width={dimension}
                height={dimension}
                style={{
                    width: dimension,
                    height: dimension,
                }}
                ref={canvasRef}
            />
            <div className="flex flex-col">
                <div className="flex flex-col">
                    <label for="gap">Gap</label>
                    <RangeInput
                        id="gap"
                        min={0}
                        max={10}
                        step={1}
                        value={store.gap}
                        setValue={actions.setGap}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="size">Size</label>
                    <RangeInput
                        id="size"
                        min={10}
                        max={100}
                        step={1}
                        value={store.size}
                        setValue={actions.setSize}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="fps">fps</label>
                    <RangeInput
                        id="fps"
                        min={1}
                        max={100}
                        step={1}
                        value={store.fps}
                        setValue={actions.setFps}
                    />
                </div>
                <span>
                    0
                    <label>Iteration</label>
                </span>
                {store.status === "resumed"
                    ? <Button label="pause" onClick={actions.pause} />
                    : (
                        <Button
                            label="resume"
                            onClick={actions.resume}
                        />
                    )}
                <Button label="iterate" onClick={actions.iterate} />
            </div>
        </>
    );
}
