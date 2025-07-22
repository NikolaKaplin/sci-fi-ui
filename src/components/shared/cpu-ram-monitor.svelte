<script lang="ts">
	import { listen } from '@tauri-apps/api/event'
	import { onMount } from 'svelte'

	let cpuUsage = 0
	let cpuHistory: number[] = Array(60).fill(0)
	let cpuMaxUsage = 100

	let memory = {
		used: 0,
		total: 0,
		percentage: 0,
	}
	let ramHistory: number[] = Array(60).fill(0)
	let ramMaxUsage = 16 // GB

	// Цвета темы
	const colors = {
		cpu: '#00ff00',
		ram: '#00aaff',
		grid: 'rgba(255, 255, 255, 0.05)',
	}

	onMount(() => {
		const cpuUnlisten = listen<{ usage: number }>('cpu_update', event => {
			cpuUsage = event.payload.usage
			cpuHistory = [...cpuHistory.slice(1), cpuUsage]
			cpuMaxUsage = Math.max(100, ...cpuHistory) * 1.1
		})

		const ramUnlisten = listen<{
			used: number
			total: number
			percentage: number
		}>('memory_update', event => {
			memory = event.payload
			ramHistory = [...ramHistory.slice(1), memory.percentage]
			ramMaxUsage = Math.max(
				16,
				Math.ceil(memory.total / (1024 * 1024 * 1024)) * 1.2
			)
		})

		return () => {
			cpuUnlisten.then(f => f())
			ramUnlisten.then(f => f())
		}
	})

	function formatMemory(bytes: number): string {
		const gb = bytes / (1024 * 1024 * 1024)
		return gb.toFixed(gb < 1 ? 2 : 1)
	}
</script>

<div class="system-monitor">
	<div class="section cpu-section">
		<div class="header">
			<span class="title">CPU</span>
			<span class="value" style="color: {colors.cpu}"
				>{cpuUsage.toFixed(1)}%</span
			>
		</div>

		<div class="chart-container">
			<svg viewBox="0 0 100 50" preserveAspectRatio="none">
				<path
					d={`M0,50 ${cpuHistory.map((point, i) => `L${(i * 100) / (cpuHistory.length - 1)},${50 - (point / cpuMaxUsage) * 50}`).join(' ')}`}
					stroke={colors.cpu}
					stroke-width="0.8"
					fill="none"
					stroke-linejoin="round"
				/>
			</svg>
			<div
				class="grid"
				style={`background:
        linear-gradient(to bottom, transparent 24%, ${colors.grid} 25%, ${colors.grid} 26%, transparent 27%, transparent 49%, ${colors.grid} 50%, ${colors.grid} 51%, transparent 52%),
        linear-gradient(to right, transparent 90%, ${colors.grid} 91%)`}
			></div>
		</div>

		<div class="meter">
			<div class="bar" style={`width: ${cpuUsage}%; background: ${colors.cpu}`}>
				<div
					class="glow"
					style={`background: linear-gradient(90deg, ${colors.cpu}, transparent)`}
				></div>
			</div>
			<div class="ticks">
				{#each Array(6) as _, i}
					<div class="tick" style={`left: ${i * 20}%`}></div>
				{/each}
			</div>
		</div>
	</div>

	<div class="section ram-section">
		<div class="header">
			<span class="title">RAM</span>
			<span class="value" style="color: {colors.ram}"
				>{memory.percentage.toFixed(1)}%</span
			>
		</div>

		<div class="chart-container">
			<svg viewBox="0 0 100 50" preserveAspectRatio="none">
				<path
					d={`M0,50 ${ramHistory.map((point, i) => `L${(i * 100) / (ramHistory.length - 1)},${50 - point * 0.5}`).join(' ')}`}
					stroke={colors.ram}
					stroke-width="0.8"
					fill="none"
					stroke-linejoin="round"
				/>
			</svg>
			<div
				class="grid"
				style={`background:
        linear-gradient(to bottom, transparent 24%, ${colors.grid} 25%, ${colors.grid} 26%, transparent 27%, transparent 49%, ${colors.grid} 50%, ${colors.grid} 51%, transparent 52%),
        linear-gradient(to right, transparent 90%, ${colors.grid} 91%)`}
			></div>
		</div>

		<div class="meter">
			<div
				class="bar"
				style={`width: ${memory.percentage}%; background: ${colors.ram}`}
			>
				<div
					class="glow"
					style={`background: linear-gradient(90deg, ${colors.ram}, transparent)`}
				></div>
			</div>
			<div class="ticks">
				{#each Array(6) as _, i}
					<div class="tick" style={`left: ${i * 20}%`}></div>
				{/each}
			</div>
		</div>

		<div class="memory-info">
			<span class="used" style="color: {colors.ram}"
				>{formatMemory(memory.used)}</span
			>
			<span class="separator">/</span>
			<span class="total">{formatMemory(memory.total)} GB</span>
		</div>
	</div>
</div>

<style>
	.system-monitor {
		padding: 10px;
		font-family: 'Courier New', monospace;
		font-size: 12px;
		color: #fff;
		text-shadow: 0 0 5px currentColor;
		width: 250px;
	}

	.section {
		margin-bottom: 12px;
	}

	.section:last-child {
		margin-bottom: 0;
	}

	.header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 8px;
	}

	.title {
		font-weight: bold;
		opacity: 0.8;
	}

	.value {
		font-weight: bold;
	}

	.chart-container {
		height: 50px;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 3px;
		margin-bottom: 8px;
		position: relative;
		overflow: hidden;
	}

	.chart-container svg {
		width: 100%;
		height: 100%;
	}

	.grid {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		pointer-events: none;
	}

	.meter {
		height: 10px;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 3px;
		position: relative;
		overflow: hidden;
		margin-bottom: 8px;
	}

	.bar {
		height: 100%;
		position: relative;
		transition: width 0.3s ease-out;
	}

	.glow {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 15px;
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
		background: rgba(255, 255, 255, 0.1);
	}

	.memory-info {
		display: flex;
		justify-content: center;
		gap: 4px;
		font-size: 11px;
	}

	.used {
		font-weight: bold;
	}

	.total {
		opacity: 0.8;
	}

	.separator {
		opacity: 0.5;
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
