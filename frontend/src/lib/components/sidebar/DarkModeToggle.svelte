<script lang="ts">
	import { Moon, Sun } from 'lucide-svelte'
	import { twMerge } from 'tailwind-merge'

	import DarkModeObserver from '../DarkModeObserver.svelte'

	export let darkMode: boolean = document.documentElement.classList.contains('dark')
	export let forcedDarkMode: boolean = true

	function onThemeChange() {
		if (document.documentElement.classList.contains('dark')) {
			darkMode = true
		} else {
			darkMode = false
		}
	}

	export function toggle() {
		if (!document.documentElement.classList.contains('dark')) {
			document.documentElement.classList.add('dark')
			window.localStorage.setItem('dark-mode', 'dark')
		} else {
			document.documentElement.classList.remove('dark')
			window.localStorage.setItem('dark-mode', 'light')
		}
	}
</script>

<button
	class={twMerge(
		'text-2xs text-white m-1 p-2 rounded-lg flex flex-row gap-2 justify-center hover:bg-gray-600',
		forcedDarkMode ? 'text-white hover:bg-gray-600' : 'text-primary hover:bg-surface-hover'
	)}
	on:click={toggle}
>
	{#if darkMode}
		<Sun class="w-4 h-4" />
	{:else}
		<Moon class="w-4 h-4" />
	{/if}
</button>

<DarkModeObserver on:change={onThemeChange} />
