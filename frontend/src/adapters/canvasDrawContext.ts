import { squareType } from "../core/square";
import { drawContextType } from "../ports/drawContext";

export class CanvasDrawContext implements drawContextType {
    public constructor(
        private readonly context: CanvasRenderingContext2D,
    ) {}

    public clear(
        { x, y, size }: squareType,
    ): void {
        this.context.fillStyle = "white";
        this.context.fillRect(x, y, size, size);
    }

    public drawSquare(
        { x, y, size }: squareType,
        color: string,
    ): void {
        this.context.fillStyle = color;
        this.context.fillRect(x, y, size, size);
    }
}
