import { createSignal, Suspense, useTransition } from "solid-js"

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

	let i = 0
	setInterval(() => {
		i++
		if (i == themes.length) {
			i = 0
		}
		setTheme(themes[i])
	}, 6000)

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
				{/* Titlebar */}
				<div
					data-tauri-drag-region
					class="backdrop-brightness-105 hover:backdrop-brightness-[1.15] fixed z-50 grid w-full h-6 duration-500">
					<p
						data-tauri-drag-region
						class="text-white/80 self-center text-xs text-center">
						Iris Installer v0.0.0
					</p>
				</div>
				{/* Main content container */}
				<div class="backdrop-blur-sm bg-gradient-to-t from-zinc-800 via-zinc-800/70 to-transparent grid h-full grid-cols-1 px-12 py-16">
					{/* Text content container */}
					<div class="self-center space-y-2">
						<header class="text-5xl font-bold text-center text-gray-100">
							<img
								class="inline w-12 h-12 mr-2 align-bottom"
								src="../app-icon.png"
							/>
							Install Iris
						</header>
						<p class="text-center leading-snug">
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
						<p>Use Qulit instead of Fabric</p>
					</div>
					<div class="self-end space-y-2">
						<button class="hover:brightness-125 border-zinc-500 text-zinc-400 w-full py-1.5 font-semibold duration-500 border-2 rounded">
							Iris & Sodium for 1.19.3
						</button>
						<button
							class="hover:brightness-125 text-zinc-800 w-full py-1.5 font-semibold duration-500 rounded"
							style={{ "background-color": theme().color }}>
							Install
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 20 20"
								fill="currentColor"
								class="inline w-4 h-4 align-text-bottom">
								<path
									fill-rule="evenodd"
									d="M3 10a.75.75 0 01.75-.75h10.638L10.23 5.29a.75.75 0 111.04-1.08l5.5 5.25a.75.75 0 010 1.08l-5.5 5.25a.75.75 0 11-1.04-1.08l4.158-3.96H3.75A.75.75 0 013 10z"
									clip-rule="evenodd"
								/>
							</svg>
						</button>
						{/* Links section */}
						<div class="flex justify-center gap-6">
							{links.map((i) => (
								<a
									class="hover:underline hover:brightness-125 font-semibold duration-500"
									href={i.href}
									target="_blank"
									style={{ color: theme().color }}>
									{i.text}
									<svg
										xmlns="http://www.w3.org/2000/svg"
										viewBox="0 0 20 20"
										fill="currentColor"
										class="inline w-4 h-4 align-text-bottom">
										<path
											fill-rule="evenodd"
											d="M5.22 14.78a.75.75 0 001.06 0l7.22-7.22v5.69a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75h-7.5a.75.75 0 000 1.5h5.69l-7.22 7.22a.75.75 0 000 1.06z"
											clip-rule="evenodd"
										/>
									</svg>
								</a>
							))}
						</div>
					</div>
				</div>
			</div>
		</>
	)
}
