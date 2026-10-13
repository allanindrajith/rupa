import { describe, expect, it } from "bun:test";
import { trimSilenceFromSceneTracks } from "../trim-silence-timeline";
import type { SceneTracks } from "@/lib/timeline";
import { TICKS_PER_SECOND } from "@/lib/wasm";

describe("Trim Silence from Scene Tracks", () => {
	it("should cut a middle silence from an element and ripple-shift subsequent elements", () => {
		// Mock scene with a 10s element on main track
		// And a 1.0s silence from 4.0s to 5.0s
		const trackId = "main-track-1";
		const tracks: SceneTracks = {
			overlay: [],
			main: {
				id: trackId,
				name: "Main",
				type: "video",
				muted: false,
				hidden: false,
				elements: [
					{
						id: "elem-1",
						name: "Clip 1",
						type: "video",
						startTime: 0,
						duration: 10 * TICKS_PER_SECOND,
						trimStart: 0,
						trimEnd: 0,
						transform: {} as any,
					} as any,
				],
			},
			audio: [],
		};

		const silences = [
			{ start: 4.0, end: 5.0, duration: 1.0 },
		];

		const updated = trimSilenceFromSceneTracks({ tracks, silences });

		// Should have split elem-1 into 2 pieces:
		// Piece 1: 0 to 4s (duration 4s)
		// Piece 2: 4s to 9s (duration 5s, shifted left by 1s)
		expect(updated.main.elements.length).toBe(2);
		expect(updated.main.elements[0].startTime).toBe(0);
		expect(updated.main.elements[0].duration).toBe(4 * TICKS_PER_SECOND);

		expect(updated.main.elements[1].startTime).toBe(4 * TICKS_PER_SECOND);
		expect(updated.main.elements[1].duration).toBe(5 * TICKS_PER_SECOND);
	});

	it("should remove elements completely inside silence and ripple shift", () => {
		const tracks: SceneTracks = {
			overlay: [],
			main: {
				id: "main-1",
				name: "Main",
				type: "video",
				muted: false,
				hidden: false,
				elements: [
					{
						id: "elem-1",
						name: "Clip 1",
						type: "video",
						startTime: 0,
						duration: 2 * TICKS_PER_SECOND,
						trimStart: 0,
						trimEnd: 0,
						transform: {} as any,
					} as any,
					{
						id: "silent-clip",
						name: "Dead Air",
						type: "video",
						startTime: 2 * TICKS_PER_SECOND,
						duration: 2 * TICKS_PER_SECOND, // 2s to 4s
						trimStart: 0,
						trimEnd: 0,
						transform: {} as any,
					} as any,
					{
						id: "elem-2",
						name: "Clip 2",
						type: "video",
						startTime: 4 * TICKS_PER_SECOND,
						duration: 3 * TICKS_PER_SECOND,
						trimStart: 0,
						trimEnd: 0,
						transform: {} as any,
					} as any,
				],
			},
			audio: [],
		};

		const silences = [
			{ start: 2.0, end: 4.0, duration: 2.0 },
		];

		const updated = trimSilenceFromSceneTracks({ tracks, silences });

		expect(updated.main.elements.length).toBe(2);
		expect(updated.main.elements[0].id).toBe("elem-1");
		expect(updated.main.elements[0].startTime).toBe(0);

		// elem-2 should have shifted from 4s to 2s
		expect(updated.main.elements[1].id).toBe("elem-2");
		expect(updated.main.elements[1].startTime).toBe(2 * TICKS_PER_SECOND);
	});
});
