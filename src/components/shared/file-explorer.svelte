<script lang="ts">
	import { invoke } from '@tauri-apps/api/core'
	import { homeDir } from '@tauri-apps/api/path'
	import { message } from '@tauri-apps/plugin-dialog'
	import { onMount } from 'svelte'
	import { fade } from 'svelte/transition'

	interface FsItem {
		name: string
		path: string
		is_dir: boolean
		size: number
		modified: number
		icon?: string
		is_drive?: boolean
	}

	interface FsResponse {
		path: string
		items: FsItem[]
	}

	const icons = {
		folder: `<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 2h9a2 2 0 0 1 2 2z"/></svg>`,
		file: `<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><polyline points="13 2 13 9 20 9"/></svg>`,
		image: `<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>`,
		code: `<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>`,
		audio: `<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 18v-6a9 9 0 0 1 18 0v6"/><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"/></svg>`,
		video: `<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><polygon points="23 7 16 12 23 17 23 7"/><rect x="1" y="5" width="15" height="14" rx="2"/></svg>`,
		drive: `<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2" y="4" width="20" height="16" rx="2"/><path d="M2 8h20M6 4v4"/></svg>`,
	}

	let currentItems: FsItem[] = []
	let currentPath = ''
	let history: Array<{ path: string; items: FsItem[] }> = []
	let loading = false
	let error: string | null = null
	let availableDrives: FsItem[] = []

	onMount(async () => {
		try {
			loading = true
			const drives = await invoke<FsItem[]>('get_drives')
			availableDrives = drives.map(drive => ({
				...drive,
				icon: 'drive',
				is_drive: true,
			}))

			const home = await homeDir()
			if (home) {
				await navigateTo(home)
			} else if (availableDrives.length > 0) {
				await navigateTo(availableDrives[0].path)
			}
		} catch (err) {
			error = err instanceof Error ? err.message : String(err)
			await message(error, { title: 'Error' })
		} finally {
			loading = false
		}
	})

	async function navigateTo(path: string) {
		try {
			loading = true
			error = null

			const response = await invoke<FsResponse>('list_dir', { path })

			if (!response || !response.items) {
				throw new Error('Invalid response from file system')
			}

			history.push({
				path: currentPath,
				items: [...currentItems],
			})

			currentPath = response.path || ''
			currentItems = [
				...(path !== '/' && path !== '\\' && !path.match(/^[A-Za-z]:\\?$/i)
					? [
							{
								name: '..',
								path: await invoke<string>('get_parent_dir', { path }),
								is_dir: true,
								size: 0,
								modified: 0,
								icon: 'folder',
							},
						]
					: []),
				...(isRootPath(path) ? availableDrives : []),
				...response.items.map(item => ({
					...item,
					name: item.name || 'Unknown',
					path: item.path || '',
					is_dir: item.is_dir || false,
					size: item.size || 0,
					modified: item.modified || 0,
					icon: getIconType(item),
				})),
			]
		} catch (err) {
			error = err instanceof Error ? err.message : String(err)
			await message(error, { title: 'Error' })
		} finally {
			loading = false
		}
	}

	function isRootPath(path: string): boolean {
		return (
			path === '/' || path === '\\' || path.match(/^[A-Za-z]:\\?$/i) !== null
		)
	}

	async function goBack() {
		if (history.length === 0) return
		const lastState = history.pop()
		if (lastState) await navigateTo(lastState.path)
	}

	function getIconType(item: FsItem): string {
		if (item.is_drive) return 'drive'
		if (item.is_dir) return 'folder'

		const ext = item.name.split('.').pop()?.toLowerCase() || ''
		const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg']
		const audioExts = ['mp3', 'wav', 'ogg', 'flac', 'aac']
		const videoExts = ['mp4', 'avi', 'mov', 'mkv', 'webm']
		const codeExts = [
			'js',
			'ts',
			'html',
			'css',
			'json',
			'py',
			'rs',
			'go',
			'java',
			'cpp',
			'c',
			'h',
		]

		if (imageExts.includes(ext)) return 'image'
		if (audioExts.includes(ext)) return 'audio'
		if (videoExts.includes(ext)) return 'video'
		if (codeExts.includes(ext)) return 'code'
		return 'file'
	}

	async function openItem(item: FsItem) {
		if (!item) return
		try {
			if (item.is_dir || item.is_drive) {
				await navigateTo(item.path)
			} else if (item.path) {
				await invoke('open_file', { path: item.path })
			}
		} catch (err) {
			error = err instanceof Error ? err.message : String(err)
			await message(`Failed to open: ${error}`, { title: 'Error' })
		}
	}
</script>

<div class="file-explorer">
	<div class="header">
		<div class="path">PATH: {currentPath || '/'}</div>
		<div class="disk-info">DISK: 83% used</div>
	</div>

	{#if error}
		<div class="error" transition:fade>{error}</div>
	{/if}

	<div class="file-list-container">
		{#if loading}
			<div class="loading" transition:fade>LOADING...</div>
		{:else}
			<div class="file-grid">
				{#each currentItems as item, index (item.path)}
					<button
						class="file-item {item.is_dir ? 'folder' : 'file'} {item.is_drive
							? 'drive'
							: ''}"
						on:click={() => openItem(item)}
						style="animation-delay: {index * 50}ms"
					>
						<span class="icon"
							>{@html icons ? [item.icon ?? 'file'] : null}</span
						>
						<div class="name">{item.name}</div>
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	:global(::-webkit-scrollbar) {
		width: 12px;
		height: 12px;
	}

	:global(::-webkit-scrollbar-track) {
		background: rgba(0, 10, 0, 0.3);
		border-left: 1px solid rgba(0, 255, 0, 0.1);
	}

	:global(::-webkit-scrollbar-thumb) {
		background: linear-gradient(45deg, #00ff00, #00ff88);
		border-radius: 6px;
		border: 2px solid rgba(0, 20, 0, 0.8);
		box-shadow:
			0 0 5px rgba(0, 255, 0, 0.8),
			inset 0 0 10px rgba(0, 255, 0, 0.5);
	}

	:global(::-webkit-scrollbar-thumb:hover) {
		background: linear-gradient(45deg, #00ff88, #00ffcc);
		box-shadow:
			0 0 10px rgba(0, 255, 128, 0.9),
			inset 0 0 15px rgba(0, 255, 200, 0.6);
	}

	:global(::-webkit-scrollbar-corner) {
		background: rgba(0, 15, 0, 0.6);
	}

	@keyframes scrollbar-pulse {
		0%,
		100% {
			box-shadow:
				0 0 5px rgba(0, 255, 0, 0.8),
				inset 0 0 10px rgba(0, 255, 0, 0.5);
		}
		50% {
			box-shadow:
				0 0 10px rgba(0, 255, 128, 0.9),
				inset 0 0 15px rgba(0, 255, 200, 0.8);
		}
	}

	:global(::-webkit-scrollbar-thumb:active) {
		animation: scrollbar-pulse 1.5s infinite;
	}

	.file-explorer {
		width: 100%;
		height: 450px;

		display: flex;
		flex-direction: column;
		font-family: 'Courier New', monospace;
		color: #00ff00;
		overflow: hidden;
	}

	.header {
		display: flex;
		align-items: center;

		border-bottom: 1px solid #00ff00;
	}

	.path {
		flex-grow: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.disk-info {
		margin-left: 15px;
		color: #ffff00;
	}

	.file-list-container {
		flex-grow: 1;
		overflow-y: auto;
		padding: 8px;
	}

	.file-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(90px, 1fr));
		gap: 10px;
	}

	.file-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 8px;
		cursor: pointer;
		border-radius: 4px;
		text-align: center;
		opacity: 0;
		animation: fadeIn 0.3s forwards;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.file-item:hover {
		background-color: rgba(0, 255, 0, 0.1);
	}

	.icon {
		width: 40px;
		height: 40px;
		margin-bottom: 5px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.name {
		font-size: 12px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 80px;
	}

	.folder {
		color: #5eb3ff;
	}

	.drive {
		color: #ff9e5e;
		font-weight: bold;
	}

	.error {
		color: #ff5555;
		padding: 8px;
		margin: 8px;
		background-color: rgba(50, 0, 0, 0.5);
		border: 1px solid #ff5555;
	}

	.loading {
		padding: 15px;
		text-align: center;
		color: #888;
	}
</style>
