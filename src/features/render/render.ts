import { dimensionType } from "../../core/dimension.ts";
import { stateType } from "../../game/cell/state.ts";
import { modelType } from "../../game/model/model.ts";
import { modelFns } from "../../game/model/modelFns.ts";
import { drawContextType } from "../../ports/drawContext.ts";

type renderParamsType = {
    model: modelType;
    drawContext: drawContextType;
    gap: number;
    size: number;
};

function render(
    { model, drawContext, size, gap }: renderParamsType,
): void {
    modelFns.forEach(model, ({ column, row }, state) => {
        drawContext.drawSquare({
            x: column * size + gap,
            y: row * size + gap,
            size: size - gap * 2,
        }, getSquareColor(state));
    });
}

function getSquareColor(state: stateType): string {
    switch (state) {
        case stateType.DEAD:
            return "#2e2e2e";
        case stateType.ALIVE:
            return "#dbdbdb";
    }
}

type renderManagerType = {
    model: modelType;
    drawContext: drawContextType;
    isPaused: boolean;
    interval: number;
    dimensions: dimensionType;
};

export class RenderManager {
    private timeoutId = 0;

    constructor() {
    }

    public onParamsChange(
        { model, drawContext, isPaused, interval, dimensions }:
            renderManagerType,
    ): void {
        const { iterate } = modelFns;

        if (this.timeoutId) {
            globalThis.clearInterval(this.timeoutId);
        }
        if (isPaused) return;
        this.timeoutId = globalThis.setInterval(() => {
            model = iterate(model);
            const dimensionSize = Math.min(
                dimensions.width,
                dimensions.height,
            );
            const modelSize = Math.min(model.width, model.height);
            const size = dimensionSize / modelSize;
            const gap = 1;

            render({ model, drawContext, size, gap });
        }, interval);
    }
}
