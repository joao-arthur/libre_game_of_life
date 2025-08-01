<script lang="ts">
    import { onMount } from "svelte";
    import initWASM, {
        engineAddOnChangeListener,
        EngineCartesianPoint,
        engineGetPresets,
        engineGetSettings,
        EngineInfo,
        engineInit,
        EngineMatrixPoint,
        engineMoveBy,
        enginePause,
        engineResume,
        engineSetDimension,
        engineSetFPS,
        engineSetGap,
        engineSetPreset,
        engineSingleIteration,
        EngineStatus,
        engineToggle,
        engineZoomIn,
        engineZoomOut,
        engineZoomTo,
    } from "game_of_life_engine";
    import Button from "$lib/components/Button.svelte";
    import RangeInput from "$lib/components/RangeInput.svelte";
    import Select from "$lib/components/Select.svelte";

    let initiated = $state(false);
    let presets = $state([]);
    let model = $state({
        size: 0,
        fps: 60,
        gap: 0,
        preset: "block",
        age: 0n,
        status: EngineStatus.Paused,
    } as EngineInfo);

    let innerWidth = $state(0);
    let innerHeight = $state(0);

    let canvas: HTMLCanvasElement;

    onMount(() => {
        if (!initiated) {
            initWASM().then(() => {
                initiated = true;
                let context = canvas.getContext("2d");
                if (!context) {
                    return;
                }
                engineAddOnChangeListener(() => {
                    model = engineGetSettings();
                });
                engineInit(context);
                presets = engineGetPresets();
                engineSetDimension(Math.min(innerWidth, innerHeight));
            });
        }
    });

    function onClick(
        event: MouseEvent & { currentTarget: EventTarget & HTMLCanvasElement },
    ): void {
        const col = event.pageX - event.currentTarget.offsetLeft;
        const row = event.pageY - event.currentTarget.offsetTop;
        engineToggle(new EngineMatrixPoint(row, col));
    }

    function handleZoomTo(size: number) {
        engineZoomTo(size);
    }

    function handleSetGap(gap: number) {
        engineSetGap(gap);
    }

    function handleSetFPS(fps: number) {
        engineSetFPS(fps);
    }

    function handleSetPreset(preset: string) {
        engineSetPreset(preset);
    }

    function handleToggle(): void {
        if (!model) return;
        switch (model.status) {
            case EngineStatus.Resumed:
                enginePause();
                break;
            case EngineStatus.Paused:
                engineResume();
                break;
        }
    }

    function handleIterate() {
        engineSingleIteration();
    }

    function onKeyPress(e: KeyboardEvent) {
        switch (e.key) {
            case "w":
                engineMoveBy(new EngineCartesianPoint(0, 1));
                break;
            case "a":
                engineMoveBy(new EngineCartesianPoint(-1, 0));
                break;
            case "s":
                engineMoveBy(new EngineCartesianPoint(0, -1));
                break;
            case "d":
                engineMoveBy(new EngineCartesianPoint(1, 0));
                break;
            case "+":
                engineZoomIn();
                break;
            case "-":
                engineZoomOut();
                break;
        }
    }
</script>

<svelte:window onkeypress={onKeyPress} bind:innerWidth bind:innerHeight />

<style>
    main {
        width: 100vw;
        height: 100vh;
        display: flex;
    }

    canvas {
        margin: auto;
    }

    .form {
        display: flex;
        flex-direction: column;
        row-gap: 1rem;
    }

    .field-container {
        display: flex;
        flex-direction: column;
    }

    .input-container {
        display: flex;
    }

    .input-value {
        width: 4rem;
        text-align: center;
        display: block;
    }
</style>

<main>
    <canvas
        bind:this={canvas}
        onkeypress={onKeyPress}
        onclick={onClick}
        width={Math.min(innerHeight, innerWidth)}
        height={Math.min(innerHeight, innerWidth)}
        style={`width: ${Math.min(innerHeight, innerWidth)}px; height: ${
            Math.min(innerHeight, innerWidth)
        }px;`}
    >
    </canvas>
    <div class="form">
        <div class="field-container">
            <label for="preset">Preset</label>
            <Select
                id="preset"
                groups={presets.map((group: any) => ({
                    label: group.info.name,
                    value: group.info.id,
                    options: group.items.map((item: any) => ({
                        label: item.name,
                        value: item.id,
                    })),
                }))}
                value={model?.preset || ""}
                onChange={handleSetPreset}
            />
        </div>
        <div class="field-container">
            <label for="gap">Gap</label>
            <div class="input-container">
                <RangeInput
                    id="gap"
                    min={0}
                    max={2}
                    step={1}
                    value={model ? model.gap : 0}
                    onChange={handleSetGap}
                />
                <label for="gap" class="input-value">{model ? model.gap : 0}</label>
            </div>
        </div>
        <div class="field-container">
            <label for="size">Size</label>
            <div class="input-container">
                <RangeInput
                    id="size"
                    min={2 + (model ? model.size % 2 === 0 ? 0 : 1 : 0)}
                    max={200 + (model ? model.size % 2 === 0 ? 0 : 1 : 0)}
                    step={2}
                    value={model ? model.size : 0}
                    onChange={handleZoomTo}
                />
                <label for="size" class="input-value">{model ? model.size : 0}</label>
            </div>
        </div>
        <div class="field-container">
            <label for="fps">FPS</label>
            <div class="input-container">
                <RangeInput
                    id="fps"
                    min={1}
                    max={60}
                    step={1}
                    value={model ? model.fps : 0}
                    onChange={handleSetFPS}
                />
                <label for="fps" class="input-value">{model ? model.fps : 0}</label>
            </div>
        </div>
        <span>
            <span>Iteration: {model ? Number(model.age) : 0}</span>
        </span>
        <Button
            icon={model?.status === 0 ? "pause" : "play"}
            label={model?.status === 0 ? "PAUSE" : "RESUME"}
            onClick={handleToggle}
        />
        <Button icon="next" label="ITERATE" onClick={handleIterate} />
    </div>
</main>
