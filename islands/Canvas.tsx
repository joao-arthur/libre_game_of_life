import { useEffect, useRef, useState } from "preact/hooks";
import { JSX, VNode } from "preact";
import { useWindowDimension } from "../hooks/useWindowDimension.ts";
import { Button } from "../components/Button.tsx";
import { RangeInput } from "../components/RangeInput.tsx";
import { CanvasDrawContext } from "../src/adapters/canvasDrawContext.ts";
import { drawContextType } from "../src/ports/drawContext.ts";
import { GameRender } from "../src/features/gameRender/mod.ts";
import {
    GameModel,
    GameModelProxy,
    gameModelType,
} from "../src/features/gameModel/mod.ts";
import { GameController } from "../src/features/gameController/mod.ts";
import { GameManager } from "../src/features/gameManager/mod.ts";
import { fromString } from "../src/game/model/mod.ts";

export default function Canvas(): VNode {
    const gameModelProxyRef = useRef<GameModelProxy>(null);
    const gameControllerRef = useRef<GameController>(null);
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const dimension = useWindowDimension();

    const [model, setModel] = useState<gameModelType | null>(null);

    useEffect(() => {
        gameControllerRef.current?.setDimension(dimension);
    }, [dimension]);

    useEffect(() => {
        if (!canvasRef.current) {
            return;
        }
        const context = canvasRef.current.getContext("2d");
        if (!context) {
            return;
        }
        const drawContext = new CanvasDrawContext(context);
        const gameModel = new GameModel(
            {
                model: fromString([
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬜⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                    "⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛⬛",
                ]),
                dimension: 0,
                gap: 2,
                tiles: 20,
                fps: 1,
                status: "initial",
                drawContext,
            },
        );
        const gameModelProxy = new GameModelProxy(gameModel);
        const gameRender = new GameRender(gameModel, drawContext);
        const gameController = new GameController(gameModelProxy);
        const gameManager = new GameManager(
            gameModelProxy,
            gameController,
            gameRender,
        );
        gameController.setDimension(dimension);
        gameModelProxy.addOnChangeListener(() =>
            setModel(gameModelProxy.getModel())
        );
        setModel(gameModelProxy.getModel());
        gameModelProxyRef.current = gameModelProxy;
        gameControllerRef.current = gameController;
    }, []);

    function onClick(
        e: JSX.TargetedMouseEvent<HTMLCanvasElement>,
    ): void {
        const x = e.pageX - e.currentTarget.offsetLeft;
        const y = e.pageY - e.currentTarget.offsetTop;
    }

    function getStatusLabel(): string {
        return model?.status === "resumed" ? "pause" : "resume";
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
                        max={7}
                        step={1}
                        value={model ? model.gap : 0}
                        setValue={(gap) =>
                            gameControllerRef.current?.setGap(gap)}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="tiles">Tiles</label>
                    <RangeInput
                        id="tiles"
                        min={10}
                        max={100}
                        step={1}
                        value={model ? model.tiles : 0}
                        setValue={(tiles) =>
                            gameControllerRef.current?.setTiles(
                                tiles,
                            )}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="fps">fps</label>
                    <RangeInput
                        id="fps"
                        min={1}
                        max={10}
                        step={1}
                        value={model ? model.fps : 0}
                        setValue={(fps) =>
                            gameControllerRef.current?.setFps(fps)}
                    />
                </div>
                <span>
                    <label>{model ? model.model.iteration : 0}</label>
                    <label>Iteration</label>
                </span>
                <Button
                    label={getStatusLabel()}
                    onClick={() =>
                        gameControllerRef.current?.resume()}
                />
                <Button
                    label="iterate"
                    onClick={() =>
                        gameControllerRef.current?.singleIteration()}
                />
            </div>
        </>
    );
}
