import { useEffect, useRef, useState } from "preact/hooks";
import { JSX, VNode } from "preact";
import { useWindowDimensions } from "../hooks/useWindowDimensions.ts";
import { CanvasDrawContext } from "../src/adapters/canvasDrawContext.ts";
import { Button } from "../components/Button.tsx";
import { RangeInput } from "../components/RangeInput.tsx";

export default function Canvas(): VNode {
    const dimensions = useWindowDimensions();
    const dimension = Math.min(dimensions.height, dimensions.width);
    const canvasRef = useRef<HTMLCanvasElement>(null);

    const [gap, setGap] = useState(1);
    const [size, setSize] = useState(20);
    const [fps, setFps] = useState(20);
    const [status, setStatus] = useState<"resumed" | "paused">(
        "resumed",
    );

    useEffect(() => {
        if (!canvasRef.current) {
            return;
        }
        const context = canvasRef.current.getContext("2d");
        if (!context) {
            return;
        }
        const drawContext = new CanvasDrawContext(context);

        console.log({
            drawContext,
            isPaused,
            interval: 500,
            dimensions,
        });

        globalThis.setInterval(
            () =>
                renderLoop({
                    drawContext,
                    dimensions,
                    status,
                }),
            1000 / fps,
        );
    }, []);

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
                        value={gap}
                        setValue={setGap}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="size">Size</label>
                    <RangeInput
                        id="size"
                        min={10}
                        max={100}
                        step={1}
                        value={size}
                        setValue={setSize}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="fps">fps</label>
                    <RangeInput
                        id="fps"
                        min={1}
                        max={100}
                        step={1}
                        value={fps}
                        setValue={setFps}
                    />
                </div>
                <span>
                    0
                    <label>Iteration</label>
                </span>
                {status === "resumed"
                    ? (
                        <Button
                            label="pause"
                            onClick={() => {
                                setStatus("paused");
                            }}
                        />
                    )
                    : (
                        <Button
                            label="resume"
                            onClick={() => {
                                setStatus("resumed");
                            }}
                        />
                    )}
                <Button label="iterate" onClick={() => {}} />
            </div>
        </>
    );
}
