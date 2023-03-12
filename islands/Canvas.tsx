import { useEffect, useRef } from "preact/hooks";
import { JSX, VNode } from "preact";
import { useWindowDimension } from "../hooks/useWindowDimension.ts";
import { Button } from "../components/Button.tsx";
import { RangeInput } from "../components/RangeInput.tsx";
import { CanvasDrawContext } from "../src/adapters/canvasDrawContext.ts";
import { drawContextType } from "../src/ports/drawContext.ts";
import { GameRender } from "../src/features/gameRender/mod.ts";
import { useGameModel } from "../src/integrations/useGameModel.ts";

export default function Canvas(): VNode {
    const drawContext = useRef<drawContextType>(null);
    const gameRender = useRef<GameRender>(null);
    const dimension = useWindowDimension();
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const { model, gameModelProxy } = useGameModel();

    useEffect(() => {
        gameModelProxy.setDimension(dimension);
    }, [dimension]);

    useEffect(() => {
        if (!canvasRef.current) {
            return;
        }
        const context = canvasRef.current.getContext("2d");
        if (!context) {
            return;
        }
        drawContext.current = new CanvasDrawContext(context);
        gameRender.current = new GameRender(
            gameModelProxy,
            drawContext.current,
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
                        max={7}
                        step={1}
                        value={model.gap}
                        setValue={(gap) => gameModelProxy.setGap(gap)}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="size">Size</label>
                    <RangeInput
                        id="size"
                        min={10}
                        max={100}
                        step={1}
                        value={model.size}
                        setValue={(size) =>
                            gameModelProxy.setSize(size)}
                    />
                </div>
                <div className="flex flex-col">
                    <label for="fps">fps</label>
                    <RangeInput
                        id="fps"
                        min={1}
                        max={10}
                        step={1}
                        value={model.fps}
                        setValue={(fps) => gameModelProxy.setFps(fps)}
                    />
                </div>
                <span>
                    <label>{model.model.iteration}</label>
                    <label>Iteration</label>
                </span>
                {model.status === "resumed"
                    ? (
                        <Button
                            label="pause"
                            onClick={() => gameModelProxy.pause()}
                        />
                    )
                    : (
                        <Button
                            label="resume"
                            onClick={() => gameModelProxy.resume()}
                        />
                    )}
                <Button
                    label="iterate"
                    onClick={() => gameModelProxy.singleIteration()}
                />
            </div>
        </>
    );
}
