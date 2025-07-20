<script lang="ts">
	import { listen } from '@tauri-apps/api/event'
	import { Globe } from 'encom-globe'
	import 'encom-globe/style.css'
	import { onMount } from 'svelte'

	// Данные соединения
	let latency = 0
	let downloadSpeed = 0
	let uploadSpeed = 0
	let status = 'offline'
	let statusClass = 'offline'

	// История для графиков
	let latencyHistory: number[] = Array(60).fill(0)
	let downloadHistory: number[] = Array(60).fill(0)
	let uploadHistory: number[] = Array(60).fill(0)

	// Глобус
	let globeContainer: HTMLDivElement
	let globe: Globe
	let animationFrameId: number

	// Цвета статусов
	const statusColors = {
		excellent: '#00ff00',
		good: '#aaff00',
		average: '#ffff00',
		poor: '#ffaa00',
		offline: '#ff0000',
		timeout: '#ff0000',
	}

	// Точки для соединений (широта, долгота)
	const connectionPoints = [
		{ lat: 40.7128, lon: -74.006, name: 'New York' },
		{ lat: 51.5074, lon: -0.1278, name: 'London' },
		{ lat: 35.6762, lon: 139.6503, name: 'Tokyo' },
		{ lat: 37.7749, lon: -122.4194, name: 'San Francisco' },
		{ lat: 34.0522, lon: -118.2437, name: 'Los Angeles' },
	]

	onMount(() => {
		// Инициализация глобуса
		initGlobe()

		// Слушатель обновлений
		const unlisten = listen<{
			latency: number
			status: string
			download: number
			upload: number
		}>('ping_update', event => {
			latency = event.payload.latency
			downloadSpeed = event.payload.download
			uploadSpeed = event.payload.upload
			status = event.payload.status
			statusClass = event.payload.status

			// Обновляем историю
			latencyHistory = [...latencyHistory.slice(1), latency]
			downloadHistory = [...downloadHistory.slice(1), downloadSpeed]
			uploadHistory = [...uploadHistory.slice(1), uploadSpeed]

			// Обновляем глобус
			updateGlobe()
		})

		return () => {
			unlisten.then(f => f())
			cancelAnimationFrame(animationFrameId)
			if (globe) globe.dispose()
		}
	})

	function initGlobe() {
		globe = new Globe(globeContainer, {
			clouds: true,
			cloudsSpeed: 0.006,
			pointSize: 2,
			baseColor: '#0a192f',
			markerColor: statusColors[statusClass as keyof typeof statusColors],
			glowColor: '#00aaff',
			trails: true,
			autoRotation: true,
			rotationSpeed: 0.3,
		})

		// Добавляем точки соединений
		connectionPoints.forEach(point => {
			globe.addMarker(point.lat, point.lon, {
				color: statusColors[statusClass as keyof typeof statusColors],
				size: 0.1,
			})
		})

		// Анимация
		animate()
	}

	function updateGlobe() {
		// Очищаем предыдущие соединения
		globe.clearLines()

		if (status !== 'offline' && status !== 'timeout') {
			// Создаем новые соединения
			const mainPoint = { lat: 0, lon: 0 } // Центральная точка

			connectionPoints.forEach(point => {
				globe.addLine(mainPoint.lat, mainPoint.lon, point.lat, point.lon, {
					color: statusColors[statusClass as keyof typeof statusColors],
					dashLength: 0.02,
					dashGap: 0.008,
					dashSpeed: -0.0005 * latency,
				})
			})
		}

		// Обновляем цвет маркеров
		connectionPoints.forEach((point, i) => {
			globe.markers[i].color =
				statusColors[statusClass as keyof typeof statusColors]
		})
	}

	function animate() {
		animationFrameId = requestAnimationFrame(animate)
		globe.tick()
	}
</script>

<div class="network-dashboard">
	<div class="globe-section">
		<div bind:this={globeContainer} class="globe-view"></div>
		<div class="globe-status {statusClass}">
			<div class="status-title">NETWORK STATUS</div>
			<div class="status-value">{status.toUpperCase()}</div>
		</div>
	</div>

	<div class="stats-section">
		<div class="stat-card ping">
			<div class="stat-title">LATENCY</div>
			<div class="stat-value {statusClass}">
				{latency > 0 ? `${latency} ms` : '--'}
			</div>
			<div class="stat-chart">
				<svg viewBox="0 0 100 40" preserveAspectRatio="none">
					<path
						d={`M0,40 ${latencyHistory.map((val, i) => `L${i * 1.67},${40 - Math.min(val, 40)}`).join(' ')}`}
						stroke={statusColors[statusClass as keyof typeof statusColors]}
						fill="none"
						stroke-width="1.5"
					/>
				</svg>
			</div>
		</div>

		<div class="speed-cards">
			<div class="stat-card download">
				<div class="stat-title">DOWNLOAD</div>
				<div class="stat-value">{downloadSpeed.toFixed(1)} Mbps</div>
				<div class="stat-chart">
					<svg viewBox="0 0 100 30" preserveAspectRatio="none">
						<path
							d={`M0,30 ${downloadHistory.map((val, i) => `L${i * 1.67},${30 - Math.min(val / 10, 30)}`).join(' ')}`}
							stroke="#00aaff"
							fill="none"
							stroke-width="1"
						/>
					</svg>
				</div>
			</div>

			<div class="stat-card upload">
				<div class="stat-title">UPLOAD</div>
				<div class="stat-value">{uploadSpeed.toFixed(1)} Mbps</div>
				<div class="stat-chart">
					<svg viewBox="0 0 100 30" preserveAspectRatio="none">
						<path
							d={`M0,30 ${uploadHistory.map((val, i) => `L${i * 1.67},${30 - Math.min(val / 10, 30)}`).join(' ')}`}
							stroke="#ff5500"
							fill="none"
							stroke-width="1"
						/>
					</svg>
				</div>
			</div>
		</div>

		<div class="connection-points">
			<div class="points-title">CONNECTION POINTS</div>
			<div class="points-list">
				{#each connectionPoints as point}
					<div class="point-item">
						<div
							class="point-marker"
							style="background: {statusColors[
								statusClass as keyof typeof statusColors
							]}"
						></div>
						<div class="point-name">{point.name}</div>
						<div class="point-latency">
							{latency > 0
								? `${latency + Math.floor(Math.random() * 20)} ms`
								: '--'}
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>

<style>
	.network-dashboard {
		display: flex;
		gap: 20px;
		background: #0a192f;
		border: 1px solid rgba(0, 170, 255, 0.2);
		border-radius: 8px;
		padding: 15px;
		font-family: 'Courier New', monospace;
		color: #e6f1ff;
		max-width: 900px;
		box-shadow: 0 0 30px rgba(0, 50, 100, 0.5);
	}

	.globe-section {
		position: relative;
		width: 400px;
		height: 400px;
	}

	.globe-view {
		width: 100%;
		height: 100%;
		border-radius: 50%;
		overflow: hidden;
		border: 1px solid rgba(0, 170, 255, 0.3);
		box-shadow: 0 0 30px rgba(0, 100, 255, 0.2);
	}

	.globe-status {
		position: absolute;
		bottom: 20px;
		left: 50%;
		transform: translateX(-50%);
		background: rgba(10, 25, 47, 0.8);
		border: 1px solid;
		border-radius: 4px;
		padding: 8px 15px;
		text-align: center;
		min-width: 180px;
	}

	.status-title {
		font-size: 0.7em;
		opacity: 0.8;
		margin-bottom: 2px;
		letter-spacing: 1px;
	}

	.status-value {
		font-size: 1.2em;
		font-weight: bold;
		letter-spacing: 2px;
	}

	.stats-section {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 15px;
	}

	.stat-card {
		background: rgba(10, 25, 47, 0.6);
		border: 1px solid rgba(0, 170, 255, 0.2);
		border-radius: 6px;
		padding: 15px;
	}

	/* .ping {
    border-top: 3px solid {statusColors[statusClass as keyof typeof statusColors]};
  } */

	.download {
		border-top: 3px solid #00aaff;
	}

	.upload {
		border-top: 3px solid #ff5500;
	}

	.stat-title {
		font-size: 0.8em;
		opacity: 0.8;
		margin-bottom: 5px;
		letter-spacing: 1px;
	}

	.stat-value {
		font-size: 1.8em;
		font-weight: bold;
		margin-bottom: 10px;
	}

	.stat-chart {
		height: 40px;
		background: rgba(0, 10, 20, 0.3);
		border-radius: 2px;
		overflow: hidden;
	}

	.speed-cards {
		display: flex;
		gap: 10px;
	}

	.speed-cards .stat-card {
		flex: 1;
	}

	.connection-points {
		background: rgba(10, 25, 47, 0.6);
		border: 1px solid rgba(0, 170, 255, 0.2);
		border-radius: 6px;
		padding: 15px;
		margin-top: auto;
	}

	.points-title {
		font-size: 0.8em;
		opacity: 0.8;
		margin-bottom: 10px;
		letter-spacing: 1px;
	}

	.points-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.point-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 5px 0;
		border-bottom: 1px solid rgba(0, 170, 255, 0.1);
	}

	.point-marker {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		box-shadow: 0 0 5px currentColor;
	}

	.point-name {
		flex: 1;
		font-size: 0.9em;
	}

	.point-latency {
		font-size: 0.8em;
		opacity: 0.8;
	}

	/* Статусные цвета */
	.excellent {
		color: #00ff00;
		text-shadow: 0 0 8px #00ff00;
		border-color: rgba(0, 255, 0, 0.3);
	}

	.good {
		color: #aaff00;
		text-shadow: 0 0 8px #aaff00;
		border-color: rgba(170, 255, 0, 0.3);
	}

	.average {
		color: #ffff00;
		text-shadow: 0 0 8px #ffff00;
		border-color: rgba(255, 255, 0, 0.3);
	}

	.poor {
		color: #ffaa00;
		text-shadow: 0 0 8px #ffaa00;
		border-color: rgba(255, 170, 0, 0.3);
	}

	.offline,
	.timeout {
		color: #ff0000;
		text-shadow: 0 0 8px #ff0000;
		border-color: rgba(255, 0, 0, 0.3);
		animation: blink 1s infinite;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}
</style>
