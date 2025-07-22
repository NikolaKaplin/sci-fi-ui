<script lang="ts">
	import { invoke } from '@tauri-apps/api/core'
	import { onMount } from 'svelte'

	export let showDialog: boolean
	let testing = false
	// Состояния
	let visible = false
	let animationStage = 'hidden'
	let horizontalWidth = 0
	let verticalHeight = 0
	let contentOpacity = 0
	let terminalLines: { text: string; revealed: number }[] = []
	let cursorVisible = true
	let contentElement: HTMLDivElement
	let apiKey = ''
	let showInput = false
	let validationError = ''

	// Увеличенные размеры окна
	const dialogWidth = 800
	const dialogHeight = 600

	const bootSequence = [
		'[SYSTEM] INITIALIZING API CONNECTION v2.3.1',
		'[SYSTEM] CHECKING SECURITY PROTOCOLS...',
		'[SYSTEM] ESTABLISHING ENCRYPTED CHANNEL...',
		'> CONNECTED TO OPENAI API GATEWAY',
		'> AUTHENTICATION REQUIRED',
		' ',
		'INSTRUCTIONS TO OBTAIN API KEY:',
		'1. VISIT https://auth.pollinations.ai/',
		'2. LOGIN OR CREATE ACCOUNT',
		'3. GET NEW API TOKEN"',
		'4. COPY GENERATED KEY',
		' ',
		'WARNING: NEVER SHARE YOUR API KEY',
		' ',
		'ADDITIONAL INFORMATION:',
		'• API keys grant access to powerful AI capabilities',
		'• Keep your keys secure like passwords',
		'• Rotate keys regularly for security',
		'• Contact support if keys are compromised',
		' ',
		'TROUBLESHOOTING:',
		'• Ensure you have an active account',
		'• Check your internet connection',
		'• Verify the API endpoint is correct',
		'• Review documentation for latest changes',
		' ',
		'> READY FOR KEY INPUT',
		'> ENTER YOUR API KEY BELOW:',
	]

	onMount(() => {
		const interval = setInterval(() => {
			cursorVisible = !cursorVisible
		}, 500)
		return () => clearInterval(interval)
	})

	$: if (showDialog) {
		openDialog()
	}

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

					if (
						contentElement.scrollTop + contentElement.clientHeight >=
						contentElement.scrollHeight - 50
					) {
						scrollToBottom()
					}

					setTimeout(typeNextChar, 10)
				} else {
					currentLine++
					currentChar = 0

					if (currentLine < terminalLines.length) {
						setTimeout(typeNextChar, 100)
					} else {
						setTimeout(() => {
							showInput = true
							scrollToBottom()
						}, 500)
					}
				}
			}
		}

		setTimeout(typeNextChar, 300)
	}

	async function handleSubmit() {
		console.log('submit')
		testing = true
		if (apiKey.trim().length === 0) {
			validationError = 'API KEY REQUIRED'
			testing = !testing
			return
		}
		try {
			await invoke<number[]>('generate_audio', {
				text: 'this is test api'.trim(),
				voice: 'onyx',
				token: apiKey.trim(),
			})

			try {
				await invoke('save_token', { token: apiKey.trim() })
				testing = false
				closeDialog()
			} catch (saveError) {
				console.error('Failed to save token:', saveError)
				validationError = 'SAVE FAILED'
				testing = false
			}
		} catch (error) {
			console.log(error)
			validationError = 'API KEY INCORRECT'
			testing = false
			return
		}

		testing = false
		closeDialog()
	}

	function closeDialog() {
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
		setTimeout(() => {}, 1500)
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
				showInput = false
				apiKey = ''
			}, 350)
		}
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
					style="width: {dialogWidth}px; height: {verticalHeight}px;"
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
									{line.text.slice(0, line.revealed)}
									{#if i === terminalLines.length - 1 && line.revealed === line.text.length}
										<span
											class="cursor"
											style="opacity: {cursorVisible ? 1 : 0}"
										></span>
									{/if}
								</div>
							{/each}

							{#if showInput}
								<form
									class="api-input-form"
									on:submit|preventDefault={handleSubmit}
								>
									<div class="input-line">
										<span class="prompt">></span>
										<input
											type="password"
											bind:value={apiKey}
											class="api-input {validationError ? 'error' : ''}"
											placeholder="sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
										/>
										{#if validationError}
											<span class="error-message">{validationError}</span>
										{/if}
									</div>
									{#if !testing}
										<button type="submit" class="submit-btn"> CONNECT </button>
									{:else}
										<div class="submit-btn">TESTING API KEY ...</div>
									{/if}
								</form>
							{/if}
						</div>
					</div>
					<div class="scanlines"></div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
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
		width: 800px;
		height: 600px;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.dialog-container {
		position: relative;
		width: 800px;
		height: 600px;
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
		overflow-y: auto; /* Включаем вертикальный скролл */
		opacity: 0;
		scrollbar-width: thin;
		scrollbar-color: #0f0 transparent;
	}

	/* Стилизация скроллбара */
	.dialog-content::-webkit-scrollbar {
		width: 8px;
	}

	.dialog-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.dialog-content::-webkit-scrollbar-thumb {
		background-color: #0f0;
		border-radius: 4px;
		border: 1px solid rgba(0, 255, 0, 0.3);
	}

	.dialog-lines {
		min-height: 100%;
		display: flex;
		flex-direction: column;
	}

	.dialog-line {
		margin-bottom: 10px;
		white-space: pre;
		flex-shrink: 0;
	}

	.cursor {
		display: inline-block;
		width: 9px;
		height: 18px;
		background-color: #0f0;
		vertical-align: middle;
		margin-left: 3px;
	}

	.api-input-form {
		margin-top: 20px;
		margin-bottom: 20px;
		flex-shrink: 0;
	}

	.input-line {
		display: flex;
		align-items: center;
		margin-bottom: 15px;
	}

	.prompt {
		color: #0f0;
		margin-right: 10px;
	}

	.api-input {
		flex: 1;
		background: transparent;
		border: none;
		border-bottom: 1px solid #0f0;
		color: #0f0;
		font-family: 'Courier New', monospace;
		font-size: 15px;
		padding: 5px;
		outline: none;
		min-width: 300px;
	}

	.api-input.error {
		border-bottom-color: #f00;
	}

	.error-message {
		color: #f00;
		margin-left: 10px;
		text-shadow: 0 0 5px #f00;
	}

	.submit-btn {
		background: rgba(0, 255, 0, 0.1);
		border: 1px solid #0f0;
		color: #0f0;
		font-family: 'Courier New', monospace;
		padding: 8px 20px;
		cursor: pointer;
		transition: all 0.3s;
	}

	.submit-btn:hover {
		background: rgba(0, 255, 0, 0.2);
		box-shadow: 0 0 10px rgba(0, 255, 0, 0.5);
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
