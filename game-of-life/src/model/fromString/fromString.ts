import { stateType } from "../../cell/mod.ts";
import { modelType } from "../model.ts";

export function fromString(stringValue: string[]): modelType {
    return {
        size: stringValue[0].length,
        value: stringValue.map((line) =>
            line.split("").map((character: string): stateType => {
                switch (character) {
                    case "⬛":
                        return stateType.DEAD;
                    case "⬜":
                        return stateType.ALIVE;
                    default:
                        return undefined!;
                }
            })
        ),
        iteration: 0,
    };
}
