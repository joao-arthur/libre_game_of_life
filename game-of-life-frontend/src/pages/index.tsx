import type { MouseEvent, ReactElement } from "react";
import { useEffect, useRef, useState } from "react";
import { cartesianPlaneFns, modelFns } from "game-of-life-engine";
import { Button } from "../components/Button";
import { RangeInput } from "../components/RangeInput";
import { Select } from "../components/Select";
import { buildPresetsOptions } from "../components/buildPresetsOptions";
import { useWindowDimension } from "../hooks/useWindowDimension";
import { useGameOfLife } from "../hooks/useGameOfLife";

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

    useEffect(() => {
        function onKeyPress(e: KeyboardEvent) {
            if (!controller) {
                return;
            }
            switch (e.key) {
                case "w":
                    controller.move({ x: 0, y: 1 });
                    break;
                case "a":
                    controller.move({ x: -1, y: 0 });
                    break;
                case "s":
                    controller.move({ x: 0, y: -1 });
                    break;
                case "d":
                    controller.move({ x: 1, y: 0 });
                    break;
                case "+":
                    zoomIn();
                    break;
                case "-":
                    zoomOut();
                    break;
            }
        }
        window.addEventListener("keypress", onKeyPress);
        return () => {
            window.removeEventListener("keypress", onKeyPress);
        };
    }, [controller, model, controller]);

    function onClick(
        e: MouseEvent<HTMLCanvasElement>,
    ): void {
        if (!controller) return;
        if (!model) {
            return;
        }
        const x = e.pageX - e.currentTarget.offsetLeft;
        const y = e.pageY - e.currentTarget.offsetTop;
        const length = modelFns.getLength(model.model);
        const middlePoint = modelFns.getMiddlePoint(model.model);
        const cellSize = modelFns.getCellSize(
            model.model,
            model.dimension,
        );
        const col = cartesianPlaneFns.absoluteToRelative(x, cellSize);
        const row = cartesianPlaneFns.absoluteToRelative(y, cellSize);
        const point = cartesianPlaneFns.indexToPoint(
            { col, row },
            length,
        );
        const cell = {
            x: point.x + middlePoint.x,
            y: point.y + middlePoint.y,
        };
        controller.toggleCell(cell);
        setPreset("");
    }

    function handleToggle(): void {
        if (!model) return;
        if (!controller) return;
        if (model.status === "resumed") {
            controller.pause();
        } else {
            controller.resume();
        }
    }

    function zoom(offset: number) {
        if (!model) return;
        if (!controller) return;
        const newSize = modelFns.getLength(model.model) + offset;
        if (newSize < 2) return;
        if (newSize > 120) return;
        controller.setSize(newSize);
    }

    function zoomIn() {
        zoom(-2);
    }

    function zoomOut() {
        zoom(+2);
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
                            max={2}
                            step={1}
                            value={model ? model.gap : 0}
                            onChange={(gap) =>
                                controller?.setGap(gap)}
                        />
                        <label className="w-8 text-center block">
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
                            min={2 + (model
                                ? modelFns.getLength(model.model) %
                                            2 === 0
                                    ? 0
                                    : 1
                                : 0)}
                            max={200 + (model
                                ? modelFns.getLength(model.model) %
                                            2 === 0
                                    ? 0
                                    : 1
                                : 0)}
                            step={2}
                            value={model
                                ? modelFns.getLength(model.model)
                                : 0}
                            onChange={(size) =>
                                controller?.setSize(size)}
                        />
                        <label className="w-8 text-center block">
                            {model
                                ? modelFns.getLength(model.model)
                                : 0}
                        </label>
                    </div>
                </div>
                <div className="flex flex-col my-1">
                    <label htmlFor="fps">FPS</label>
                    <div className="flex">
                        <RangeInput
                            id="fps"
                            min={1}
                            max={60}
                            step={1}
                            value={model ? model.fps : 0}
                            onChange={(fps) =>
                                controller?.setFps(fps)}
                        />
                        <label className="w-8 text-center block">
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
