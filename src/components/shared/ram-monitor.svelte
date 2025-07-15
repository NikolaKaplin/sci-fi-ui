<script lang="ts">
	import { listen } from '@tauri-apps/api/event'
	import { onMount } from 'svelte'

	let memory = {
		used: 0,
		total: 0,
		percentage: 0,
	}

	onMount(() => {
		const unlisten = listen<{
			used: number
			total: number
			percentage: number
		}>('memory_update', event => {
			memory = event.payload
		})

		return () => {
			unlisten.then(f => f())
		}
	})
</script>

<div class="memory-container">
	<div class="memory-text">{memory.percentage.toFixed(1)}%</div>
	<div class="memory-bar">
		<div class="memory-fill" style={`width: ${memory.percentage}%`}></div>
	</div>
	<div class="memory-details">
		{Math.round(memory.used / 1024 / 1024)} MB / {Math.round(
			memory.total / 1024 / 1024
		)} MB
	</div>
</div>

<style>
	.memory-container {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.memory-bar {
		height: 20px;
		background: #002200;
		border-radius: 3px;
		overflow: hidden;
		position: relative;
	}

	.memory-fill {
		height: 100%;
		background: linear-gradient(90deg, #0066ff, #00aaff);
		transition: width 0.3s ease;
	}

	.memory-text {
		font-size: 1.1em;
		color: #00aaff;
	}

	.memory-details {
		font-size: 0.9em;
		color: #00aa00;
	}
</style>
