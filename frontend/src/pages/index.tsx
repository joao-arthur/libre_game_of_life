import type { MouseEvent, ReactElement } from "react";
import { useEffect, useRef, useState } from "react";
import initWASM, {
    EngineCartesianPoint,
    engineGetPresets,
    EngineMatrixPoint,
    engineMoveBy,
    enginePause,
    engineResume,
    engineSetFPS,
    engineSetGap,
    engineSetPreset,
    engineSingleIteration,
    EngineStatus,
    engineToggle,
    engineZoomIn,
    engineZoomOut,
    engineZoomTo,
} from "game_of_life_engine";
import { Button } from "../components/Button";
import { RangeInput } from "../components/RangeInput";
import { Select } from "../components/Select";
import { useWindowDimension } from "../hooks/useWindowDimension";
import { useGameOfLife } from "../hooks/useGameOfLife";

export default function Main(): ReactElement {
    const { init, model } = useGameOfLife();
    const initiated = useRef(false);
    const [presets, setPresets] = useState<any[]>([]);
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    useEffect(() => {
        if (!initiated.current) {
            initiated.current = true;
            initWASM().then(() => {
                if (!canvasRef.current) {
                    return;
                }
                init(canvasRef.current);
                setPresets(engineGetPresets());
            });
        }
    }, []);

    useEffect(() => {
        function onKeyPress(e: KeyboardEvent) {
            switch (e.key) {
                case "w":
                    try {
                        engineMoveBy(new EngineCartesianPoint(BigInt(0), BigInt(1)));
                    } catch (e) {
                        console.error(e);
                    }
                    break;
                case "a":
                    try {
                        engineMoveBy(new EngineCartesianPoint(BigInt(-1), BigInt(0)));
                    } catch (e) {
                        console.error(e);
                    }
                    break;
                case "s":
                    try {
                        engineMoveBy(new EngineCartesianPoint(BigInt(0), BigInt(-1)));
                    } catch (e) {
                        console.error(e);
                    }
                    break;
                case "d":
                    try {
                        engineMoveBy(new EngineCartesianPoint(BigInt(1), BigInt(0)));
                    } catch (e) {
                        console.error(e);
                    }
                    break;
                case "+":
                    try {
                        engineZoomIn();
                    } catch (e) {
                        console.error(e);
                    }
                    break;
                case "-":
                    try {
                        engineZoomOut();
                    } catch (e) {
                        console.error(e);
                    }
                    break;
            }
        }
        window.addEventListener("keypress", onKeyPress);
        return () => {
            window.removeEventListener("keypress", onKeyPress);
        };
    }, [model]);

    function onClick(e: MouseEvent<HTMLCanvasElement>): void {
        if (!model) {
            return;
        }
        const row = e.pageX - e.currentTarget.offsetLeft;
        const col = e.pageY - e.currentTarget.offsetTop;
        try {
            engineToggle(new EngineMatrixPoint(BigInt(Number(col)), BigInt(Number(row))));
        } catch (e) {
            console.error(e);
        }
    }

    function handleZoomTo(size: number) {
        try {
            engineZoomTo(size);
        } catch (e) {
            console.error(e);
        }
    }

    function handleSetGap(gap: number) {
        try {
            engineSetGap(gap);
        } catch (e) {
            console.error(e);
        }
    }

    function handleSetFPS(fps: number) {
        try {
            engineSetFPS(fps);
        } catch (e) {
            console.error(e);
        }
    }

    function handleSetPreset(preset: string) {
        try {
            engineSetPreset(preset);
        } catch (e) {
            console.error(e);
        }
    }

    function handleToggle(): void {
        if (!model) return;
        switch (model.status) {
            case EngineStatus.Resumed:
                try {
                    enginePause();
                } catch (e) {
                    console.error(e);
                }
                break;
            case EngineStatus.Paused:
                try {
                    engineResume();
                } catch (e) {
                    console.error(e);
                }
                break;
        }
    }

    function handleIterate() {
        try {
            engineSingleIteration();
        } catch (e) {
            console.error(e);
        }
    }

    return (
        <main className="w-screen h-screen flex">
            <canvas
                onClick={onClick}
                className="m-auto"
                width={dimension}
                height={dimension}
                style={{ width: dimension, height: dimension }}
                ref={canvasRef}
            />
            <div className="flex flex-col">
                <div className="flex flex-col my-1">
                    <label htmlFor="preset">Preset</label>
                    <Select
                        id="preset"
                        groups={presets.map((group: any) => ({
                            label: group.info.name,
                            value: group.info.id,
                            options: group.items.map((item: any) => ({
                                label: item.name,
                                value: item.id,
                            })),
                        }))}
                        value={model?.preset || ""}
                        onChange={handleSetPreset}
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
                            onChange={handleSetGap}
                        />
                        <label className="w-8 text-center block">{model ? model.gap : 0}</label>
                    </div>
                </div>
                <div className="flex flex-col my-1">
                    <label htmlFor="size">Size</label>
                    <div className="flex">
                        <RangeInput
                            id="size"
                            min={2 + (model ? model.size % 2 === 0 ? 0 : 1 : 0)}
                            max={200 + (model ? model.size % 2 === 0 ? 0 : 1 : 0)}
                            step={2}
                            value={model ? model.size : 0}
                            onChange={handleZoomTo}
                        />
                        <label className="w-8 text-center block">{model ? model.size : 0}</label>
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
                            onChange={handleSetFPS}
                        />
                        <label className="w-8 text-center block">{model ? model.fps : 0}</label>
                    </div>
                </div>
                <span className="my-1">
                    <label>Iteration: {model ? Number(model.age) : 0}</label>
                </span>
                <Button
                    icon={model?.status === EngineStatus.Resumed ? "pause" : "play"}
                    label={model?.status === EngineStatus.Resumed ? "PAUSE" : "RESUME"}
                    onClick={handleToggle}
                />
                <Button icon="next" label="ITERATE" onClick={handleIterate} />
            </div>
        </main>
    );
}
