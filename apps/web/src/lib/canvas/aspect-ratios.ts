export type AspectRatioId = "16:9" | "9:16" | "1:1" | "4:5";

export interface AspectRatioPreset {
	id: AspectRatioId;
	label: string;
	description: string;
	width: number;
	height: number;
}

export const ASPECT_RATIO_PRESETS: AspectRatioPreset[] = [
	{
		id: "16:9",
		label: "16:9",
		description: "YouTube / Landscape",
		width: 1920,
		height: 1080,
	},
	{
		id: "9:16",
		label: "9:16",
		description: "TikTok / Reels / Shorts",
		width: 1080,
		height: 1920,
	},
	{
		id: "1:1",
		label: "1:1",
		description: "Square / Instagram",
		width: 1080,
		height: 1080,
	},
	{
		id: "4:5",
		label: "4:5",
		description: "Portrait / Feed",
		width: 1080,
		height: 1350,
	},
];

export function getAspectRatioPreset(id: AspectRatioId): AspectRatioPreset | undefined {
	return ASPECT_RATIO_PRESETS.find((preset) => preset.id === id);
}

export function getAspectRatioFromDimensions(
	width?: number,
	height?: number,
): AspectRatioId {
	if (!width || !height) return "16:9";
	const ratio = width / height;

	if (Math.abs(ratio - 16 / 9) < 0.05) return "16:9";
	if (Math.abs(ratio - 9 / 16) < 0.05) return "9:16";
	if (Math.abs(ratio - 1) < 0.05) return "1:1";
	if (Math.abs(ratio - 4 / 5) < 0.05) return "4:5";

	return "16:9";
}
