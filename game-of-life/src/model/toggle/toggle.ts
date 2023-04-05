import { cellFns } from "../../cell/mod.js";
import { positionType } from "../position.js";
import { modelType } from "../model.js";
import { map } from "../map/mod.js";

export function toggle(
    model: modelType,
    position: positionType,
): modelType {
    return map(
        model,
        ({ column, row }, current) =>
            column === position.column && row === position.row
                ? cellFns.toggle(current)
                : current,
    );
}
