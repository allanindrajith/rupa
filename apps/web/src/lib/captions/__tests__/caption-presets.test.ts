import { describe, expect, it } from "bun:test";
import {
	CAPTION_PRESETS,
	applyPresetToCaptions,
	getCaptionPresetById,
} from "../presets";
import type { SubtitleCue } from "@/lib/subtitles/types";

describe("Caption Creator Presets", () => {
	it("should have essential creator presets defined", () => {
		const ids = CAPTION_PRESETS.map((p) => p.id);
		expect(ids).toContain("mrbeast-yellow");
		expect(ids).toContain("neon-glow");
		expect(ids).toContain("minimal-white");
		expect(ids).toContain("viral-punch");
	});

	it("should retrieve a preset by id or return the default", () => {
		const mrbeast = getCaptionPresetById("mrbeast-yellow");
		expect(mrbeast.id).toBe("mrbeast-yellow");
		expect(mrbeast.style.color).toBe("#FFE600");

		const unknown = getCaptionPresetById("unknown-preset-id");
		expect(unknown.id).toBe("mrbeast-yellow"); // fallback to default
	});

	it("should apply preset style to all cues without mutating originals", () => {
		const originalCues: SubtitleCue[] = [
			{ text: "HELLO", startTime: 0, duration: 1 },
			{ text: "WORLD", startTime: 1, duration: 1.5 },
		];

		const styledCues = applyPresetToCaptions(originalCues, "neon-glow");

		expect(styledCues.length).toBe(2);
		expect(styledCues[0].style?.color).toBe("#00FFFF");
		expect(styledCues[1].style?.color).toBe("#00FFFF");
		// Verify immutability
		expect(originalCues[0].style).toBeUndefined();
	});
});
