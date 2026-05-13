<script>
	let { class: className, toggled } = $props();
</script>

<svg
	class:toggled
	class={className}
	viewBox="0 0 24 24"
	fill="none"
	xmlns="http://www.w3.org/2000/svg"
>
	<path d="M4 6L20 6" stroke-linecap="round"></path>
	<path d="M4 12L20 12" stroke-linecap="round"></path>
	<path d="M4 18L20 18" stroke-linecap="round"></path>
</svg>

<style>
	path {
		transition: transform 0.5s cubic-bezier(0.68, -0.6, 0.32, 1.6);
		transform-box: fill-box;
		transform-origin: center;
	}

	svg.toggled {
		path:nth-child(1) {
			transform: translateY(6px) rotate(45deg);
		}

		path:nth-child(2) {
			transform: scaleX(0%);
		}

		path:nth-child(3) {
			transform: translateY(-6px) rotate(-45deg);
		}
	}
</style>
