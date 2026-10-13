import { describe, expect, it } from "bun:test";
import {
	VOICE_PRESETS,
	getVoicePresetById,
	generateVoiceoverWav,
} from "../tts-service";

describe("TTS Voiceover Service", () => {
	it("should provide creator voice presets", () => {
		const ids = VOICE_PRESETS.map((v) => v.id);
		expect(ids).toContain("storyteller");
		expect(ids).toContain("hype-creator");
		expect(ids).toContain("deep-cinematic");
		expect(ids).toContain("warm-explainer");
	});

	it("should find a voice preset by id or return fallback", () => {
		const preset = getVoicePresetById("deep-cinematic");
		expect(preset.name).toBe("Deep Cinematic");

		const fallback = getVoicePresetById("non-existent");
		expect(fallback.id).toBe("storyteller");
	});

	it("should generate a valid WAV blob with proper RIFF header and duration", async () => {
		const text = "Welcome to OpenCut Studio. Today we create viral short form content.";
		const result = await generateVoiceoverWav({
			text,
			voiceId: "hype-creator",
			speed: 1.1,
			pitch: 1.0,
		});

		expect(result).toBeDefined();
		expect(result.blob).toBeInstanceOf(Blob);
		expect(result.blob.type).toBe("audio/wav");
		expect(result.durationSeconds).toBeGreaterThan(0.5);

		// Verify RIFF and WAVE magic headers in the generated buffer
		const arrayBuffer = await result.blob.arrayBuffer();
		const bytes = new Uint8Array(arrayBuffer);
		const header = String.fromCharCode(...bytes.slice(0, 4));
		const format = String.fromCharCode(...bytes.slice(8, 12));
		expect(header).toBe("RIFF");
		expect(format).toBe("WAVE");
	});
});
