<script lang="ts">
	import { onMount } from 'svelte'
	import CpuMonitor from '../components/shared/cpu-monitor.svelte'

	import LogoUi from '../components/shared/logo-ui.svelte'
	import Old from '../components/shared/old.svelte'
	import ProcessMonitor from '../components/shared/process-monitor.svelte'
	import RamMonitor from '../components/shared/ram-monitor.svelte'
	import StartDialog from '../components/shared/start-dialog.svelte'
	import Terminal from '../components/shared/terminal.svelte'

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
			<div class="panel cpu-panel">
				<div class="panel-header">CPU Monitor</div>
				<CpuMonitor />
			</div>

			<div class="panel memory-panel">
				<div class="panel-header">Memory Monitor</div>
				<RamMonitor />
			</div>

			<div class="panel ping-panel">
				<div class="panel-header">Network Ping</div>
				<!-- <PingMonitor /> -->
			</div>

			<div class="panel process-panel">
				<div class="panel-header">Processes</div>
				<ProcessMonitor />
			</div>

			<div class="panel terminal-panel">
				<Terminal />
			</div>
		</div>
	{/if}
</Old>

<style>
</style>
