<script lang="ts">
	import { driver, type Driver, type DriveStep } from 'driver.js'
	import 'driver.js/dist/driver.css'
	import { createEventDispatcher } from 'svelte'
	import { updateProgress } from '$lib/tutorialUtils'
	import { ignoredTutorials } from './ignoredTutorials'

	export let index: number = 0
	export let name: string = 'action'
	export let tainted: boolean = false

	type Options = {
		indexToInsertAt?: number
		skipStepsCount?: number
	}

	export let getSteps: (driver: Driver, options?: Options | undefined) => DriveStep[] = () => []

	const dispatch = createEventDispatcher()

	function createTutorialButton(text: string, onClick: () => void) {
		const button = document.createElement('button')
		button.innerHTML = text
		button.addEventListener('click', onClick)
		button.setAttribute(
			'class',
			'border px-2 py-1 !text-xs font-normal rounded-md hover:bg-surface-hover transition-all flex items-center'
		)
		return button
	}

	export const runTutorial = (options?: Options | undefined) => {
		if (tainted) {
			dispatch('error', { detail: name })
			return
		}

		const tutorial = driver({
			showProgress: true,
			allowClose: true,
			onPopoverRender: (popover, { config, state }) => {
				if (state.activeIndex == 0) {
					const skipThisButton = createTutorialButton('Mark this tutorial as completed', () => {
						updateProgress(index)
						tutorial.destroy()
					})

					const skipAllButton = createTutorialButton(
						'Mark&nbsp;<b>all</b>&nbsp;tutorials as completed',
						() => {
							dispatch('skipAll')
							tutorial.destroy()
						}
					)

					const popoverDescription = document.querySelector('#driver-popover-description')
					const div = document.createElement('div')

					div.setAttribute('class', 'flex flex-row gap-2 justify-between w-full pt-2')

					if (popoverDescription) {
						div.appendChild(skipAllButton)
						div.appendChild(skipThisButton)
						popoverDescription.appendChild(div)
					}
				}
			},
			onDestroyed: () => {
				if (!tutorial.hasNextStep()) {
					$ignoredTutorials = Array.from(new Set([...$ignoredTutorials, index]))
				}
			}
		})

		tutorial.setSteps(getSteps(tutorial, options))
		tutorial.drive()
	}
</script>
