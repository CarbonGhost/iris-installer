export type Meta = {
	betaVersionSnippet: string
	hasBeta: boolean
	versions: Version[]
}

export type Version = {
	name: string
	irisVersion: string
	sodiumVersion: string
	outdated: boolean
	snapshot: boolean
}
