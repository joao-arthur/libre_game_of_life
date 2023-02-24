import { useEffect, useRef } from "preact/hooks";
import { JSX, VNode } from "preact";
import { useWindowDimensions } from "../components/useWindowDimensions.ts";
import { renderLoop } from "../src/features/render.ts";
import { CanvasDrawContext } from "../src/adapters/canvasDrawContext.ts";

export default function Canvas(): VNode {
    const dimensions = useWindowDimensions();
    const dimension = Math.min(dimensions.height, dimensions.width);
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        if (!canvasRef.current) {
            return;
        }
        const context = canvasRef.current.getContext("2d");
        if (!context) {
            return;
        }
        const drawContext = new CanvasDrawContext(context);

        globalThis.setInterval(
            () =>
                renderLoop({
                    drawContext,
                    dimensions,
                    isPaused: false,
                }),
            250,
        );
    }, []);

    function onClick(
        e: JSX.TargetedMouseEvent<HTMLCanvasElement>,
    ): void {
        const x = e.pageX - e.currentTarget.offsetLeft;
        const y = e.pageY - e.currentTarget.offsetTop;
    }

    return (
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
    );
}
