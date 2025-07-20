<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';

    let text = "";
    let voices = [
        { value: 'alloy', label: 'Alloy' },
        { value: 'echo', label: 'Echo' },
        { value: 'fable', label: 'Fable' },
        { value: 'onyx', label: 'Onyx' },
        { value: 'nova', label: 'Nova' },
        { value: 'shimmer', label: 'Shimmer' }
    ];
    let selectedVoice = 'echo';
    let audioUrl: string | null = null;
    let isLoading = false;
    let error = "";

    // Аудио визуализация
    let audioContext: AudioContext;
    let analyser: AnalyserNode;
    let dataArray: Uint8Array;
    let animationId: number;
    let bars = Array(16).fill(0);

    onMount(() => {
        audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
        analyser = audioContext.createAnalyser();
        analyser.fftSize = 64;
        dataArray = new Uint8Array(analyser.frequencyBinCount);

        return () => {
            cancelAnimationFrame(animationId);
            if (audioContext?.state !== 'closed') {
                audioContext?.close();
            }
        };
    });

    function visualize() {
        analyser.getByteFrequencyData(dataArray);

        const newBars = [];
        const step = Math.floor(dataArray.length / bars.length);

        for (let i = 0; i < bars.length; i++) {
            newBars.push(dataArray[i * step] / 255);
        }

        bars = newBars;
        animationId = requestAnimationFrame(visualize);
    }

    async function generate() {
        if (!text.trim()) {
            error = "Please enter text";
            return;
        }

        isLoading = true;
        error = "";

        try {
            if (audioUrl) URL.revokeObjectURL(audioUrl);

            const audioData = await invoke<number[]>('generate_audio', {
                text: text.trim(),
                voice: selectedVoice
            });

            const blob = new Blob([new Uint8Array(audioData)], { type: 'audio/mpeg' });
            audioUrl = URL.createObjectURL(blob);

            // Подключаем визуализацию
            const audioEl = new Audio(audioUrl);
            const source = audioContext.createMediaElementSource(audioEl);
            source.connect(analyser);
            analyser.connect(audioContext.destination);

            audioEl.play();
            visualize();

        } catch (err) {
            error = `Error: ${err}`;
            console.error(err);
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="compact-audio-gen">
    <div class="visualizer">
        {#each bars as bar, i}
            <div
                    class="bar"
                    style={`height: ${bar * 100}%; background: hsl(${200 + i * 10}, 100%, 50%)`}
            ></div>
        {/each}
    </div>

    <textarea
            bind:value={text}
            placeholder="Enter text..."
            rows={2}
            disabled={isLoading}
    ></textarea>

    <div class="controls">
        <select bind:value={selectedVoice} disabled={isLoading}>
            {#each voices as voice}
                <option value={voice.value}>{voice.label}</option>
            {/each}
        </select>

        <button on:click={generate} disabled={isLoading || !text.trim()}>
            {#if isLoading}
                <span class="spinner" ></span>
            {/if}
            Generate
        </button>
    </div>

    {#if error}
        <div class="error">{error}</div>
    {/if}
</div>

<style>
    .compact-audio-gen {
        width: 250px;
        padding: 10px;
        background: #2d3748;
        border-radius: 6px;
        box-shadow: 0 2px 10px rgba(0,0,0,0.2);
        font-family: sans-serif;
        color: white;
    }

    .visualizer {
        height: 60px;
        display: flex;
        align-items: flex-end;
        gap: 2px;
        margin-bottom: 10px;
        background: #1a202c;
        padding: 5px;
        border-radius: 4px;
    }

    .bar {
        flex: 1;
        min-height: 1px;
        border-radius: 2px;
        transition: height 0.1s ease-out;
    }

    textarea {
        width: 100%;
        padding: 8px;
        margin-bottom: 8px;
        background: #4a5568;
        border: none;
        border-radius: 4px;
        color: white;
        resize: none;
    }

    textarea:focus {
        outline: 1px solid #4299e1;
    }

    .controls {
        display: flex;
        gap: 8px;
    }

    select {
        flex: 1;
        padding: 6px;
        background: #4a5568;
        border: none;
        border-radius: 4px;
        color: white;
    }

    button {
        padding: 6px 12px;
        background: #4299e1;
        border: none;
        border-radius: 4px;
        color: white;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 6px;
    }

    button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .spinner {
        width: 12px;
        height: 12px;
        border: 2px solid rgba(255,255,255,0.3);
        border-radius: 50%;
        border-top-color: white;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        to { transform: rotate(360deg); }
    }

    .error {
        margin-top: 8px;
        color: #fc8181;
        font-size: 12px;
    }
</style>