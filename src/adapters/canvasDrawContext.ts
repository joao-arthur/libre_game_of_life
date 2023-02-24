import { squareType } from "../core/square.ts";
import { drawContextType } from "../ports/drawContext.ts";

export class CanvasDrawContext implements drawContextType {
    public constructor(
        private readonly context: CanvasRenderingContext2D,
    ) {}

    public drawSquare(
        { x, y, size }: squareType,
        color: string,
    ): void {
        this.context.fillStyle = color;
        this.context.fillRect(x, y, size, size);
    }
}
