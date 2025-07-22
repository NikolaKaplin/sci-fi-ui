<script lang="ts">
	import AiButton from '../components/shared/ai/ai-button.svelte'
	import AudioGenerator from '../components/shared/audio-generator.svelte'
	import CpuRamMonitor from '../components/shared/cpu-ram-monitor.svelte'

	import FileExplorer from '../components/shared/file-explorer.svelte'
	import Keyboard from '../components/shared/keyboard.svelte'
	import LogoUi from '../components/shared/logo-ui.svelte'
	import ProcessMonitor from '../components/shared/process-monitor.svelte'
	import StartDialog from '../components/shared/start-dialog.svelte'
	import Terminal from '../components/shared/terminal.svelte'

	let showDialog = false
	let showMainInterface = true
	let showLogo = false
	let showDialog2 = false
	// onMount(() => {
	// 	setTimeout(() => {
	// 		showLogo = false
	// 		showDialog = true
	// 	}, 4500)
	// })

	function handleDialogComplete() {
		showDialog = false
		showMainInterface = true
	}

	let text = ''
	let syncKeyboard = true

	function handleKeyPress(event: { detail: { key: any; value: any } }) {
		const { key, value } = event.detail

		if (key === 'Backspace') {
			text = text.slice(0, -1)
		} else if (key === 'Enter') {
			text += '\n'
		} else if (value) {
			text += value
		}
	}
</script>

{#if showLogo}
	<LogoUi duration={4000} typingSpeed={500} logoText="RIZO" />
{:else if showDialog}
	<StartDialog showDialog onComplete={handleDialogComplete} />
{:else if showMainInterface}
	<div class="min-h-screen">
		<div class=" flex gap-2 p-2">
			<div>
				<div class="flex gap-1.5">
					<CpuRamMonitor />
				</div>
				<ProcessMonitor />
			</div>
			<!-- <AudioGenerator /> -->
			<Terminal />
			<AudioGenerator />
			<AiButton />
		</div>
		<div class="flex w-full justify-between">
			<div class="w-[40vw]"><FileExplorer /></div>
			<Keyboard
				on:keypress={handleKeyPress}
				syncWithPhysicalKeyboard={syncKeyboard}
			/>
		</div>
	</div>
{/if}
