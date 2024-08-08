import { State } from "../cell/mod.js";
import { Rectangle } from "../cartesianPlane/mod.js";

export type Model = {
    readonly value: Map<string, State.ALIVE>;
    readonly iteration: number;
    readonly position: Rectangle;
};
