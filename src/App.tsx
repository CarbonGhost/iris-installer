import {
	createRenderEffect,
	createResource,
	createSignal,
	JSXElement,
	Suspense
} from "solid-js"
import { createStore } from "solid-js/store"

import ArrowRight from "./components/icons/ArrowRight"
import Titlebar from "./components/Titlebar"
import ExternalLink from "./components/ExternalLink"
import { Version } from "./types"
import { invoke } from "@tauri-apps/api"
import { listen } from "@tauri-apps/api/event"

export default function App() {
	const themes: { bg: string; color: string }[] = [
		{
			bg: "birch",
			color: "#84b641"
		},
		{
			bg: "geode",
			color: "#793db7"
		},
		{
			bg: "village",
			color: "#b5c733"
		},
		{
			bg: "lanterns",
			color: "#b7bfc2"
		},
		{
			bg: "sunset",
			color: "#db8a1e"
		},
		{
			bg: "redstone",
			color: "#c13232"
		},
		{
			bg: "japan",
			color: "#a8c3de"
		}
	]
	const links: { text: string; href: string }[] = [
		{
			text: "Get Support",
			href: "https://discord.gg/the-iris-project-774352792659820594"
		},
		{
			text: "Website",
			href: "https://irisshaders.net"
		},
		{
			text: "GitHub",
			href: "https://github.com/IrisShaders"
		}
	]
	const [theme, setTheme] = createSignal(themes[0])
	const [versions] = createResource<Version[]>(async () =>
		invoke("versions", {
			outdated: false,
			snapshot: false,
			meta: await invoke("fetch_meta")
		})
	)
	const [installConfig, setInstallConfig] = createStore<{
		iris: boolean
		version: Version | undefined
		generateProfile: true
	}>({ iris: true, version: versions()?.[0], generateProfile: true })

	let versionSelect: any
	let irisSodiumCheck: any
	let generateProfileCheck: any
	let customDirectoryCheck: any

	let i = 0
	setInterval(() => {
		i++
		if (i == themes.length) {
			i = 0
		}
		setTheme(themes[i])
	}, 5000)

	return (
		<>
			{/* Background image container */}
			<div
				class="content-between h-screen text-sm text-gray-200 duration-1000 ease-in-out bg-no-repeat bg-cover"
				style={{
					"background-image": `url('../src/assets/backgrounds/${
						theme().bg
					}.png')`
				}}>
				<Titlebar />
				{/* Main content container */}
				<div class="backdrop-blur-sm bg-gradient-to-t from-zinc-800 via-zinc-800/70 to-transparent grid h-full grid-cols-1 px-12 py-16">
					{/* Text content container */}
					<div class="self-center space-y-2">
						<header class="text-5xl font-bold text-center text-gray-100">
							<img
								class="inline w-12 h-12 mr-2 align-bottom"
								src="../src-tauri/icons/Square310x310Logo.png"
							/>
							Install Iris
						</header>
						<p class="leading-snug text-center">
							Welcome to the Iris Installer, this program will
							allow you to get up and running with next-gen
							Minecraft shaders in minutes. Unless you know what
							you're doing, all you need to do is click install.
						</p>
					</div>
					<div class="gap-y-2 w-fit flex flex-col self-center mx-auto">
						<label>
							<input
								class="bg-zinc-800 border-zinc-700 checked:bg-zinc-400 w-4 h-4 mr-2 align-middle duration-100 border rounded-full appearance-none"
								ref={irisSodiumCheck as HTMLInputElement}
								checked={installConfig.iris}
								type="checkbox"
							/>
							Install Iris and Sodium
						</label>
						<label>
							<input
								class="bg-zinc-800 border-zinc-700 checked:bg-zinc-400 w-4 h-4 mr-2 align-middle duration-100 border rounded-full appearance-none"
								ref={generateProfileCheck as HTMLInputElement}
								checked={installConfig.generateProfile}
								type="checkbox"
							/>
							Generate a new profile with Quilt
						</label>
						<label>
							<input
								class="bg-zinc-800 border-zinc-700 checked:bg-zinc-400 w-4 h-4 mr-2 align-middle duration-100 border rounded-full appearance-none"
								ref={customDirectoryCheck as HTMLInputElement}
								type="checkbox"
							/>
							Use custom install directory{" "}
							<a
								class="hover:underline hover:brightness-125 font-semibold duration-500"
								style={{ color: theme().color }}
								href="#">
								C:/temp/placeholder
							</a>
						</label>
					</div>
					<div class="self-end space-y-2">
						<select
							id="versionSelect"
							name="versionSelect"
							disabled={versions.loading}
							class="focus:border-zinc-400 focus:outline-none disabled:opacity-30 bg-transparent text-center appearance-none hover:brightness-125 border-zinc-500 text-zinc-400 focus:text-zinc-300 w-full py-1.5 font-semibold duration-500 border-2 rounded"
							ref={versionSelect}>
							<Suspense fallback={<option>Loading...</option>}>
								{versions()?.map((i) => (
									<option
										class="bg-zinc-800 text-zinc-200"
										value={i.irisVersion}>
										{i.name}
									</option>
								))}
							</Suspense>
						</select>
						<button
							class="hover:brightness-125 text-zinc-800 w-full py-1.5 font-semibold duration-500 rounded disabled:opacity-30"
							style={{ "background-color": theme().color }}
							disabled={versions.loading}
							onClick={() => {
								let option = (versionSelect as HTMLInputElement)
									.value
								const version = versions()?.find(
									(i) => option == i.irisVersion
								)
								setInstallConfig({ version: version })

								invoke("download_mods", installConfig)
							}}>
							Install
							<ArrowRight />
						</button>
						<div class="flex justify-center gap-6">
							{links.map((i) => (
								<ExternalLink
									href={i.href}
									color={theme().color}>
									{i.text}
								</ExternalLink>
							))}
						</div>
					</div>
				</div>
			</div>
		</>
	)
}
