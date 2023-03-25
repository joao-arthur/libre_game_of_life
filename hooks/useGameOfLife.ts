import {
    useCallback,
    useEffect,
    useRef,
    useState,
} from "preact/hooks";
import { useWindowDimension } from "../hooks/useWindowDimension.ts";
import { CanvasDrawContext } from "../src/adapters/canvasDrawContext.ts";
import { buildModel } from "../src/features/gameModel/buildModel.ts";
import {
    GameController,
    GameManager,
    GameModel,
    gameModelType,
    GameRender,
} from "../src/features/mod.ts";

type gameOfLifeType = {
    readonly init: (canvasElement: HTMLCanvasElement) => void;
    readonly model: gameModelType | undefined;
    readonly controller: GameController | undefined;
};

export function useGameOfLife(): gameOfLifeType {
    const gameControllerRef = useRef<GameController | undefined>(
        undefined,
    );
    const dimension = useWindowDimension();

    const [model, setModel] = useState<gameModelType | undefined>(
        undefined,
    );

    useEffect(() => {
        gameControllerRef.current?.setDimension(dimension);
    }, [dimension]);

    const init = useCallback(
        (canvasElement: HTMLCanvasElement) => {
            const context = canvasElement.getContext("2d");
            if (!context) {
                return;
            }
            const drawContext = new CanvasDrawContext(context);
            const gameModel = new GameModel(
                buildModel(drawContext, dimension),
            );
            const gameRender = new GameRender(gameModel, drawContext);
            const gameController = new GameController(gameModel);
            const _gameManager = new GameManager(
                gameModel,
                gameController,
                gameRender,
            );
            gameModel.addOnChangeListener(() =>
                setModel(gameModel.getModel())
            );
            setModel(gameModel.getModel());
            gameControllerRef.current = gameController;
        },
        [],
    );

    return {
        init,
        model,
        controller: gameControllerRef.current,
    };
}
