<script lang="ts">
	import AiDialog from './ai-dialog.svelte'

	let showDialog = false
	let isConnected = false
	let isHovered = false

	function handleConnectionSuccess() {
		isConnected = true
		showDialog = false
	}
</script>

<div class="elex-container">
	<button
		class="elex-btn {isConnected ? 'connected' : ''}"
		on:click={() => (showDialog = true)}
		on:mouseenter={() => (isHovered = true)}
		on:mouseleave={() => (isHovered = false)}
	>
		<div class="robot-container">
			<!-- Кастомный SVG робот -->
			<svg class="robot-icon" viewBox="0 0 64 64" width="40" height="40">
				<!-- Голова -->
				<rect
					x="20"
					y="10"
					width="24"
					height="20"
					rx="2"
					fill="#1a1a1a"
					stroke="#00ff00"
					stroke-width="1"
				/>
				<!-- Экран лица -->
				<rect
					x="24"
					y="14"
					width="16"
					height="12"
					fill="#003300"
					stroke="#00aa00"
					stroke-width="0.5"
				/>
				<!-- Глаза -->
				<circle cx="30" cy="20" r="2" fill="#00ff00" class="eye-pulse" />
				<circle
					cx="34"
					cy="20"
					r="2"
					fill="#00ff00"
					class="eye-pulse"
					style="animation-delay: 0.3s"
				/>
				<!-- Тело -->
				<rect
					x="22"
					y="30"
					width="20"
					height="24"
					fill="#111"
					stroke="#00ff00"
					stroke-width="1"
				/>
				<!-- Панель на груди -->
				<rect
					x="26"
					y="34"
					width="12"
					height="8"
					fill="#002200"
					stroke="#00aa00"
					stroke-width="0.5"
				/>
				<!-- Индикаторы -->
				<circle
					cx="28"
					cy="36"
					r="1"
					fill={isConnected ? '#00ff00' : '#ff0000'}
					class="status-led"
				/>
				<circle
					cx="32"
					cy="36"
					r="1"
					fill={isHovered ? '#ffff00' : '#555555'}
					class="status-led"
				/>
				<circle cx="36" cy="36" r="1" fill="#555555" class="status-led" />
				<!-- Руки -->
				<rect
					x="12"
					y="32"
					width="10"
					height="4"
					rx="1"
					fill="#1a1a1a"
					stroke="#00ff00"
					stroke-width="0.5"
				/>
				<rect
					x="42"
					y="32"
					width="10"
					height="4"
					rx="1"
					fill="#1a1a1a"
					stroke="#00ff00"
					stroke-width="0.5"
				/>
				<!-- Ноги -->
				<rect
					x="24"
					y="54"
					width="6"
					height="6"
					rx="1"
					fill="#1a1a1a"
					stroke="#00ff00"
					stroke-width="0.5"
				/>
				<rect
					x="34"
					y="54"
					width="6"
					height="6"
					rx="1"
					fill="#1a1a1a"
					stroke="#00ff00"
					stroke-width="0.5"
				/>

				{#if isConnected}
					<circle
						cx="32"
						cy="32"
						r="20"
						fill="none"
						stroke="#00ff00"
						stroke-width="1"
						stroke-dasharray="5,2"
						class="connection-wave"
						opacity="0.5"
					/>
				{/if}
			</svg>
		</div>

		<div class="btn-content">
			<div class="status-line">
				<span class="status-indicator {isConnected ? 'active' : ''}"></span>
				<span class="status-text">
					{isConnected ? 'SYSTEM ONLINE' : 'AWAITING CONNECTION'}
				</span>
			</div>
			<div class="btn-label">
				{isConnected ? 'NEURAL LINK ACTIVE' : 'INITIATE AI LINK'}
			</div>
		</div>

		<div class="signal-bars {isConnected ? 'active' : ''}">
			<div class="bar"></div>
			<div class="bar"></div>
			<div class="bar"></div>
			<div class="bar"></div>
		</div>
	</button>

	<div class="connection-details">
		{#if isConnected}
			<div class="detail-line">
				<span class="detail-label">PROTOCOL:</span>
				<span class="detail-value">QUANTUM ENCRYPTED</span>
			</div>
			<div class="detail-line">
				<span class="detail-label">BANDWIDTH:</span>
				<span class="detail-value">12.7 Tb/s</span>
			</div>
		{:else}
			<div class="detail-line warning">
				<span class="detail-label">STATUS:</span>
				<span class="detail-value">API KEY REQUIRED</span>
			</div>
		{/if}
	</div>
</div>

<AiDialog bind:showDialog on:connected={handleConnectionSuccess} />

<style>
	.elex-container {
		font-family: 'Courier New', monospace;
		--elex-green: #00ff00;
		--elex-dark: #0a0a0a;
		--elex-panel: #111111;
		--elex-border: #003300;
	}

	.elex-btn {
		position: relative;
		display: flex;
		align-items: center;
		gap: 15px;
		padding: 12px 20px;
		background: var(--elex-panel);
		border: 1px solid var(--elex-border);
		color: #aaa;
		cursor: pointer;
		transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);
		text-align: left;
		width: 320px;
		box-shadow: inset 0 0 10px rgba(0, 0, 0, 0.5);
		border-radius: 2px;
	}

	.elex-btn:hover {
		background: #151515;
		border-color: #006600;
		color: var(--elex-green);
		box-shadow:
			inset 0 0 15px rgba(0, 255, 0, 0.2),
			0 0 20px rgba(0, 255, 0, 0.1);
	}

	.elex-btn.connected {
		border-color: var(--elex-green);
		color: var(--elex-green);
		box-shadow:
			inset 0 0 20px rgba(0, 255, 0, 0.3),
			0 0 30px rgba(0, 255, 0, 0.2);
	}

	.robot-container {
		position: relative;
		flex-shrink: 0;
	}

	.robot-icon {
		filter: drop-shadow(0 0 5px rgba(0, 255, 0, 0.3));
	}

	.btn-content {
		flex-grow: 1;
	}

	.status-line {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 5px;
		font-size: 11px;
		letter-spacing: 0.5px;
	}

	.status-indicator {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #555;
		box-shadow: 0 0 5px rgba(0, 0, 0, 0.5);
		transition: all 0.3s ease;
	}

	.status-indicator.active {
		background: var(--elex-green);
		box-shadow: 0 0 10px rgba(0, 255, 0, 0.7);
		animation: pulse 1.5s infinite;
	}

	.btn-label {
		font-size: 14px;
		font-weight: bold;
		letter-spacing: 1px;
	}

	.signal-bars {
		display: flex;
		align-items: flex-end;
		gap: 3px;
		height: 20px;
		margin-left: auto;
	}

	.signal-bars .bar {
		width: 4px;
		background: #333;
		transition: all 0.3s ease;
	}

	.signal-bars .bar:nth-child(1) {
		height: 25%;
	}
	.signal-bars .bar:nth-child(2) {
		height: 50%;
	}
	.signal-bars .bar:nth-child(3) {
		height: 75%;
	}
	.signal-bars .bar:nth-child(4) {
		height: 100%;
	}

	.signal-bars.active .bar {
		background: var(--elex-green);
		box-shadow: 0 0 5px rgba(0, 255, 0, 0.7);
	}

	.signal-bars.active .bar:nth-child(1) {
		animation: barPulse 1.2s infinite;
	}
	.signal-bars.active .bar:nth-child(2) {
		animation: barPulse 1.2s infinite 0.3s;
	}
	.signal-bars.active .bar:nth-child(3) {
		animation: barPulse 1.2s infinite 0.6s;
	}
	.signal-bars.active .bar:nth-child(4) {
		animation: barPulse 1.2s infinite 0.9s;
	}

	.connection-details {
		margin-top: 10px;
		padding: 8px 12px;
		background: rgba(10, 10, 10, 0.7);
		border: 1px solid #002200;
		font-size: 11px;
		width: 320px;
	}

	.detail-line {
		display: flex;
		justify-content: space-between;
		margin-bottom: 4px;
	}

	.detail-line:last-child {
		margin-bottom: 0;
	}

	.detail-label {
		color: #666;
	}

	.detail-value {
		color: #ccc;
		font-weight: bold;
	}

	.detail-line.warning .detail-value {
		color: var(--elex-green);
	}

	/* Анимации */
	@keyframes pulse {
		0% {
			opacity: 1;
		}
		50% {
			opacity: 0.3;
		}
		100% {
			opacity: 1;
		}
	}

	@keyframes barPulse {
		0% {
			opacity: 0.3;
		}
		50% {
			opacity: 1;
		}
		100% {
			opacity: 0.3;
		}
	}

	.eye-pulse {
		animation: eyeGlow 2s infinite alternate;
	}

	@keyframes eyeGlow {
		from {
			opacity: 0.7;
		}
		to {
			opacity: 1;
		}
	}

	.connection-wave {
		animation: waveExpand 3s infinite;
		transform-origin: center;
	}

	@keyframes waveExpand {
		0% {
			transform: scale(0.8);
			opacity: 0;
		}
		50% {
			opacity: 0.5;
		}
		100% {
			transform: scale(1.5);
			opacity: 0;
		}
	}

	.status-led {
		animation: ledFlicker 5s infinite;
	}

	@keyframes ledFlicker {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.8;
		}
	}
</style>
