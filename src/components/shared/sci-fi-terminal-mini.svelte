<script lang="ts">
	import { onMount } from 'svelte'

	export let isOpen = false
	let terminalContent = ''
	let cursorVisible = true
	let animationProgress = 0

	// Сообщения для терминала
	const messages = [
		'> SYSTEM INITIALIZATION',
		'> MEMORY CHECK: OK',
		'> SECURITY PROTOCOLS: ENABLED',
		'> BIOMETRIC SCAN COMPLETE',
		'> WELCOME, USER',
	]

	// CSS-переменные для динамических стилей
	let styleVars = {
		cursorOpacity: '1',
		terminalHeight: '0px',
	}

	onMount(() => {
		const cursorInterval = setInterval(() => {
			cursorVisible = !cursorVisible
			styleVars.cursorOpacity = cursorVisible ? '1' : '0'
		}, 500)

		return () => clearInterval(cursorInterval)
	})

	function toggleTerminal() {
		isOpen = !isOpen
		animationProgress = 0
		terminalContent = ''

		if (isOpen) {
			const animate = () => {
				if (animationProgress < 100) {
					animationProgress += 2
					styleVars.terminalHeight = `${animationProgress * 2}px`
					requestAnimationFrame(animate)
				} else {
					typeText()
				}
			}
			animate()
		} else {
			styleVars.terminalHeight = '0px'
		}
	}

	function typeText() {
		let currentMessage = 0
		let currentChar = 0

		const typeInterval = setInterval(() => {
			if (currentMessage < messages.length) {
				if (currentChar < messages[currentMessage].length) {
					terminalContent += messages[currentMessage][currentChar]
					currentChar++
				} else {
					terminalContent += '\n'
					currentMessage++
					currentChar = 0
				}
			} else {
				clearInterval(typeInterval)
			}
		}, 50)
	}
</script>

<div
	class="terminal-container"
	style="--terminal-height: {styleVars.terminalHeight}"
>
	<div class="terminal-header" on:click={toggleTerminal}>
		<span>TERMINAL v3.7.2</span>
		<span>{isOpen ? '▲' : '▼'}</span>
	</div>

	{#if isOpen}
		<div class="terminal-body">
			<div class="terminal-content">
				{terminalContent}
				{#if terminalContent.length > 0}
					<span
						class="cursor"
						style="--cursor-opacity: {styleVars.cursorOpacity}"
					></span>
				{/if}
			</div>
			<div class="scanlines" />
		</div>
	{/if}
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		background-color: #111;
	}

	.terminal-container {
		position: fixed;
		bottom: 20px;
		right: 20px;
		width: 400px;
		overflow: hidden;
		border-radius: 4px;
		box-shadow: 0 0 20px rgba(0, 255, 0, 0.3);
	}

	.terminal-header {
		background: linear-gradient(to right, #1a1a1a, #333);
		color: #0f0;
		padding: 8px 12px;
		font-family: 'Courier New', monospace;
		font-size: 14px;
		cursor: pointer;
		user-select: none;
		display: flex;
		justify-content: space-between;
	}

	.terminal-body {
		background-color: rgba(0, 0, 0, 0.8);
		color: #0f0;
		font-family: 'Courier New', monospace;
		font-size: 14px;
		padding: 12px;
		height: var(--terminal-height, 0px);
		overflow-y: auto;
		white-space: pre-wrap;
		border: 1px solid #0f0;
		border-top: none;

		/* Эффект старых мониторов */
		text-shadow: 0 0 5px #0f0;
		background-image: linear-gradient(
			rgba(0, 255, 0, 0.03) 50%,
			transparent 50%
		);
		background-size: 100% 4px;
	}

	.terminal-content {
		line-height: 1.4;
	}

	.cursor {
		display: inline-block;
		width: 8px;
		height: 16px;
		background-color: #0f0;
		opacity: var(--cursor-opacity, 1);
		vertical-align: middle;
		margin-left: 2px;
	}

	/* Эффект сканирования */
	@keyframes scanline {
		0% {
			background-position: 0 0;
		}
		100% {
			background-position: 0 100%;
		}
	}

	.scanlines {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: linear-gradient(
			to bottom,
			transparent 95%,
			rgba(0, 255, 0, 0.1) 95%
		);
		background-size: 100% 4px;
		pointer-events: none;
		animation: scanline 6s linear infinite;
		z-index: 1;
	}

	/* Эффект мерцания */
	@keyframes flicker {
		0%,
		19%,
		21%,
		23%,
		25%,
		54%,
		56%,
		100% {
			opacity: 1;
		}
		20%,
		22%,
		24%,
		55% {
			opacity: 0.5;
		}
	}

	.terminal-body {
		animation: flicker 5s infinite;
	}
</style>
