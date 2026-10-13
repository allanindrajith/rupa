import { describe, expect, it } from "bun:test";
import {
	detectSilenceIntervals,
	calculateNonSilentIntervals,
} from "../silence-detector";

describe("Smart Silence Detector", () => {
	it("should detect silent sections in audio samples", () => {
		const sampleRate = 44100;
		// 3 seconds total:
		// 0.0s - 1.0s: Sound (amplitude 0.5)
		// 1.0s - 2.0s: Silence (amplitude 0.0001 = -80dB)
		// 2.0s - 3.0s: Sound (amplitude 0.5)
		const totalSamples = sampleRate * 3;
		const samples = new Float32Array(totalSamples);

		// Sound 1: 0 to 1s
		for (let i = 0; i < sampleRate; i++) {
			samples[i] = 0.5 * Math.sin((2 * Math.PI * 440 * i) / sampleRate);
		}
		// Silence: 1s to 2s is 0
		// Sound 2: 2s to 3s
		for (let i = sampleRate * 2; i < totalSamples; i++) {
			samples[i] = 0.5 * Math.sin((2 * Math.PI * 440 * i) / sampleRate);
		}

		const silences = detectSilenceIntervals({
			samples,
			sampleRate,
			thresholdDb: -35,
			minDurationSeconds: 0.3,
			paddingSeconds: 0.0,
		});

		expect(silences.length).toBe(1);
		expect(silences[0].start).toBeCloseTo(1.0, 1);
		expect(silences[0].end).toBeCloseTo(2.0, 1);
		expect(silences[0].duration).toBeCloseTo(1.0, 1);
	});

	it("should ignore short pauses shorter than minDurationSeconds", () => {
		const sampleRate = 44100;
		// 1 second with a tiny 0.1s pause in between
		const samples = new Float32Array(sampleRate);
		for (let i = 0; i < sampleRate; i++) {
			if (i >= 0.4 * sampleRate && i < 0.5 * sampleRate) {
				samples[i] = 0; // 100ms pause
			} else {
				samples[i] = 0.3 * Math.sin((2 * Math.PI * 440 * i) / sampleRate);
			}
		}

		const silences = detectSilenceIntervals({
			samples,
			sampleRate,
			thresholdDb: -35,
			minDurationSeconds: 0.3, // Requires at least 300ms
		});

		expect(silences.length).toBe(0);
	});

	it("should calculate active non-silent speech intervals", () => {
		const silences = [
			{ start: 1.0, end: 2.0, duration: 1.0 },
			{ start: 4.0, end: 4.5, duration: 0.5 },
		];
		const speechIntervals = calculateNonSilentIntervals({
			silenceIntervals: silences,
			totalDuration: 6.0,
		});

		expect(speechIntervals.length).toBe(3);
		// 0.0 to 1.0
		expect(speechIntervals[0].start).toBe(0.0);
		expect(speechIntervals[0].end).toBe(1.0);
		// 2.0 to 4.0
		expect(speechIntervals[1].start).toBe(2.0);
		expect(speechIntervals[1].end).toBe(4.0);
		// 4.5 to 6.0
		expect(speechIntervals[2].start).toBe(4.5);
		expect(speechIntervals[2].end).toBe(6.0);
	});
});
