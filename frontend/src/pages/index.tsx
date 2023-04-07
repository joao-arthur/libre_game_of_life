import type {ReactElement, MouseEvent} from  'react';
import { useEffect, useRef, useState } from "react";
import { Button } from "../components/Button";
import { RangeInput } from "../components/RangeInput";
import { Select } from "../components/Select";
import { useWindowDimension } from "../hooks/useWindowDimension";
import { useGameOfLife } from "../hooks/useGameOfLife";
import { absoluteToRelative } from "../features/absoluteToRelative/absoluteToRelative";
import { buildPresetsOptions } from "../components/buildPresetsOptions";

export default function Main(): ReactElement {
    const {
        init,
        model,
        controller,
    } = useGameOfLife();
    const [preset, setPreset] = useState<string>(undefined!);

    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (!canvasRef.current) {
            return;
        }
        init(canvasRef.current);
    }, []);

    useEffect(() => {
        if (!controller) return;
        controller.setPreset("block");
        setPreset("block");
    }, [controller]);

    function onClick(
        e: MouseEvent<HTMLCanvasElement>,
    ): void {
        if (!controller) return;
        if (!model) {
            return;
        }
        const x = e.pageX - e.currentTarget.offsetLeft;
        const y = e.pageY - e.currentTarget.offsetTop;
        const unitSize = model.dimension / model.model.size;

        const column = absoluteToRelative(x, unitSize);
        const row = absoluteToRelative(y, unitSize);

        controller.toggleCell({ column, row });
        setPreset("");
    }

    function handleToggle(): void {
        if (model?.status === "resumed") {
            controller?.pause();
        } else {
            controller?.resume();
        }
    }

    return (
        <main className="w-screen h-screen flex">
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
                <div className="flex flex-col my-1">
                    <label htmlFor="preset">Preset</label>
                    <Select
                        id="preset"
                        groups={buildPresetsOptions()}
                        value={preset}
                        onChange={(preset) => {
                            setPreset(preset);
                            controller?.setPreset(preset);
                        }}
                    />
                </div>
                <div className="flex flex-col my-1">
                    <label htmlFor="gap">Gap</label>
                    <div className="flex">
                        <RangeInput
                            id="gap"
                            min={0}
                            max={5}
                            step={1}
                            value={model ? model.gap : 0}
                            onChange={(gap) =>
                                controller?.setGap(gap)}
                        />
                        <label className="w-6 text-center block">
                            {model ? model.gap : 0}
                        </label>
                    </div>
                </div>
                <div className="flex flex-col my-1">
                    <label htmlFor="size">
                        Size
                    </label>
                    <div className="flex">
                        <RangeInput
                            id="size"
                            min={2}
                            max={100}
                            step={2}
                            value={model ? model.model.size : 0}
                            onChange={(size) =>
                                controller?.setSize(size)}
                        />
                        <label className="w-6 text-center block">
                            {model ? model.model.size : 0}
                        </label>
                    </div>
                </div>
                <div className="flex flex-col my-1">
                    <label htmlFor="fps">FPS</label>
                    <div className="flex">
                        <RangeInput
                            id="fps"
                            min={1}
                            max={20}
                            step={1}
                            value={model ? model.fps : 0}
                            onChange={(fps) =>
                                controller?.setFps(fps)}
                        />
                        <label className="w-6 text-center block">
                            {model ? model.fps : 0}
                        </label>
                    </div>
                </div>
                <span className="my-1">
                    <label>
                        Iteration: {model ? model.model.iteration : 0}
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
        </main>
    );
}
