import { describe, expect, it } from "bun:test";
import { GUIDE_REGISTRY, getGuideById } from "../index";

describe("Social Guide Overlays", () => {
	it("should register tiktok, ig-reels, and yt-shorts", () => {
		const guideIds = GUIDE_REGISTRY.map((g) => g.id);
		expect(guideIds).toContain("tiktok");
		expect(guideIds).toContain("ig-reels");
		expect(guideIds).toContain("yt-shorts");
	});

	it("should provide non-null overlay renderers for tiktok, reels, and shorts", () => {
		const tiktok = getGuideById("tiktok");
		const reels = getGuideById("ig-reels");
		const shorts = getGuideById("yt-shorts");

		expect(tiktok?.renderOverlay).toBeDefined();
		expect(reels?.renderOverlay).toBeDefined();
		expect(shorts?.renderOverlay).toBeDefined();

		// Should render React elements without throwing
		const reelsOverlay = reels?.renderOverlay({ width: 1080, height: 1920 });
		const shortsOverlay = shorts?.renderOverlay({ width: 1080, height: 1920 });

		expect(reelsOverlay).not.toBeNull();
		expect(shortsOverlay).not.toBeNull();
	});
});
