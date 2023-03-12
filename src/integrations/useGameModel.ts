import { useEffect, useRef, useState } from "preact/hooks";
import {
    GameModel,
    GameModelProxy,
    gameModelType,
} from "../features/gameModel/mod.ts";

type returnType = {
    readonly model: gameModelType;
    readonly gameModelProxy: GameModelProxy;
};

export function useGameModel(): returnType {
    const gameModel = useRef(new GameModel());
    const gameModelProxy = useRef(
        new GameModelProxy(gameModel.current),
    );
    const [model, setModel] = useState(
        gameModelProxy.current.getModel(),
    );

    useEffect(() => {
        gameModelProxy.current.addOnChangeListener(() => {
            setModel(gameModelProxy.current.getModel());
        });
    }, []);

    return {
        model,
        gameModelProxy: gameModelProxy.current,
    };
}
