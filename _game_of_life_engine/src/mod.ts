export type { ArrPos } from "./array.js";
export { arrPosFrom } from "./array.js";

export type { Point, Rect } from "./cartesianPlane.js";
export {
    absoluteToRelative,
    deserializePoint,
    indexToPoint,
    pointFrom,
    pointToIndex,
    rectFrom,
    relativeToAbsolute,
    serializePoint,
} from "./cartesianPlane.js";

export { iterateCell, State, toggleCell } from "./cell.js";

export type { AliveNeighbors } from "./neighbors.js";
export {
    aliveNeighborsFromModel,
    neighborsFromModel,
    numberOfAliveNeighbors,
} from "./neighbors.js";

export type { Model } from "./model.js";
export {
    getModelCellSize,
    getModelLength,
    getModelMiddleCell,
    getModelMiddlePoint,
    getModelValue,
    iterateModel,
    modelDefault,
    modelFromPos,
    modelFromString,
    moveModel,
    toggleModel,
    zoomModel,
} from "./model.js";

export type { Preset, PresetGroup, PresetSubGroup } from "./presets.js";
export { presetGroups } from "./presets.js";
