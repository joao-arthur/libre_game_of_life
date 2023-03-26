import { useEffect, useRef } from "preact/hooks";
import { JSX, VNode } from "preact";
import { useWindowDimension } from "../hooks/useWindowDimension.ts";
import { Button } from "../components/Button.tsx";
import { RangeInput } from "../components/RangeInput.tsx";
import { useGameOfLife } from "../hooks/useGameOfLife.ts";
import { absoluteToRelative } from "../src/features/absoluteToRelative/absoluteToRelative.ts";

export default function Canvas(): VNode {
    const {
        init,
        model,
        controller,
    } = useGameOfLife();

    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (!canvasRef.current) {
            return;
        }
        init(canvasRef.current);
    }, []);

    function onClick(
        e: JSX.TargetedMouseEvent<HTMLCanvasElement>,
    ): void {
        if (!model) {
            return;
        }
        const x = e.pageX - e.currentTarget.offsetLeft;
        const y = e.pageY - e.currentTarget.offsetTop;
        const unitSize = model.dimension / model.model.size;

        const column = absoluteToRelative(x, unitSize);
        const row = absoluteToRelative(y, unitSize);

        controller?.toggleCell({ column, row });
    }

    function handleToggle(): void {
        if (model?.status === "resumed") {
            controller?.pause();
        } else {
            controller?.resume();
        }
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
                    <label for="gap">Gap: {model?.gap}</label>
                    <RangeInput
                        id="gap"
                        min={0}
                        max={7}
                        step={1}
                        value={model ? model.gap : 0}
                        setValue={(gap) => controller?.setGap(gap)}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="size">
                        Size: {model?.model.size}
                    </label>
                    <RangeInput
                        id="size"
                        min={2}
                        max={100}
                        step={2}
                        value={model ? model.model.size : 0}
                        setValue={(size) => controller?.setSize(size)}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="fps">FPS: {model?.fps}</label>
                    <RangeInput
                        id="fps"
                        min={1}
                        max={20}
                        step={1}
                        value={model ? model.fps : 0}
                        setValue={(fps) => controller?.setFps(fps)}
                    />
                </div>
                <span>
                    <label>Iteration: {model?.model.iteration}</label>
                </span>
                <Button
                    label={model?.status === "resumed"
                        ? "pause"
                        : "resume"}
                    onClick={handleToggle}
                />
                <Button
                    label="iterate"
                    onClick={() => controller?.singleIteration()}
                />
            </div>
        </>
    );
}
