import type { SubtitleCue, SubtitleStyleOverrides } from "@/lib/subtitles/types";

export interface CaptionPreset {
	id: string;
	name: string;
	description: string;
	preview: {
		textColor: string;
		bgColor: string;
		badgeText: string;
	};
	style: SubtitleStyleOverrides;
}

export const CAPTION_PRESETS: CaptionPreset[] = [
	{
		id: "mrbeast-yellow",
		name: "MrBeast Bold Yellow",
		description: "High-energy bold yellow text with high contrast black backing",
		preview: {
			textColor: "#FFE600",
			bgColor: "#000000",
			badgeText: "VIRAL",
		},
		style: {
			fontSize: 7,
			fontFamily: "Inter",
			fontWeight: "bold",
			color: "#FFE600",
			textAlign: "center",
			background: {
				enabled: true,
				color: "#000000",
				paddingX: 18,
				paddingY: 10,
				cornerRadius: 8,
			},
			placement: {
				verticalAlign: "bottom",
				marginVerticalRatio: 0.12,
			},
		},
	},
	{
		id: "neon-glow",
		name: "Electric Cyan Glow",
		description: "Vibrant streamer aesthetic with glowing cyan accents",
		preview: {
			textColor: "#00FFFF",
			bgColor: "#0F172A",
			badgeText: "STREAM",
		},
		style: {
			fontSize: 6.5,
			fontFamily: "Inter",
			fontWeight: "bold",
			color: "#00FFFF",
			textAlign: "center",
			background: {
				enabled: true,
				color: "#0B0F19EE",
				paddingX: 16,
				paddingY: 8,
				cornerRadius: 6,
			},
			placement: {
				verticalAlign: "bottom",
				marginVerticalRatio: 0.1,
			},
		},
	},
	{
		id: "viral-punch",
		name: "Neon Green Punch",
		description: "Punchy short-form green text optimized for TikTok retention",
		preview: {
			textColor: "#00FF66",
			bgColor: "#000000",
			badgeText: "TIKTOK",
		},
		style: {
			fontSize: 7.2,
			fontFamily: "Inter",
			fontWeight: "bold",
			color: "#00FF66",
			textAlign: "center",
			background: {
				enabled: true,
				color: "#000000FA",
				paddingX: 20,
				paddingY: 10,
				cornerRadius: 10,
			},
			placement: {
				verticalAlign: "bottom",
				marginVerticalRatio: 0.14,
			},
		},
	},
	{
		id: "minimal-white",
		name: "Minimalist Studio White",
		description: "Clean modern cinematic subtitle pill with soft contrast",
		preview: {
			textColor: "#FFFFFF",
			bgColor: "#1E293B",
			badgeText: "CLEAN",
		},
		style: {
			fontSize: 5.5,
			fontFamily: "Inter",
			fontWeight: "normal",
			color: "#FFFFFF",
			textAlign: "center",
			background: {
				enabled: true,
				color: "#00000088",
				paddingX: 14,
				paddingY: 6,
				cornerRadius: 6,
			},
			placement: {
				verticalAlign: "bottom",
				marginVerticalRatio: 0.08,
			},
		},
	},
];

export const DEFAULT_CAPTION_PRESET_ID = "mrbeast-yellow";

export function getCaptionPresetById(id: string): CaptionPreset {
	return (
		CAPTION_PRESETS.find((p) => p.id === id) ??
		CAPTION_PRESETS[0]
	);
}

export function applyPresetToCaptions(
	captions: SubtitleCue[],
	presetId: string,
): SubtitleCue[] {
	const preset = getCaptionPresetById(presetId);
	return captions.map((cue) => ({
		...cue,
		style: {
			...cue.style,
			...preset.style,
		},
	}));
}
