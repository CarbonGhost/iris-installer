import { createResource, createSignal, Suspense } from "solid-js"
import { createStore } from "solid-js/store"

import ArrowRight from "./components/icons/ArrowRight"
import Titlebar from "./components/Titlebar"
import ExternalLink from "./components/ExternalLink"
import { Version } from "./types"
import { invoke } from "@tauri-apps/api"

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
		generate_profile: boolean
	}>({ iris: false, version: versions()?.[0], generate_profile: true })

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
						<p>Install Iris and Sodium</p>
						<p>Always check for new versions of the mod</p>
						<p>Automatically detect game directory</p>
					</div>
					<div class="self-end space-y-2">
						<select
							id="versionSelect"
							name="versionSelect"
							disabled={versions.loading}
							class="disabled:opacity-30 bg-transparent text-center appearance-none hover:brightness-125 border-zinc-500 text-zinc-400 w-full py-1.5 font-semibold duration-500 border-2 rounded"
							onClick={(e) => {
								const option = (e.target as HTMLInputElement)
									.value
								const version = versions()?.find(
									(i) => option == i.irisVersion
								)
								setInstallConfig({ version: version })
							}}>
							<Suspense fallback={<option>Loading...</option>}>
								{versions()?.map((i) => (
									<option value={i.irisVersion}>
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
								invoke("download_mods", installConfig)
								
								console.log("installing with:")
								console.log(installConfig)
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
