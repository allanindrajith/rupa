import { describe, expect, it } from "bun:test";
import {
	ASPECT_RATIO_PRESETS,
	getAspectRatioPreset,
	getAspectRatioFromDimensions,
	type AspectRatioId,
} from "../aspect-ratios";

describe("Aspect Ratio Presets", () => {
	it("should provide social presets: 16:9, 9:16, 1:1, 4:5", () => {
		const ids = ASPECT_RATIO_PRESETS.map((p) => p.id);
		expect(ids).toContain("16:9");
		expect(ids).toContain("9:16");
		expect(ids).toContain("1:1");
		expect(ids).toContain("4:5");
	});

	it("should return correct dimensions for 9:16 vertical (Shorts/TikTok)", () => {
		const preset = getAspectRatioPreset("9:16");
		expect(preset).toBeDefined();
		expect(preset?.width).toBe(1080);
		expect(preset?.height).toBe(1920);
	});

	it("should return correct dimensions for 16:9 landscape", () => {
		const preset = getAspectRatioPreset("16:9");
		expect(preset).toBeDefined();
		expect(preset?.width).toBe(1920);
		expect(preset?.height).toBe(1080);
	});

	it("should match aspect ratio from dimensions", () => {
		expect(getAspectRatioFromDimensions(1920, 1080)).toBe("16:9");
		expect(getAspectRatioFromDimensions(1080, 1920)).toBe("9:16");
		expect(getAspectRatioFromDimensions(1080, 1080)).toBe("1:1");
		expect(getAspectRatioFromDimensions(1080, 1350)).toBe("4:5");
	});
});
