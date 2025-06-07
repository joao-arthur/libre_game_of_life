<script lang="ts">
    import { onMount } from "svelte";
    import initWASM, { engineGetPresets, EngineStatus, } from "game_of_life_engine";
    import Button from "$lib/components/Button.svelte";
    import RangeInput from "$lib/components/RangeInput.svelte";
    import Select from "$lib/components/Select.svelte";

    let initiated = $state(false);
    let presets = $state([]);

    onMount(() => {
        if (!initiated) {
            initiated = true;
            initWASM().then(() => {
                presets = engineGetPresets();
            });
        }
    });

    function onClick(): void {}

    function handleZoomTo(size: number) {}

    function handleSetGap(gap: number) {}

    function handleSetFPS(fps: number) {}

    function handleSetPreset(preset: string) {}

    function handleToggle(): void {}

    function handleIterate() {}

    const model = {
        size: 500,
        fps: 60,
        gap: 0,
        preset: "block",
        age: 0n,
        status: EngineStatus.Paused,
    };
</script>

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
        onclick={onClick}
        width={300}
        height={300}
        style={`width: ${300}px; height: ${300}px;`}
    >
    </canvas>
    <div>
        <div class="form">
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
                <label class="input-value">{model ? model.gap : 0}</label>
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
                <label class="input-value">{model ? model.size : 0}</label>
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
                <label class="input-value">{model ? model.fps : 0}</label>
            </div>
        </div>
        <span>
            <label>Iteration: {model ? Number(model.age) : 0}</label>
        </span>
        <Button
            icon={model?.status === 0 ? "pause" : "play"}
            label={model?.status === 0 ? "PAUSE" : "RESUME"}
            onClick={handleToggle}
        />
        <Button icon="next" label="ITERATE" onClick={handleIterate} />
    </div>
</main>
