import { stateType } from "../cell/mod.js";
import { rectangleType } from "../cartesianPlane/mod.js";

/*{
    value: the alive cells position,
    iteration: the number of current generation,
    position: the ui position
}*/
export type modelType = {
    readonly value: Map<string, stateType.ALIVE>;
    readonly iteration: number;
    readonly position: rectangleType;
};
