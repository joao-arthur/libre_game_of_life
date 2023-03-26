import { cellFns } from "../../cell/mod.ts";
import { positionType } from "../position.ts";
import { modelType } from "../model.ts";
import { map } from "../map/mod.ts";

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
