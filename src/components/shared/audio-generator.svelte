<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { fade } from 'svelte/transition';
    import { audioContext } from './stores'; // Опционально, для глобального управления аудио

    export let initialText = '';
    export let buttonLabel = 'Generate Audio';
    export let voices = [
        { value: 'alloy', label: 'Alloy' },
        { value: 'echo', label: 'Echo' },
        { value: 'fable', label: 'Fable' },
        { value: 'onyx', label: 'Onyx' },
        { value: 'nova', label: 'Nova' },
        { value: 'shimmer', label: 'Shimmer' }
    ];
    export let selectedVoice = 'echo';

    let text = initialText;
    let audioUrl: string | null = null;
    let isLoading = false;
    let error: string | null = null;
    let audioElement: HTMLAudioElement;

    async function generateAudio() {
        if (!text.trim()) {
            error = 'Please enter some text';
            return;
        }

        isLoading = true;
        error = null;

        try {
            // Освобождаем предыдущий URL, если он существует
            if (audioUrl) {
                URL.revokeObjectURL(audioUrl);
                audioUrl = null;
            }

            // Вызываем Rust backend
            const audioData = await invoke<number[]>('generate_audio', {
                text,
                voice: selectedVoice
            });

            // Создаем Blob и URL для аудио
            const blob = new Blob([new Uint8Array(audioData)], { type: 'audio/mpeg' });
            audioUrl = URL.createObjectURL(blob);

            // Воспроизводим автоматически (опционально)
            setTimeout(() => {
                if (audioElement) audioElement.play().catch(e => console.error('Playback failed:', e));
            }, 100);
        } catch (err) {
            error = `Failed to generate audio: ${err}`;
            console.error(err);
        } finally {
            isLoading = false;
        }
    }

    function handleKeyDown(e: KeyboardEvent) {
        if (e.key === 'Enter' && e.ctrlKey) {
            generateAudio();
        }
    }

    $: if (initialText && initialText !== text) {
        text = initialText;
    }
</script>

<div class="audio-generator">
    <div class="input-group">
    <textarea
            bind:value={text}
            on:keydown={handleKeyDown}
            placeholder="Enter text to convert to speech..."
            rows={3}
            disabled={isLoading}
    />

        <div class="controls">
            <select bind:value={selectedVoice} disabled={isLoading}>
                {#each voices as voice}
                    <option value={voice.value}>{voice.label}</option>
                {/each}
            </select>

            <button
                    on:click={generateAudio}
                    disabled={isLoading || !text.trim()}
                    class:loading={isLoading}
            >
                {#if isLoading}
                    <span class="spinner" />
                {/if}
                {buttonLabel}
            </button>
        </div>
    </div>

    {#if error}
        <div transition:fade class="error-message">
            {error}
        </div>
    {/if}

    {#if audioUrl}
        <div transition:fade class="audio-player">
            <audio
                    bind:this={audioElement}
                    controls
                    src={audioUrl}
            />
            <div class="audio-tip">Press space to play/pause</div>
        </div>
    {/if}
</div>

<style>
    .audio-generator {
        width: 100%;
        max-width: 600px;
        margin: 0 auto;
        font-family: sans-serif;
    }

    .input-group {
        margin-bottom: 1rem;
    }

    textarea {
        width: 100%;
        padding: 0.75rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        resize: vertical;
        min-height: 100px;
        font-size: 1rem;
        margin-bottom: 0.5rem;
    }

    textarea:focus {
        outline: none;
        border-color: #646cff;
        box-shadow: 0 0 0 2px rgba(100, 108, 255, 0.2);
    }

    .controls {
        display: flex;
        gap: 0.5rem;
        align-items: center;
    }

    select {
        padding: 0.5rem;
        border-radius: 4px;
        border: 1px solid #ccc;
        background: white;
        flex-grow: 1;
    }

    button {
        padding: 0.5rem 1rem;
        background: #646cff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        transition: background 0.2s;
    }

    button:hover {
        background: #535bf2;
    }

    button:disabled {
        background: #ccc;
        cursor: not-allowed;
    }

    .loading {
        position: relative;
    }

    .spinner {
        width: 1rem;
        height: 1rem;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-radius: 50%;
        border-top-color: white;
        animation: spin 1s ease-in-out infinite;
    }

    @keyframes spin {
        to { transform: rotate(360deg); }
    }

    .error-message {
        color: #ff3e3e;
        background: #ffebeb;
        padding: 0.75rem;
        border-radius: 4px;
        margin-bottom: 1rem;
    }

    .audio-player {
        margin-top: 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    audio {
        width: 100%;
    }

    .audio-tip {
        font-size: 0.8rem;
        color: #666;
        text-align: center;
    }
</style>