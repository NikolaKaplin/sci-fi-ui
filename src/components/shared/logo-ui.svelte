<script lang="ts">
	import { onMount } from 'svelte'

	export let duration = 800 // Общее время показа логотипа
	export let logoText = 'RIZO' // Текст логотипа
	export let typingSpeed = 200 // Скорость печати каждой буквы

	let visible = true
	let typingComplete = false

	onMount(() => {
		const typingTime = logoText.length * typingSpeed + 1000
		setTimeout(() => {
			typingComplete = true

			setTimeout(() => {
				visible = false
			}, duration - typingTime)
		}, typingTime)
	})
</script>

{#if visible}
	<div class="logo-container {typingComplete ? 'assembled' : ''}">
		<div class="logo-assembly">
			{#each logoText as char, i}
				<div
					class="logo-piece piece-{i + 1} {typingComplete ? 'typed' : ''}"
					style={`--typing-delay: ${i * typingSpeed}ms`}
				>
					{char}
					<div></div>
				</div>
			{/each}
		</div>
	</div>
{/if}

<style>
	.logo-container {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		justify-content: center;
		align-items: center;
		background-color: #000;
		z-index: 2000;
		pointer-events: none;
		opacity: 1;
		transition: opacity 0.5s ease-out;
	}

	.logo-container:not(.assembled) {
		opacity: 1;
	}

	.logo-assembly {
		display: flex;
		gap: 10px;
		perspective: 1000px;
	}

	.logo-piece {
		position: relative;
		font-family: 'Courier New', monospace;
		font-weight: bold;
		font-size: 80px;
		color: #0f0;
		text-shadow:
			0 0 5px #0f0,
			0 0 10px #0f0;
		transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
		opacity: 0;
		filter: drop-shadow(0 0 5px #0f0);
		transform: translateY(20px);
	}

	.logo-piece {
		animation:
			typeIn 0.3s forwards cubic-bezier(0.175, 0.885, 0.32, 1.275),
			glow 2s infinite alternate;
		animation-delay: var(--typing-delay);
	}

	@keyframes typeIn {
		from {
			opacity: 0;
			transform: translateY(20px) scale(0.8);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.cursor {
		position: absolute;
		right: -10px;
		top: 5px;
		width: 3px;
		height: 70px;
		background-color: #0f0;
		opacity: 0;
		box-shadow: 0 0 5px #0f0;
	}

	.cursor.visible {
		animation: blink 0.7s infinite;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 0;
		}
		50% {
			opacity: 1;
		}
	}

	@keyframes glow {
		0%,
		100% {
			text-shadow:
				0 0 5px #0f0,
				0 0 10px #0f0;
		}
		50% {
			text-shadow:
				0 0 10px #0f0,
				0 0 20px #0f0,
				0 0 30px #0f0;
		}
	}

	.logo-container.fade-out {
		opacity: 1;
	}
</style>
