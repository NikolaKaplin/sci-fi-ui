<script lang="ts">
	import { listen } from '@tauri-apps/api/event'
	import { onMount } from 'svelte'

	let memory = {
		used: 0,
		total: 0,
		percentage: 0,
	}
	let memoryHistory: number[] = Array(60).fill(0)
	let maxMemory = 16 // GB (автоматически обновится)

	onMount(() => {
		const unlisten = listen<{
			used: number
			total: number
			percentage: number
		}>('memory_update', event => {
			memory = event.payload
			// Обновляем историю
			memoryHistory = [...memoryHistory.slice(1), memory.percentage]
			// Обновляем максимум для масштабирования (в GB)
			maxMemory = Math.max(
				16,
				Math.ceil(memory.total / (1024 * 1024 * 1024)) * 1.2
			)
		})

		return () => unlisten.then(f => f())
	})
</script>

<div class="ram-widget">
	<div class="header">
		<div class="title">RAM USAGE</div>
		<div class="value">{memory.percentage.toFixed(1)}%</div>
	</div>

	<div class="chart-container">
		<svg class="chart" viewBox="0 0 300 100" preserveAspectRatio="none">
			<path
				d={`M0,100 ${memoryHistory.map((point, i) => `L${i * 5},${100 - point}`).join(' ')}`}
				stroke="#00aaff"
				stroke-width="2"
				fill="none"
				stroke-linejoin="round"
			/>
		</svg>
		<div class="grid-lines"></div>
	</div>

	<div class="meter">
		<div class="bar" style={`width: ${memory.percentage}%`}>
			<div class="glow"></div>
			<div class="pulse"></div>
		</div>
		<div class="ticks">
			{#each Array(11) as _, i}
				<div class="tick" style={`left: ${i * 10}%`}></div>
			{/each}
		</div>
	</div>

	<div class="details">
		<div class="used">
			{Math.round(memory.used / (1024 * 1024 * 1024 * 0.1) / 10)} GB
		</div>
		<div class="total">
			{Math.round(memory.total / (1024 * 1024 * 1024 * 0.1) / 10)} GB
		</div>
	</div>

	<div class="memory-map">
		{#each Array(20) as _, i}
			<div
				class="memory-block"
				class:used={i < memory.percentage / 5}
				style={`--delay: ${i * 0.05}s`}
			></div>
		{/each}
	</div>
</div>

<style>
	.ram-widget {
		background: rgba(0, 10, 20, 0.7);
		border: 1px solid rgba(0, 170, 255, 0.3);
		border-radius: 4px;
		padding: 12px;
		font-family: 'Courier New', monospace;
		color: #00aaff;
		text-shadow: 0 0 5px rgba(0, 170, 255, 0.5);
		box-shadow: 0 0 15px rgba(0, 170, 255, 0.1);
		width: 320px;
	}

	.header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 10px;
		font-size: 1.1em;
	}

	.title {
		font-weight: bold;
		opacity: 0.8;
	}

	.value {
		font-weight: bold;
		color: #00aaff;
	}

	.chart-container {
		height: 100px;
		background: rgba(0, 5, 10, 0.5);
		border: 1px solid rgba(0, 170, 255, 0.2);
		position: relative;
		margin-bottom: 12px;
		overflow: hidden;
	}

	.chart {
		width: 100%;
		height: 100%;
	}

	.grid-lines {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: linear-gradient(
				to bottom,
				transparent 24%,
				rgba(0, 170, 255, 0.05) 25%,
				rgba(0, 170, 255, 0.05) 26%,
				transparent 27%,
				transparent 49%,
				rgba(0, 170, 255, 0.05) 50%,
				rgba(0, 170, 255, 0.05) 51%,
				transparent 52%,
				transparent 74%,
				rgba(0, 170, 255, 0.05) 75%,
				rgba(0, 170, 255, 0.05) 76%,
				transparent 77%
			),
			linear-gradient(to right, transparent 90%, rgba(0, 170, 255, 0.05) 91%);
		pointer-events: none;
	}

	.meter {
		height: 16px;
		background: rgba(0, 10, 20, 0.5);
		border: 1px solid rgba(0, 170, 255, 0.3);
		border-radius: 2px;
		position: relative;
		overflow: hidden;
		margin-bottom: 10px;
	}

	.bar {
		height: 100%;
		background: linear-gradient(90deg, #003366, #0066aa, #00aaff);
		position: relative;
		transition: width 0.3s ease-out;
	}

	.glow {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 20px;
		background: linear-gradient(90deg, rgba(0, 170, 255, 0.5), transparent);
		animation: pulse 1.5s infinite;
	}

	.pulse {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 100%;
		background: rgba(255, 255, 255, 0.1);
		animation: pulse-opacity 2s infinite;
	}

	.ticks {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
	}

	.tick {
		position: absolute;
		top: 0;
		width: 1px;
		height: 100%;
		background: rgba(0, 170, 255, 0.2);
	}

	.details {
		display: flex;
		justify-content: space-between;
		font-size: 0.9em;
		margin-bottom: 10px;
	}

	.used {
		color: #00aaff;
		font-weight: bold;
	}

	.total {
		color: #00aa00;
		opacity: 0.8;
	}

	.memory-map {
		display: grid;
		grid-template-columns: repeat(10, 1fr);
		gap: 4px;
	}

	.memory-block {
		height: 10px;
		background: rgba(0, 50, 100, 0.3);
		border: 1px solid rgba(0, 170, 255, 0.1);
		transition: all 0.3s ease;
		transition-delay: var(--delay);
	}

	.memory-block.used {
		background: linear-gradient(to bottom, #00aaff, #0066ff);
		box-shadow: 0 0 5px #00aaff;
		animation: block-pulse 2s infinite;
		animation-delay: var(--delay);
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 0.8;
		}
		50% {
			opacity: 0.3;
		}
	}

	@keyframes pulse-opacity {
		0%,
		100% {
			opacity: 0;
		}
		50% {
			opacity: 0.2;
		}
	}

	@keyframes block-pulse {
		0%,
		100% {
			opacity: 0.8;
		}
		50% {
			opacity: 1;
		}
	}
</style>
