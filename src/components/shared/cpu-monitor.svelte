<script lang="ts">
	import { listen } from '@tauri-apps/api/event'
	import { onMount } from 'svelte'

	let cpuUsage = 0
	let cpuHistory: number[] = Array(60).fill(0)
	let maxUsage = 100

	onMount(() => {
		const unlisten = listen<{ usage: number }>('cpu_update', event => {
			cpuUsage = event.payload.usage

			// Создаем новый массив вместо мутации старого
			cpuHistory = [...cpuHistory.slice(1), cpuUsage]

			// Явно обновляем maxUsage
			maxUsage = Math.max(100, ...cpuHistory) * 1.1

			// Принудительное обновление (альтернативный вариант)
			cpuHistory = cpuHistory
		})

		return () => unlisten.then(f => f())
	})
</script>

<div class="cpu-widget">
	<div class="header">
		<div class="title">CPU USAGE</div>
		<div class="value">{cpuUsage.toFixed(1)}%</div>
	</div>

	<div class="chart-container">
		<svg class="chart" viewBox="0 0 300 100" preserveAspectRatio="none">
			{#each cpuHistory as point, i}
				<svg class="chart" viewBox="0 0 300 100" preserveAspectRatio="none">
					<path
						d={`M0,100 ${cpuHistory.map((point, i) => `L${i * 5},${100 - (point / maxUsage) * 100}`).join(' ')}`}
						stroke="#00ff00"
						stroke-width="2"
						fill="none"
						stroke-linejoin="round"
					/>
				</svg>
			{/each}
		</svg>
		<div class="grid-lines"></div>
	</div>

	<div class="meter">
		<div class="bar" style={`width: ${cpuUsage}%`}>
			<div class="glow"></div>
		</div>
		<div class="ticks">
			{#each Array(11) as _, i}
				<div class="tick" style={`left: ${i * 10}%`}></div>
			{/each}
		</div>
	</div>
</div>

<style>
	.cpu-widget {
		background: rgba(0, 20, 0, 0.7);
		border: 1px solid rgba(0, 255, 0, 0.3);
		border-radius: 4px;
		padding: 12px;
		font-family: 'Courier New', monospace;
		color: #00ff00;
		text-shadow: 0 0 5px #00ff00;
		box-shadow: 0 0 15px rgba(0, 255, 0, 0.1);
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
		color: #00ff00;
	}

	.chart-container {
		height: 100px;
		background: rgba(0, 10, 0, 0.5);
		border: 1px solid rgba(0, 255, 0, 0.2);
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
				rgba(0, 255, 0, 0.05) 25%,
				rgba(0, 255, 0, 0.05) 26%,
				transparent 27%,
				transparent 49%,
				rgba(0, 255, 0, 0.05) 50%,
				rgba(0, 255, 0, 0.05) 51%,
				transparent 52%,
				transparent 74%,
				rgba(0, 255, 0, 0.05) 75%,
				rgba(0, 255, 0, 0.05) 76%,
				transparent 77%
			),
			linear-gradient(to right, transparent 90%, rgba(0, 255, 0, 0.05) 91%);
		pointer-events: none;
	}

	.meter {
		height: 16px;
		background: rgba(0, 20, 0, 0.5);
		border: 1px solid rgba(0, 255, 0, 0.3);
		border-radius: 2px;
		position: relative;
		overflow: hidden;
	}

	.bar {
		height: 100%;
		background: linear-gradient(90deg, #003300, #00aa00, #00ff00);
		position: relative;
		transition: width 0.3s ease-out;
	}

	.glow {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 20px;
		background: linear-gradient(90deg, rgba(0, 255, 0, 0.5), transparent);
		animation: pulse 1.5s infinite;
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
		background: rgba(0, 255, 0, 0.2);
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
</style>
