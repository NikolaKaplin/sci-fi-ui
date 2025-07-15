<script lang="ts">
	import { listen } from '@tauri-apps/api/event'
	import { onMount } from 'svelte'

	let processes: Array<{
		name: string
		pid: number
		cpu: number
		memory: number
	}> = []

	onMount(() => {
		const unlisten = listen<
			Array<{
				name: string
				pid: number
				cpu: number
				memory: number
			}>
		>('process_update', event => {
			processes = event.payload.sort((a, b) => b.cpu - a.cpu).slice(0, 10)
		})

		return () => {
			unlisten.then(f => f())
		}
	})
</script>

<table class="process-table">
	<thead>
		<tr>
			<th>Name</th>
			<th>PID</th>
			<th>CPU %</th>
			<th>Memory</th>
		</tr>
	</thead>
	<tbody>
		{#each processes as proc}
			<tr>
				<td>{proc.name}</td>
				<td>{proc.pid}</td>
				<td class="cpu-cell">{proc.cpu.toFixed(1)}</td>
				<td class="memory-cell">{(proc.memory / 1024 / 1024).toFixed(1)} MB</td>
			</tr>
		{/each}
	</tbody>
</table>

<style>
	.process-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.9em;
	}

	.process-table th {
		text-align: left;
		padding: 5px 10px;
		background: #002200;
		color: #00ff00;
	}

	.process-table td {
		padding: 5px 10px;
		border-bottom: 1px solid #003300;
	}

	.process-table tr:hover {
		background: #002200;
	}

	.cpu-cell {
		color: #00ff00;
	}

	.memory-cell {
		color: #00aaff;
	}
</style>
