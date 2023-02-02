import { appWindow } from "@tauri-apps/api/window"
import { getVersion } from "@tauri-apps/api/app"
import { createResource } from "solid-js"

import X from "./icons/X"

export default function Titlebar() {
	const [version] = createResource(async () => await getVersion())

	return (
		<div
			data-tauri-drag-region
			class="text-white backdrop-brightness-105 hover:backdrop-brightness-[1.15] fixed z-50 grid w-full duration-500">
			<p
				data-tauri-drag-region
				class="self-center text-xs text-center my-1.5">
				Iris Installer v{version()}
			</p>
			<button
				class="fixed top-1.5 right-1.5"
				onClick={() => appWindow.close()}>
				<X />
			</button>
		</div>
	)
}
