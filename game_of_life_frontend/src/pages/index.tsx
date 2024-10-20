import type { MouseEvent, ReactElement } from "react";
import { useEffect, useRef, useState } from "react";
import initWASM, {
    EngineCartesianPoint,
    engineMove,
    enginePause,
    engineResume,
    engineSetDimension,
    engineSetFPS,
    engineSetGap,
    engineSetPreset,
    engineSingleIteration,
    EngineStatus,
} from "game_of_life_engine";
import { Button } from "../components/Button";
import { RangeInput } from "../components/RangeInput";
import { Select } from "../components/Select";
import { useWindowDimension } from "../hooks/useWindowDimension";
import { useGameOfLife } from "../hooks/useGameOfLife";

export default function Main(): ReactElement {
    const { init, model } = useGameOfLife();
    const [presets, setPresets] = useState<any[]>([]);
    const [preset, setPreset] = useState<string>(undefined!);
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    //const modelLength = model ? Number(getModelLength({ Ok: model.model })) : 0;
    let modelLength = 1000;

    useEffect(() => {
        globalThis.setTimeout(() => {
            initWASM().then(() => {
                if (!canvasRef.current) {
                    return;
                }
                console.log("dfffff");
                init(canvasRef.current);
            });
        }, 1000);
    }, []);

    useEffect(() => {
        //  setPresets(getPresetGroups());
    }, []);

    useEffect(() => {
        function onKeyPress(e: KeyboardEvent) {
            switch (e.key) {
                case "w":
                    engineMove(new EngineCartesianPoint(BigInt(0), BigInt(1)));
                    break;
                case "a":
                    engineMove(new EngineCartesianPoint(BigInt(-1), BigInt(0)));
                    break;
                case "s":
                    engineMove(new EngineCartesianPoint(BigInt(0), BigInt(-1)));
                    break;
                case "d":
                    engineMove(new EngineCartesianPoint(BigInt(1), BigInt(0)));
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
    }, [model]);

    function onClick(e: MouseEvent<HTMLCanvasElement>): void {
        if (!model) {
            return;
        }
        /*  const x = e.pageX - e.currentTarget.offsetLeft;
        const y = e.pageY - e.currentTarget.offsetTop;
        const length = getModelLength({ Ok: model.model });
        const middlePoint = getModelMiddlePoint({ Ok: model.model });
        const cellSize = getModelCellSize({ Ok: model.model }, model.dimension);
        if (cellSize <= 0) {
            return;
        }
        const col = absoluteToRelative(BigInt(x), cellSize);
        const row = absoluteToRelative(BigInt(y), cellSize);
        const point = indexToPoint(new ArrPos(BigInt(row), BigInt(col)), BigInt(length));
        const cell = new EngineCartesianPoint(
            BigInt(Number(point.x + middlePoint.x)),
            BigInt(Number(point.y + middlePoint.y)),
        );
        engineToggle(cell);
        setPreset("");*/
    }

    function handleToggle(): void {
        if (!model) return;
        switch (model.status) {
            case EngineStatus.Resumed:
                enginePause();
                break;
            case EngineStatus.Paused:
                engineResume();
                break;
        }
    }

    function zoom(offset: number) {
        //if (!model) return;
        //const newSize = Number(getModelLength({ Ok: model.model })) + offset;
        //if (newSize < 2) return;
        //if (newSize > 120) return;
        //engineZoom(newSize);
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
                        groups={presets.map((group: any) => ({
                            label: group.info.name,
                            value: group.info.id,
                            options: group.sub_groups
                                .flatMap((subGroup: any) => subGroup.items)
                                .map((item: any) => ({
                                    label: item.name,
                                    value: item.id,
                                })),
                        }))}
                        value={preset}
                        onChange={(preset) => {
                            setPreset(preset);
                            engineSetPreset(preset);
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
                            onChange={(gap) => engineSetGap(gap)}
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
                            min={2 + (model ? modelLength % 2 === 0 ? 0 : 1 : 0)}
                            max={200 + (model ? modelLength % 2 === 0 ? 0 : 1 : 0)}
                            step={2}
                            value={model ? modelLength : 0}
                            onChange={(size) => engineSetDimension(size)}
                        />
                        <label className="w-8 text-center block">
                            {model ? modelLength : 0}
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
                            onChange={(fps) => {
                                engineSetFPS(fps);
                            }}
                        />
                        <label className="w-8 text-center block">
                            {model ? model.fps : 0}
                        </label>
                    </div>
                </div>
                <span className="my-1">
                    <label>
                        Iteration: {0}
                    </label>
                </span>
                <Button
                    icon={model?.status === EngineStatus.Resumed ? "pause" : "play"}
                    label={model?.status === EngineStatus.Resumed ? "PAUSE" : "RESUME"}
                    onClick={handleToggle}
                />
                <Button
                    icon="next"
                    label="ITERATE"
                    onClick={() => engineSingleIteration()}
                />
            </div>
        </main>
    );
}
