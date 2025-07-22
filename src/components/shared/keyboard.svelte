<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte'
	import { writable, type Writable } from 'svelte/store'

	export let syncWithPhysicalKeyboard: boolean = false // Опция синхронизации

	// Initialize event dispatcher
	const dispatch = createEventDispatcher<{
		keypress: { key: string; value: string | null }
	}>()

	// Звуковые эффекты
	interface KeySounds {
		default: HTMLAudioElement
		special: HTMLAudioElement
		space: HTMLAudioElement
		backspace: HTMLAudioElement
	}

	const keySounds: KeySounds = {
		default: new Audio(
			'https://assets.mixkit.co/sfx/preview/mixkit-modern-click-box-check-1120.mp3'
		),
		special: new Audio(
			'https://assets.mixkit.co/sfx/preview/mixkit-arcade-game-jump-coin-216.mp3'
		),
		space: new Audio(
			'https://assets.mixkit.co/sfx/preview/mixkit-quick-jump-arcade-game-239.mp3'
		),
		backspace: new Audio(
			'https://assets.mixkit.co/sfx/preview/mixkit-arcade-retro-game-over-213.mp3'
		),
	}

	const activeKeys: Writable<Set<string>> = writable(new Set())
	let isShiftPressed: boolean = false
	let isCapsLock: boolean = false

	// Раскладка клавиатуры с цифровым рядом
	const sciFiLayout: string[][] = [
		[
			'Esc',
			'F1',
			'F2',
			'F3',
			'F4',
			'F5',
			'F6',
			'F7',
			'F8',
			'F9',
			'F10',
			'F11',
			'F12',
		],
		[
			'`',
			'1',
			'2',
			'3',
			'4',
			'5',
			'6',
			'7',
			'8',
			'9',
			'0',
			'-',
			'=',
			'Backspace',
		],
		['Tab', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\\'],
		['Caps', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', "'", 'Enter'],
		['Shift', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', 'Shift'],
		['Ctrl', 'Win', 'Alt', 'Space', 'Alt', 'Ctrl', 'Fn'],
	]

	// Синхронизация с физической клавиатурой
	onMount(() => {
		if (syncWithPhysicalKeyboard) {
			window.addEventListener('keydown', handlePhysicalKeyDown)
			window.addEventListener('keyup', handlePhysicalKeyUp)
		}

		return () => {
			window.removeEventListener('keydown', handlePhysicalKeyDown)
			window.removeEventListener('keyup', handlePhysicalKeyUp)
		}
	})

	function handlePhysicalKeyDown(e: KeyboardEvent): void {
		const virtualKey = getVirtualKeyFromPhysical(e.key)
		if (virtualKey) {
			handleKeyPress(virtualKey, true)
		}
	}

	function handlePhysicalKeyUp(e: KeyboardEvent): void {
		const virtualKey = getVirtualKeyFromPhysical(e.key)
		if (virtualKey) {
			activeKeys.update(keys => {
				keys.delete(virtualKey)
				return keys
			})
		}
	}

	function getVirtualKeyFromPhysical(key: string): string | undefined {
		const mapping: Record<string, string> = {
			Escape: 'Esc',
			Backspace: 'Backspace',
			Enter: 'Enter',
			Shift: 'Shift',
			CapsLock: 'Caps',
			Tab: 'Tab',
			' ': 'Space',
			Control: 'Ctrl',
			Alt: 'Alt',
			Meta: 'Win',
			F1: 'F1',
			F2: 'F2',
			F3: 'F3',
			F4: 'F4',
			F5: 'F5',
			F6: 'F6',
			F7: 'F7',
			F8: 'F8',
			F9: 'F9',
			F10: 'F10',
			F11: 'F11',
			F12: 'F12',
		}

		if (/^[0-9]$/.test(key)) return key
		if (/^[`\-=\[\]\\;',./]$/.test(key)) return key

		return mapping[key] || key.toUpperCase()
	}

	// Воспроизведение звука
	function playSound(key: string): void {
		let sound: HTMLAudioElement

		if (key === 'Space') sound = keySounds.space
		else if (key === 'Backspace' || key === 'Enter') sound = keySounds.backspace
		else if (key.length > 1) sound = keySounds.special
		else sound = keySounds.default

		sound.currentTime = 0
		sound.play().catch(e => console.log('Audio play failed:', e))
	}

	function handleKeyPress(key: string, fromPhysical: boolean = false): void {
		if (!fromPhysical) {
			playSound(key)
		}

		activeKeys.update(keys => {
			keys.add(key)
			return keys
		})

		if (!fromPhysical) {
			setTimeout(() => {
				activeKeys.update(keys => {
					keys.delete(key)
					return keys
				})
			}, 150)
		}

		switch (key) {
			case 'Shift':
				isShiftPressed = !isShiftPressed
				return
			case 'Caps':
				isCapsLock = !isCapsLock
				return
		}

		dispatch('keypress', {
			key,
			value: getKeyValue(key),
		})
	}

	function getKeyValue(key: string): string | null {
		if (key.length > 1) return null

		if (/^[0-9`\-=\[\]\\;',./]$/.test(key)) {
			if (isShiftPressed) {
				// Символы с Shift
				const shiftMap: Record<string, string> = {
					'`': '~',
					'1': '!',
					'2': '@',
					'3': '#',
					'4': '$',
					'5': '%',
					'6': '^',
					'7': '&',
					'8': '*',
					'9': '(',
					'0': ')',
					'-': '_',
					'=': '+',
					'[': '{',
					']': '}',
					'\\': '|',
					';': ':',
					"'": '"',
					',': '<',
					'.': '>',
					'/': '?',
				}
				return shiftMap[key] || key
			}
			return key
		}

		if (isShiftPressed || isCapsLock) {
			return key.toUpperCase()
		}
		return key.toLowerCase()
	}

	function getKeyClass(key: string): string {
		let classes: string[] = ['key']

		if (
			[
				'Shift',
				'Caps',
				'Tab',
				'Enter',
				'Backspace',
				'Esc',
				'Ctrl',
				'Alt',
				'Win',
				'Fn',
			].includes(key)
		) {
			classes.push('key--special')
		}

		if (key === 'Space') {
			classes.push('key--space')
		}

		if (key.startsWith('F') && key.length <= 3) {
			classes.push('key--function')
		}

		if (key.length > 1 && !['Space', 'Enter'].includes(key)) {
			classes.push('key--modifier')
		}

		return classes.join(' ')
	}

	function getKeyDisplay(key: string): string {
		const specialDisplays: Record<string, string> = {
			Space: 'SPACE',
			Enter: '⏎',
			Backspace: '⌫',
			Shift: '⇧',
			Caps: '⇪',
			Tab: '⇥',
			Ctrl: 'CTRL',
			Alt: 'ALT',
			Win: 'WIN',
			Fn: 'FN',
			Esc: 'ESC',
		}

		return specialDisplays[key] || key
	}
</script>

<div class="sci-fi-keyboard">
	{#each sciFiLayout as row, i}
		<div class="keyboard-row row-{i}">
			{#each row as key}
				<button
					class={getKeyClass(key)}
					class:active={$activeKeys.has(key)}
					on:click={() => handleKeyPress(key)}
					on:mousedown|preventDefault
				>
					<span class="key-label">
						{getKeyDisplay(key)}
					</span>
					{#if $activeKeys.has(key)}
						<span class="key-glow"></span>
					{/if}
				</button>
			{/each}
		</div>
	{/each}
</div>

<style>
	.sci-fi-keyboard {
		font-family: 'Courier New', monospace;
		padding: 12px;
		user-select: none;
		overflow: hidden;
		position: relative;
	}

	.keyboard-row {
		display: flex;
		justify-content: center;
		margin-bottom: 6px;
	}

	.keyboard-row:last-child {
		margin-bottom: 0;
	}

	.key {
		position: relative;
		min-width: 65px;
		height: 70px;
		margin: 0 5px;
		padding: 0 10px;
		background: rgba(20, 0, 0, 0.7);
		border: 1px solid rgba(255, 0, 51, 0.3);
		border-radius: 3px;
		color: #ff0033;
		font-size: 20px;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.1s ease;
		overflow: hidden;
	}

	.key:hover {
		background: rgba(50, 0, 0, 0.7);
	}

	.key:active,
	.key.active {
		background: rgba(255, 0, 51, 0.2);
		transform: translateY(1px);
	}

	.key--special {
		background: rgba(50, 0, 0, 0.7);
		min-width: 70px;
	}

	.key--space {
		min-width: 300px;
	}

	.key--modifier {
		background: rgba(70, 0, 0, 0.7);
	}

	.key--function {
		min-width: 50px;
		font-size: 12px;
	}

	.key-glow {
		position: absolute;
		top: -10px;
		left: -10px;
		right: -10px;
		bottom: -10px;
		background: radial-gradient(circle, #ff0033 0%, transparent 70%);
		opacity: 0;
		animation: key-glow 0.3s ease-out;
	}

	@keyframes key-glow {
		0% {
			opacity: 0.7;
			transform: scale(0.5);
		}
		100% {
			opacity: 0;
			transform: scale(1.5);
		}
	}

	.key-label {
		position: relative;
		z-index: 1;
		text-shadow: 0 0 5px #ff0033;
	}

	.row-0 .key {
		height: 60px;
	}
	.row-5 .key {
		height: 60px;
	}
</style>
