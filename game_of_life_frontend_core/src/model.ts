import { map } from "funis";
import {
    getModelCellSize,
    getModelLength,
    getModelMiddleCell,
    getPresetGroups,
    iterateModel,
    moveModel,
    Point,
    pointToIndex,
    toggleModel,
    zoomModel,
} from "game_of_life_engine";

export type ArrPos = {
    readonly row: number;
    readonly col: number;
};

export type Rect = {
    readonly x1: number;
    readonly y1: number;
    readonly x2: number;
    readonly y2: number;
};

export type _Point = {
    readonly x: number;
    readonly y: number;
};

export enum State {
    DEAD,
    ALIVE,
}

export type Model = {
    readonly value: Map<_Point, State>;
    readonly iter: number;
    readonly pos: Rect;
};

export type Square = {
    readonly x: number;
    readonly y: number;
    readonly size: number;
};

export type Preset = {
    readonly name: string;
    readonly id: string;
    readonly discover: {
        readonly name: string;
        readonly year: number;
    };
    readonly model: Model;
};

export type PresetSubGroup = {
    readonly name: string;
    readonly id: string;
    readonly items: readonly Preset[];
};

export type PresetGroup = {
    readonly group: {
        readonly name: string;
        readonly id: string;
    };
    readonly subGroups: readonly PresetSubGroup[];
};

export type DrawContext = {
    readonly clear: (square: Square) => void;
    readonly drawSquare: (square: Square, color: string) => void;
};

export type SystemModel = {
    readonly model: Model;
    readonly gap: number;
    readonly fps: number;
    readonly status: "resumed" | "paused";
    readonly dimension: bigint;
    readonly drawContext: DrawContext;
};

export function buildModel(drawContext: DrawContext, dimension: bigint): SystemModel {
    return {
        model: {
            iter: 0,
            pos: { x1: -10, x2: -10, y1: 10, y2: 10 },
            value: new Map(),
        },
        dimension,
        gap: 1,
        fps: 4,
        status: "paused",
        drawContext,
    };
}

type PresetOptions = {
    readonly label: string;
    readonly value: string;
    readonly options: {
        readonly label: string;
        readonly value: string;
    }[];
}[];

export function buildPresetsOptions(): PresetOptions {
    return (getPresetGroups() as any).map((group: any) => ({
        label: group.group.name,
        value: group.group.id,
        options: group.sub_groups
            .flatMap((subGroup: any) => subGroup.items)
            .map((item: any) => ({
                label: item.name,
                value: item.id,
            })),
    }));
}

export function buildPresets(): readonly Preset[] {
    return (getPresetGroups() as any).flatMap((group: any) =>
        group.sub_groups.flatMap((subGroup: any) => subGroup.items)
    );
}

export function fpsToMilis(fps: number): number {
    return 1000 / fps;
}

const DEAD_COLOR = "#dbdbdb";
const ALIVE_COLOR = "#2e2e2e";

function render(systemModel: SystemModelProxy): void {
    const model = systemModel.getModel();
    const length = getModelLength({ Ok: model.model });
    const cellSize = getModelCellSize({ Ok: model.model }, model.dimension);
    const middleCell = getModelMiddleCell({ Ok: model.model }, model.dimension);

    const dim = { x: 0, y: 0, size: Number(model.dimension) };
    model.drawContext.drawSquare(dim, DEAD_COLOR);

    map.keys(model.model.value)
        .forEach((point) => {
            const { col, row } = pointToIndex(new Point(BigInt(point.x), BigInt(point.y)), length);
            model.drawContext.drawSquare({
                x: Number(col) * Number(cellSize) + model.gap - Number(middleCell.x),
                y: Number(row) * Number(cellSize) + model.gap + Number(middleCell.y),
                size: Number(cellSize) - model.gap * 2,
            }, ALIVE_COLOR);
        });
}

type onModelChange = (param: keyof SystemModel) => void;

export class SystemModelProxy {
    private readonly onChangeListeners: (onModelChange)[] = [];
    private model: Model;
    private gap: number;
    private fps: number;
    private status: "resumed" | "paused";
    private dimension: bigint;
    private drawContext: DrawContext;

    constructor(systemModel: SystemModel) {
        this.model = systemModel.model;
        this.gap = systemModel.gap;
        this.fps = systemModel.fps;
        this.status = systemModel.status;
        this.dimension = systemModel.dimension;
        this.drawContext = systemModel.drawContext;
    }

    public setModel(model: SystemModel["model"]): void {
        this.model = model;
        this.onChange("model");
    }

    public setGap(gap: SystemModel["gap"]): void {
        this.gap = gap;
        this.onChange("gap");
    }

    public setFps(fps: SystemModel["fps"]): void {
        this.fps = fps;
        this.onChange("fps");
    }

    public setStatus(status: SystemModel["status"]): void {
        this.status = status;
        this.onChange("status");
    }

    public setDimension(dimension: SystemModel["dimension"]): void {
        this.dimension = dimension;
        this.onChange("dimension");
    }

    public setDrawContext(drawContext: SystemModel["drawContext"]): void {
        this.drawContext = drawContext;
        this.onChange("drawContext");
    }

    public getModel(): SystemModel {
        return {
            model: this.model,
            gap: this.gap,
            fps: this.fps,
            status: this.status,
            dimension: this.dimension,
            drawContext: this.drawContext,
        };
    }

    public addOnChangeListener(cb: onModelChange): void {
        this.onChangeListeners.push(cb);
    }

    private onChange(param: keyof SystemModel): void {
        this.onChangeListeners.forEach((cb) => cb(param));
    }
}

export class SystemController {
    constructor(private readonly systemModel: SystemModelProxy) {}

    public pause(): void {
        this.systemModel.setStatus("paused");
    }

    public resume(): void {
        this.systemModel.setStatus("resumed");
    }

    public singleIteration(): void {
        this.systemModel.setStatus("paused");
        this.systemModel.setModel(iterateModel({ Ok: this.systemModel.getModel().model }));
    }

    public iterate(): void {
        this.systemModel.setModel(iterateModel({ Ok: this.systemModel.getModel().model }));
    }

    public toggleCell(point: _Point): void {
        this.systemModel.setModel(
            toggleModel({ Ok: this.systemModel.getModel().model }, new Point(BigInt(point.x), BigInt(point.y))),
        );
    }

    public setPreset(preset: string): void {
        const selectedPreset = buildPresets().find(({ id }) => id === preset);
        if (selectedPreset === undefined) {
            return;
        }
        this.systemModel.setModel(selectedPreset.model);
    }

    public setDimension(dimension: SystemModel["dimension"]): void {
        this.systemModel.setDimension(dimension);
    }

    public setGap(gap: SystemModel["gap"]): void {
        this.systemModel.setGap(gap);
    }

    public setSize(newSize: number): void {
        this.systemModel.setModel(
            zoomModel({ Ok: this.systemModel.getModel().model }, BigInt(newSize)),
        );
    }

    public move(delta: _Point): void {
        this.systemModel.setModel(
            moveModel(
                { Ok: this.systemModel.getModel().model },
                new Point(BigInt(delta.x), BigInt(delta.y)),
            ),
        );
    }

    public setFps(fps: SystemModel["fps"]): void {
        this.systemModel.setFps(fps);
    }
}

let timeoutId = 0;

export function manage(systemModel: SystemModelProxy, systemController: SystemController): void {
    systemModel.addOnChangeListener((prop) => {
        const model = systemModel.getModel();
        switch (model.status) {
            case "resumed":
                switch (prop) {
                    case "status":
                    case "fps":
                        window.clearInterval(timeoutId);
                        timeoutId = window.setInterval(
                            () => {
                                systemController.iterate();
                                render(systemModel);
                            },
                            fpsToMilis(model.fps),
                        );
                }
                break;
            case "paused":
                switch (prop) {
                    case "gap":
                    case "dimension":
                    case "model":
                        render(systemModel);
                        break;
                    case "status":
                        window.clearInterval(timeoutId);
                }
        }
    });
    render(systemModel);
    systemController.pause();
}
