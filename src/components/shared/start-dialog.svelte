<script lang="ts">
	import { onMount } from 'svelte'

	export let showDialog: boolean
	export let onComplete: () => void

	$: if (showDialog) {
		openDialog()
	}

	let visible = false
	let animationStage = 'hidden'
	let horizontalWidth = 0
	let verticalHeight = 0
	let contentOpacity = 0
	let terminalLines: { text: string; revealed: number }[] = []
	let cursorVisible = true
	let contentElement: HTMLDivElement

	const dialogWidth = 700
	const dialogHeight = 400

	const bootSequence = [
		'[SYSTEM] INITIALIZING SCI-FI UI RIZO v1',
		'[SYSTEM] USER: RIZO',
		'[SYSTEM] MEMORY ALLOCATION: 12.4GB/16.0GB',
		'[SYSTEM] CPU USAGE: 23%',
		'> SYSTEM TIME: {LOCAL_TIME}',
		'> LAST LOGIN: {LAST_LOGIN}',
		'> SECURITY STATUS: GREEN',
		'> NETWORK STATUS: STABLE',
		'> PRESS [ENTER] TO CONTINUE',
	]

	onMount(() => {
		const interval = setInterval(() => {
			cursorVisible = !cursorVisible
		}, 500)
		return () => clearInterval(interval)
	})

	function scrollToBottom() {
		if (contentElement) {
			contentElement.scrollTop = contentElement.scrollHeight
		}
	}

	function openDialog() {
		if (animationStage !== 'hidden') return

		visible = true
		startAnimation()
	}

	function startAnimation() {
		animationStage = 'dot'

		setTimeout(() => {
			animationStage = 'horizontal'
			animateHorizontal()
		}, 350)
	}

	function animateHorizontal() {
		if (horizontalWidth < dialogWidth) {
			horizontalWidth += 25
			requestAnimationFrame(animateHorizontal)
		} else {
			animationStage = 'vertical'
			animateVertical()
		}
	}

	function animateVertical() {
		if (verticalHeight < dialogHeight) {
			verticalHeight += 15
			requestAnimationFrame(animateVertical)
		} else {
			animationStage = 'content'
			showContent()
		}
	}

	function showContent() {
		terminalLines = bootSequence.map(line => ({ text: line, revealed: 0 }))

		const fadeIn = () => {
			if (contentOpacity < 1) {
				contentOpacity += 0.03
				requestAnimationFrame(fadeIn)
			}
		}

		fadeIn()
		typeContent()
	}

	function typeContent() {
		let currentLine = 0
		let currentChar = 0

		const typeNextChar = () => {
			if (currentLine < terminalLines.length) {
				if (currentChar < terminalLines[currentLine].text.length) {
					terminalLines[currentLine].revealed++
					currentChar++

					scrollToBottom()

					setTimeout(typeNextChar, 10)
				} else {
					currentLine++
					currentChar = 0

					if (currentLine < terminalLines.length) {
						setTimeout(typeNextChar, 100)
					} else {
						setTimeout(() => {
							window.addEventListener('keydown', handleKeyPress)
						}, 500)
					}
				}
			}
		}

		setTimeout(typeNextChar, 300)
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			closeDialog()
		}
	}

	function closeDialog() {
		window.removeEventListener('keydown', handleKeyPress)

		const fadeOut = () => {
			if (contentOpacity > 0) {
				contentOpacity -= 0.05
				requestAnimationFrame(fadeOut)
			} else {
				animationStage = 'vertical'
				animateVerticalClose()
			}
		}

		fadeOut()
		setTimeout(() => {
			onComplete()
		}, 1500)
	}

	function animateVerticalClose() {
		if (verticalHeight > 0) {
			verticalHeight -= 15
			requestAnimationFrame(animateVerticalClose)
		} else {
			animationStage = 'horizontal'
			animateHorizontalClose()
		}
	}

	function animateHorizontalClose() {
		if (horizontalWidth > 0) {
			horizontalWidth -= 25
			requestAnimationFrame(animateHorizontalClose)
		} else {
			animationStage = 'dot'
			setTimeout(() => {
				animationStage = 'hidden'
				visible = false
				terminalLines = []
			}, 350)
		}
	}

	function formatTime(date: Date) {
		return date.toLocaleTimeString() + ' ' + date.toLocaleDateString()
	}
</script>

{#if visible}
	<div class="dialog-overlay">
		<div class="dialog-animation-container">
			{#if animationStage === 'dot'}
				<div class="dot"></div>
			{:else if animationStage === 'horizontal'}
				<div class="horizontal-line" style="width: {horizontalWidth}px;"></div>
			{:else if animationStage === 'vertical'}
				<div
					class="vertical-expand glow-effect"
					style="width: 700px; height: {verticalHeight}px;"
				>
					<div class="scanlines"></div>
				</div>
			{:else if animationStage === 'content'}
				<div
					class="dialog-container glow-effect"
					style="opacity: {contentOpacity}"
				>
					<div
						class="dialog-content"
						style="opacity: {contentOpacity}"
						bind:this={contentElement}
					>
						<div class="dialog-lines">
							{#each terminalLines as line, i}
								<div class="dialog-line">
									{#if line.text.includes('{LOCAL_TIME}')}
										{line.text
											.slice(0, line.revealed)
											.replace('{LOCAL_TIME}', formatTime(new Date()))}
									{:else if line.text.includes('{LAST_LOGIN}')}
										{line.text
											.slice(0, line.revealed)
											.replace(
												'{LAST_LOGIN}',
												formatTime(new Date(Date.now() - 86400000))
											)}
									{:else}
										{line.text.slice(0, line.revealed)}
									{/if}
									{#if i === terminalLines.length - 1 && line.revealed === line.text.length}
										<span
											class="cursor"
											style="opacity: {cursorVisible ? 1 : 0}"
										></span>
									{/if}
								</div>
							{/each}
						</div>
					</div>
					<div class="scanlines"></div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		background-color: #000;
		color: #0f0;
		font-family: 'Courier New', monospace;
		overflow: hidden;
	}

	.dialog-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 1000;
		background-color: rgba(0, 0, 0, 0.7);
	}

	.dialog-animation-container {
		position: absolute;
		width: 700px;
		height: 400px;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.dialog-container {
		position: relative;
		width: 700px;
		height: 400px;
		background: rgba(10, 20, 10, 0.9);
		border: 1px solid #0f0;
		box-shadow: 0 0 30px rgba(0, 255, 0, 0.6);
		overflow: hidden;
		color: #0f0;
		font-size: 15px;
		line-height: 1.5;
		text-shadow: 0 0 7px #0f0;
	}

	.dialog-content {
		padding: 25px;
		height: calc(100% - 50px);
		overflow: hidden;
		opacity: 0;
	}

	.dialog-lines {
		height: 100%;
		overflow: hidden;
		pointer-events: none;
	}

	.dialog-line {
		margin-bottom: 10px;
		white-space: pre;
	}

	.cursor {
		display: inline-block;
		width: 9px;
		height: 18px;
		background-color: #0f0;
		vertical-align: middle;
		margin-left: 3px;
		animation: blink 1s infinite;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0;
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
		background-size: 100% 5px;
		pointer-events: none;
		animation: scanline 5s linear infinite;
	}

	@keyframes scanline {
		0% {
			background-position: 0 0;
		}
		100% {
			background-position: 0 100%;
		}
	}

	.glow-effect {
		animation: flicker 7s infinite;
	}

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
			opacity: 0.95;
		}
	}

	.dot {
		width: 5px;
		height: 5px;
		background: #0f0;
		border-radius: 50%;
		position: absolute;
		box-shadow: 0 0 10px #0f0;
	}

	.horizontal-line {
		height: 3px;
		background: linear-gradient(to right, #0f0, #0a0);
		position: absolute;
		box-shadow: 0 0 15px #0f0;
	}

	.vertical-expand {
		background: rgba(10, 20, 10, 0.9);
		border: 1px solid #0f0;
		position: absolute;
		overflow: hidden;
		box-shadow: 0 0 25px rgba(0, 255, 0, 0.7);
	}
</style>
