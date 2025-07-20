<script lang="ts">
	import { onMount } from 'svelte'
	import CpuMonitor from '../components/shared/cpu-monitor.svelte'

	import LogoUi from '../components/shared/logo-ui.svelte'
	import Old from '../components/shared/old.svelte'
	import ProcessMonitor from '../components/shared/process-monitor.svelte'
	import RamMonitor from '../components/shared/ram-monitor.svelte'
	import StartDialog from '../components/shared/start-dialog.svelte'
	import Terminal from '../components/shared/terminal.svelte'
	import AudioGenerator from '../components/shared/audio-generator.svelte'
	
	let showDialog = false
	let showMainInterface = false
	let showLogo = true

	onMount(() => {
		setTimeout(() => {
			showLogo = false
			showDialog = true
		}, 4500)
	})

	function handleDialogComplete() {
		showDialog = false
		showMainInterface = true
	}
</script>

<Old>
	{#if showLogo}
		<LogoUi duration={4000} typingSpeed={500} logoText="RIZO" />
	{:else if showDialog}
		<StartDialog showDialog onComplete={handleDialogComplete} />
	{:else if showMainInterface}
		<div class="dashboard">
			<AudioGenerator/>

			<div class="main-terminal">
				<Terminal />
			</div>

			<div class="cpu-monitor">
				<CpuMonitor />
			</div>

			<div class="ram-monitor">
				<RamMonitor />
			</div>

			<div class="process-monitor">
				<ProcessMonitor />
			</div>

			<!-- <div class="ping-monitor">
                <PingMonitor />
            </div> -->
		</div>
	{/if}
</Old>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		background-color: #0a0a0a;
		color: #00ff00;
		font-family: 'Courier New', monospace;
		overflow: hidden;
		height: 100vh;
	}

	.dashboard {
		position: relative;
		width: 100%;
		height: 100vh;
		background-color: rgba(0, 0, 0, 0.85);
		display: grid;
		grid-template-areas:
            "cpu terminal terminal ram"
            "process terminal terminal ."
            ". terminal terminal .";
		grid-template-columns: 1fr 1fr 1fr 1fr;
		grid-template-rows: auto auto 1fr;
		gap: 10px;
		padding: 10px;
		box-sizing: border-box;
	}

	.main-terminal {
		grid-area: terminal;
		height: 90vh;
		border: 1px solid #00ff00;
		border-radius: 4px;
		box-shadow: 0 0 15px rgba(0, 255, 0, 0.3);
		background-color: rgba(10, 10, 10, 0.9);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.cpu-monitor {
		grid-area: cpu;
		border: 1px solid #00ff00;
		border-radius: 4px;
		padding: 8px;
		background-color: rgba(10, 10, 10, 0.8);
		height: fit-content;
		box-shadow: 0 0 10px rgba(0, 255, 0, 0.2);
	}

	.ram-monitor {
		grid-area: ram;
		border: 1px solid #00ff00;
		border-radius: 4px;
		padding: 8px;
		background-color: rgba(10, 10, 10, 0.8);
		height: fit-content;
		box-shadow: 0 0 10px rgba(0, 255, 0, 0.2);
	}

	.process-monitor {
		grid-area: process;
		border: 1px solid #00ff00;
		border-radius: 4px;
		padding: 8px;
		background-color: rgba(10, 10, 10, 0.8);
		box-shadow: 0 0 10px rgba(0, 255, 0, 0.2);
	}

	:global(.panel-header) {
		background-color: rgba(0, 30, 0, 0.7);
		padding: 5px 10px;
		margin: -8px -8px 8px -8px;
		font-weight: bold;
		border-bottom: 1px solid #00ff00;
		text-shadow: 0 0 5px #00ff00;
		font-size: 0.9em;
	}


	.cpu-monitor:hover,
	.ram-monitor:hover,
	.process-monitor:hover,
	.main-terminal:hover {
		box-shadow: 0 0 20px rgba(0, 255, 0, 0.4);
		border-color: #00ff99;
	}


	@keyframes pulse-glow {
		0% { box-shadow: 0 0 10px rgba(0, 255, 0, 0.2); }
		50% { box-shadow: 0 0 15px rgba(0, 255, 0, 0.5); }
		100% { box-shadow: 0 0 10px rgba(0, 255, 0, 0.2); }
	}

	.main-terminal {
		animation: pulse-glow 4s infinite;
	}
</style>
