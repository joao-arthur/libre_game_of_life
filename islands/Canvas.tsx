import { useEffect, useRef } from "preact/hooks";
import { VNode } from "preact";
import { useWindowDimensions } from "../components/useWindowDimensions.ts";

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
    }, []);

    return (
        <canvas
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
