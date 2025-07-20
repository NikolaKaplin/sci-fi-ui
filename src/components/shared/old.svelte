<!-- oldScreenEffect.svelte -->
<script lang="ts">
	import { onDestroy, onMount } from 'svelte'

	let flickerIntensity = 0
	let noiseOpacity = 0.03
	let scanlineOpacity = 0.1
	let glitchActive = false

	let flickerInterval: number
	let noiseInterval: number
	let glitchInterval: number

	onMount(() => {
		// Мерцание экрана
		flickerInterval = setInterval(() => {
			flickerIntensity = Math.random() * 0.1
		}, 100)

		// Изменение интенсивности шума
		noiseInterval = setInterval(() => {
			noiseOpacity = 0.02 + Math.random() * 0.02
		}, 500)

		// Редкие глитч-эффекты
		glitchInterval = setInterval(() => {
			if (Math.random() > 0.95) {
				glitchActive = true
				setTimeout(
					() => {
						glitchActive = false
					},
					50 + Math.random() * 150
				)
			}
		}, 3000)

		return () => {
			clearInterval(flickerInterval)
			clearInterval(noiseInterval)
			clearInterval(glitchInterval)
		}
	})

	onDestroy(() => {
		clearInterval(flickerInterval)
		clearInterval(noiseInterval)
		clearInterval(glitchInterval)
	})
</script>

<div class="old-screen-effect">
	<!-- Основной контент -->
	<div class="content" style={`opacity: ${1 - flickerIntensity}`}>
		<slot />
	</div>

	<!-- Наложение шума/зернистости -->
	<div class="noise" style={`opacity: ${noiseOpacity}`} />

	<!-- Сканирующие линии -->
	<div class="scanlines" style={`opacity: ${scanlineOpacity}`} />

	<!-- Эффект глитча -->
	{#if glitchActive}
		<div class="glitch-effect" />
	{/if}
</div>

<style>
	.old-screen-effect {
		position: relative;
		width: 100%;
		height: 100%;
		overflow: hidden;
		background-color: #000;
		color: #0f0;
		font-family: 'Courier New', monospace;
	}

	.content {
		width: 100%;
		height: 100%;
		transition: opacity 0.1s ease;
	}

	.noise {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)' opacity='0.20'/%3E%3C/svg%3E");
		pointer-events: none;
	}

	.scanlines {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: linear-gradient(rgba(0, 0, 0, 0) 50%, rgba(0, 255, 0, 0.1) 50%);
		background-size: 100% 4px;
		pointer-events: none;
	}

	.glitch-effect {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: linear-gradient(
			0deg,
			transparent 45%,
			rgba(0, 255, 0, 0.2) 45%,
			rgba(0, 255, 0, 0.2) 55%,
			transparent 55%
		);
		pointer-events: none;
		animation: glitch 0.2s infinite;
	}

	@keyframes glitch {
		0% {
			transform: translateX(-2px);
		}
		20% {
			transform: translateX(2px);
		}
		40% {
			transform: translateX(-1px);
		}
		60% {
			transform: translateX(1px);
		}
		80% {
			transform: translateX(-1px);
		}
		100% {
			transform: translateX(0);
		}
	}
</style>
