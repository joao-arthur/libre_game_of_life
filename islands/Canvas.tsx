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
                    <label for="gap">Gap</label>
                    <div className="flex">
                        <RangeInput
                            id="gap"
                            min={0}
                            max={5}
                            step={1}
                            value={model ? model.gap : 0}
                            setValue={(gap) =>
                                controller?.setGap(gap)}
                        />
                        <label className="w-6 text-center block">
                            {model?.gap}
                        </label>
                    </div>
                </div>
                <div className="flex flex-col">
                    <label for="size">
                        Size
                    </label>
                    <div className="flex">
                        <RangeInput
                            id="size"
                            min={2}
                            max={100}
                            step={2}
                            value={model ? model.model.size : 0}
                            setValue={(size) =>
                                controller?.setSize(size)}
                        />
                        <label className="w-6 text-center block">
                            {model?.model.size}
                        </label>
                    </div>
                </div>
                <div className="flex flex-col">
                    <label for="fps">FPS</label>
                    <div className="flex">
                        <RangeInput
                            id="fps"
                            min={1}
                            max={99999}
                            step={1}
                            value={model ? model.fps : 0}
                            setValue={(fps) =>
                                controller?.setFps(fps)}
                        />
                        <label className="w-6 text-center block">
                            {model?.fps}
                        </label>
                    </div>
                </div>
                <span className="my-1">
                    <label>
                        Iteration: {model?.model.iteration}
                    </label>
                </span>
                <Button
                    icon={model?.status === "resumed"
                        ? "pause"
                        : "play"}
                    label={model?.status === "resumed"
                        ? "PAUSE"
                        : "RESUME"}
                    onClick={handleToggle}
                />
                <Button
                    icon="next"
                    label="ITERATE"
                    onClick={() => controller?.singleIteration()}
                />
            </div>
        </>
    );
}
