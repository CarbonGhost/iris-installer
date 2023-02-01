import { createSignal, useTransition } from "solid-js"

export default function App() {
	const themes: { bg: string; color: string }[] = [
		{
			bg: "birch",
			color: "#8ca641"
		},
		{
			bg: "geode",
			color: "#793cb9"
		},
		{
			bg: "village",
			color: "#a3ba3f"
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
	}, 5000)

	return (
		<>
			{/* Background image container */}
			<div
				class="content-between h-screen text-sm duration-1000 bg-no-repeat bg-cover"
				style={{
					"background-image": `url('../src/assets/backgrounds/${
						theme().bg
					}.png')`
				}}>
				<div
					data-tauri-drag-region
					class="mix-blend-overlay bg-white/20 hover:bg-white/40 fixed z-50 w-full h-6 duration-500"
				/>
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
						<p class="text-center text-gray-200">
							Welcome to the Iris Installer, this program will get
							you up and running with next-gen Minecraft shaders
							in minutes. Simply click "Install" to use our
							sensible defaults, or customize your installation if
							you are a power user.
						</p>
					</div>
					<div class="gap-y-2 flex flex-col self-center text-center text-gray-200">
						<p>Advanced install option</p>
						<p>Advanced install option</p>
						<p>Advanced install option</p>
						<p>Advanced install option</p>
					</div>
					<div class="self-end space-y-2">
						<button class="hover:brightness-125 border-zinc-500 text-zinc-400 w-full py-1.5 font-semibold duration-500 border-2 rounded">
							Iris for 1.19.3
						</button>
						<button
							class="hover:brightness-125 text-zinc-800 w-full py-1.5 font-semibold duration-500 rounded"
							style={{ "background-color": theme().color }}>
							Install
						</button>
						<div class="flex justify-center gap-6">
							<a
								class="hover:underline hover:brightness-125 duration-500"
								href="https://discord.gg/the-iris-project-774352792659820594"
								target="_blank"
								style={{ color: theme().color }}>
								Get Support
							</a>
							<a
								class="hover:underline hover:brightness-125 duration-500"
								href="https://irisshaders.net/"
								target="_blank"
								style={{ color: theme().color }}>
								Website
							</a>
							<a
								class="hover:underline hover:brightness-125 duration-500"
								href="https://github.com/IrisShaders"
								target="_blank"
								style={{ color: theme().color }}>
								GitHub
							</a>
						</div>
					</div>
				</div>
			</div>
		</>
	)
}
