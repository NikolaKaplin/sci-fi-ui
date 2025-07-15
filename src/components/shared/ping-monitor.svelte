<script lang="ts">
	import { listen } from '@tauri-apps/api/event'
	import { onMount } from 'svelte'

	let ping = 0
	let status = 'offline'

	onMount(() => {
		const unlisten = listen<{ latency: number }>('ping_update', event => {
			ping = event.payload.latency
			status = ping < 100 ? 'online' : ping < 300 ? 'unstable' : 'offline'
		})

		return () => {
			unlisten.then(f => f())
		}
	})
</script>

<div class="ping-container">
	<div class="ping-value">{ping} ms</div>
	<div class="ping-status {status}">
		{#if status === 'online'}
			Stable connection
		{:else if status === 'unstable'}
			Unstable connection
		{:else}
			Offline
		{/if}
	</div>
</div>

<style>
	.ping-container {
		display: flex;
		flex-direction: column;
		gap: 5px;
	}

	.ping-value {
		font-size: 1.5em;
		color: #00ff00;
	}

	.ping-status {
		font-size: 0.9em;
		color: #00aa00;
	}

	.online {
		color: #00ff00;
	}

	.unstable {
		color: #ffaa00;
	}

	.offline {
		color: #ff0000;
	}
</style>
