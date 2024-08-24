import type { DrawContext, Square } from "./model.js";

export class CanvasDrawContext implements DrawContext {
    public constructor(private readonly context: CanvasRenderingContext2D) {}

    public clear(square: Square): void {
        this.context.fillStyle = "white";
        this.context.fillRect(square.x, square.y, square.size, square.size);
    }

    public drawSquare(square: Square, color: string): void {
        this.context.fillStyle = color;
        this.context.fillRect(square.x, square.y, square.size, square.size);
    }
}
