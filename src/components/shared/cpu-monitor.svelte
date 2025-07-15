<script lang="ts">
	import { listen } from '@tauri-apps/api/event'
	import { onMount } from 'svelte'

	let cpuUsage = 0

	onMount(() => {
		const unlisten = listen<{ usage: number }>('cpu_update', event => {
			cpuUsage = event.payload.usage
		})

		return () => {
			unlisten.then(f => f())
		}
	})
</script>

<div class="cpu-container">
	<div class="cpu-text">{cpuUsage.toFixed(1)}%</div>
	<div class="cpu-bar">
		<div class="cpu-fill" style={`width: ${cpuUsage}%`}></div>
	</div>
</div>

<style>
	.cpu-container {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.cpu-bar {
		height: 20px;
		background: #002200;
		border-radius: 3px;
		overflow: hidden;
		position: relative;
	}

	.cpu-fill {
		height: 100%;
		background: linear-gradient(90deg, #00aa00, #00ff00);
		transition: width 0.3s ease;
	}

	.cpu-text {
		font-size: 1.5em;
		text-align: center;
		color: #00ff00;
	}
</style>
