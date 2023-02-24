import { dimensionType } from "../core/dimension.ts";
import { drawContextType } from "./drawContext.ts";

export type renderContext = {
    drawContext: drawContextType;
    dimensions: dimensionType;
    isPaused: boolean;
};
