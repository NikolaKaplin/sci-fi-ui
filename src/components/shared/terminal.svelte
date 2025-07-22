<script lang="ts">
	import { invoke } from '@tauri-apps/api/core'
	import { onDestroy, onMount } from 'svelte'
	import { Terminal } from 'xterm'
	import { FitAddon } from 'xterm-addon-fit'
	import 'xterm/css/xterm.css'

	let terminalElement: HTMLElement
	let term: Terminal
	const fitAddon = new FitAddon()
	invoke('get_sys_info')
	async function fitTerminal() {
		fitAddon.fit()
		await invoke<string>('async_resize_pty', {
			rows: term.rows,
			cols: term.cols,
		})
	}

	function writeToTerminal(data: string) {
		return new Promise<void>(resolve => {
			term.write(data, () => resolve())
		})
	}

	function writeToPty(data: string) {
		invoke('async_write_to_pty', { data })
	}

	function initShell() {
		invoke('async_create_shell').catch(error => {
			console.error('Error creating shell:', error)
		})
	}

	async function readFromPty() {
		const data = await invoke<string>('async_read_from_pty')
		if (data) {
			await writeToTerminal(data)
		}
		window.requestAnimationFrame(readFromPty)
	}

	onMount(() => {
		term = new Terminal({
			fontFamily: 'Jetbrains Mono, monospace',
			fontSize: 14,
			letterSpacing: 0.5,
			lineHeight: 1.2,
			cursorBlink: true,
			cursorStyle: 'block',
			theme: {
				foreground: '#00ff00',
				background: 'rgba(0, 10, 0, 0.7)',
				cursor: '#00ff00',
				cursorAccent: '#000',
				black: '#000000',
				red: '#ff0000',
				green: '#00ff00',
				yellow: '#ffff00',
				blue: '#0000ff',
				magenta: '#ff00ff',
				cyan: '#00ffff',
				white: '#ffffff',
				brightBlack: '#666666',
				brightRed: '#ff6666',
				brightGreen: '#66ff66',
				brightYellow: '#ffff66',
				brightBlue: '#6666ff',
				brightMagenta: '#ff66ff',
				brightCyan: '#66ffff',
				brightWhite: '#ffffff',
			},
			windowOptions: {
				fullscreenWin: true,
			},
		})

		term.loadAddon(fitAddon)
		term.open(terminalElement)
		term.onData(writeToPty)

		window.addEventListener('resize', fitTerminal)
		initShell()
		fitTerminal()
		window.requestAnimationFrame(readFromPty)
	})

	onDestroy(() => {
		if (term) {
			term.dispose()
		}
		window.removeEventListener('resize', fitTerminal)
	})
</script>

<div id="terminal" bind:this={terminalElement} class="cyber-terminal"></div>

<style>
	.cyber-terminal {
		width: 100%;
		height: 55vh;
	}

	@keyframes blink {
		0%,
		49% {
			opacity: 1;
		}
		50%,
		100% {
			opacity: 0.3;
		}
	}

	:global(.xterm-viewport::-webkit-scrollbar-track) {
		background: rgba(0, 255, 0, 0.1);
	}

	:global(.xterm-viewport::-webkit-scrollbar-thumb) {
		background: rgba(0, 255, 0, 0.3);
		border-radius: 4px;
	}

	:global(.xterm-viewport::-webkit-scrollbar-thumb:hover) {
		background: rgba(0, 255, 0, 0.5);
	}
</style>
