import { JSXElement } from "solid-js"

import ArrowTopRight from "./icons/ArrowTopRight"

export default function ExternalLink(props: {
	children: JSXElement
	href: string
	color: string
}) {
	return (
		<a
			class="hover:underline hover:brightness-125 font-semibold duration-500"
			href={props.href}
			target="_blank"
			style={{ color: props.color }}>
			{props.children}
			<ArrowTopRight />
		</a>
	)
}
