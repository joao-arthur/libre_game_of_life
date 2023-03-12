import { useEffect, useRef } from "preact/hooks";
import { JSX, VNode } from "preact";
import { useWindowDimensions } from "../hooks/useWindowDimensions.ts";
import { Button } from "../components/Button.tsx";
import { RangeInput } from "../components/RangeInput.tsx";
import { CanvasDrawContext } from "../src/adapters/canvasDrawContext.ts";
import { drawContextType } from "../src/ports/drawContext.ts";
import { GameRender } from "../src/features/gameRender/mod.ts";
import { useGameModel } from "../src/integrations/useGameModel.ts";

let drawContext: drawContextType;
let gameRender: GameRender;

export default function Canvas(): VNode {
    const dimensions = useWindowDimensions();
    const dimension = Math.min(dimensions.height, dimensions.width);
    const canvasRef = useRef<HTMLCanvasElement>(null);

    const {
        model,
        actions,
        gameModelProxy,
    } = useGameModel();

    useEffect(() => {
        actions.setDimensions(dimensions);
    }, [dimensions]);

    useEffect(() => {
        if (!canvasRef.current) {
            return;
        }
        const context = canvasRef.current.getContext("2d");
        if (!context) {
            return;
        }
        drawContext = new CanvasDrawContext(context);
        gameRender = new GameRender(gameModelProxy, drawContext);
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
                        max={10}
                        step={1}
                        value={model.gap}
                        setValue={actions.setGap}
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
                        setValue={actions.setSize}
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
                        setValue={actions.setFps}
                    />
                </div>
                <span>
                    <label>{model.model.iteration}</label>
                    <label>Iteration</label>
                </span>
                {model.status === "resumed"
                    ? <Button label="pause" onClick={actions.pause} />
                    : (
                        <Button
                            label="resume"
                            onClick={actions.resume}
                        />
                    )}
                <Button
                    label="iterate"
                    onClick={actions.singleIteration}
                />
            </div>
        </>
    );
}
