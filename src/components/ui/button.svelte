<script lang="ts">
	export let onClick: (() => void) | undefined = undefined
</script>

<button onclick={onClick}><slot /></button>

<style lang="scss">
	button {
		position: relative;
		padding: 8px 16px;
		background: transparent;
		border: none;
		color: #0ff;
		font-family: 'Titillium Web', sans-serif;
		text-transform: uppercase;
		letter-spacing: 2px;
		cursor: pointer;
		outline: none;
		overflow: hidden;
		transition: all 0.3s;

		&::before {
			content: '';
			position: absolute;
			top: 0;
			left: 0;
			width: 100%;
			height: 100%;
			border: 2px solid #0ff;
			box-sizing: border-box;
			transition: all 0.3s;
		}

		&:hover {
			text-shadow: 0 0 10px #0ff;

			&::before {
				border-color: #3ff;
				box-shadow: 0 0 10px #0ff;
			}
		}
	}
</style>
