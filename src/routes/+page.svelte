<script lang="ts">
	import CpuMonitor from '../components/shared/cpu-monitor.svelte'
	import PingMonitor from '../components/shared/ping-monitor.svelte'
	import ProcessMonitor from '../components/shared/process-monitor.svelte'
	import RamMonitor from '../components/shared/ram-monitor.svelte'
	import Terminal from '../components/shared/terminal.svelte'
</script>

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
		<PingMonitor />
	</div>

	<div class="panel process-panel">
		<div class="panel-header">Processes</div>
		<ProcessMonitor />
	</div>

	<div class="panel terminal-panel">
		<Terminal />
	</div>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		background-color: #0c0c0c;
		color: #00ff00;
		font-family: 'Courier New', monospace;
		height: 100vh;
		overflow: hidden;
	}

	.dashboard {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		grid-template-rows: auto auto 1fr;
		gap: 10px;
		height: 100vh;
		padding: 10px;
		box-sizing: border-box;
	}

	.panel {
		background: rgba(20, 20, 20, 0.8);
		border: 1px solid #00aa00;
		border-radius: 5px;
		padding: 10px;
		overflow: hidden;
	}

	.panel-header {
		color: #00ff00;
		margin-bottom: 10px;
		font-size: 1.2em;
		border-bottom: 1px solid #00aa00;
		padding-bottom: 5px;
	}

	.cpu-panel {
		grid-column: 1;
		grid-row: 1;
	}

	.memory-panel {
		grid-column: 2;
		grid-row: 1;
	}

	.ping-panel {
		grid-column: 3;
		grid-row: 1;
	}

	.process-panel {
		grid-column: 1 / span 3;
		grid-row: 2;
		height: 200px;
	}

	.terminal-panel {
		grid-column: 1 / span 4;
		grid-row: 3;
	}
</style>
