export interface SilenceInterval {
	start: number; // in seconds
	end: number;   // in seconds
	duration: number; // in seconds
}

export interface DetectSilenceOptions {
	samples: Float32Array;
	sampleRate: number;
	thresholdDb?: number; // e.g. -35 dB
	minDurationSeconds?: number; // e.g. 0.4s (400ms)
	paddingSeconds?: number; // e.g. 0.05s (50ms kept around speech edges)
	windowSizeSeconds?: number; // e.g. 0.02s (20ms analysis window)
}

/**
 * Scans audio PCM samples and detects silent intervals exceeding minDurationSeconds.
 */
export function detectSilenceIntervals(
	options: DetectSilenceOptions,
): SilenceInterval[] {
	const {
		samples,
		sampleRate,
		thresholdDb = -35,
		minDurationSeconds = 0.4,
		paddingSeconds = 0.05,
		windowSizeSeconds = 0.02,
	} = options;

	const windowSize = Math.max(1, Math.floor(windowSizeSeconds * sampleRate));
	const numWindows = Math.floor(samples.length / windowSize);
	const intervals: SilenceInterval[] = [];

	let inSilence = false;
	let silenceStartWindow = 0;

	for (let w = 0; w < numWindows; w++) {
		const offset = w * windowSize;
		let sumSquares = 0;

		for (let i = 0; i < windowSize; i++) {
			const s = samples[offset + i];
			sumSquares += s * s;
		}

		const rms = Math.sqrt(sumSquares / windowSize);
		const db = 20 * Math.log10(rms + 1e-9);
		const isSilent = db < thresholdDb;

		if (isSilent && !inSilence) {
			inSilence = true;
			silenceStartWindow = w;
		} else if (!isSilent && inSilence) {
			inSilence = false;
			const rawStart = silenceStartWindow * windowSizeSeconds;
			const rawEnd = w * windowSizeSeconds;
			const rawDuration = rawEnd - rawStart;

			if (rawDuration >= minDurationSeconds) {
				const start = Math.min(rawStart + paddingSeconds, rawEnd);
				const end = Math.max(rawEnd - paddingSeconds, start);
				const duration = end - start;

				if (duration > 0.01) {
					intervals.push({
						start: Number(start.toFixed(3)),
						end: Number(end.toFixed(3)),
						duration: Number(duration.toFixed(3)),
					});
				}
			}
		}
	}

	// Handle silence extending to the very end of the buffer
	if (inSilence) {
		const rawStart = silenceStartWindow * windowSizeSeconds;
		const rawEnd = numWindows * windowSizeSeconds;
		const rawDuration = rawEnd - rawStart;

		if (rawDuration >= minDurationSeconds) {
			const start = Math.min(rawStart + paddingSeconds, rawEnd);
			const end = Math.max(rawEnd - paddingSeconds, start);
			const duration = end - start;

			if (duration > 0.01) {
				intervals.push({
					start: Number(start.toFixed(3)),
					end: Number(end.toFixed(3)),
					duration: Number(duration.toFixed(3)),
				});
			}
		}
	}

	return intervals;
}

/**
 * Given a list of silence intervals and total duration, computes the active speech segments.
 */
export function calculateNonSilentIntervals({
	silenceIntervals,
	totalDuration,
}: {
	silenceIntervals: SilenceInterval[];
	totalDuration: number;
}): { start: number; end: number; duration: number }[] {
	if (silenceIntervals.length === 0) {
		return [{ start: 0, end: totalDuration, duration: totalDuration }];
	}

	const sorted = [...silenceIntervals].sort((a, b) => a.start - b.start);
	const keepers: { start: number; end: number; duration: number }[] = [];
	let currentPos = 0;

	for (const silence of sorted) {
		if (silence.start > currentPos) {
			const dur = silence.start - currentPos;
			if (dur > 0.01) {
				keepers.push({
					start: Number(currentPos.toFixed(3)),
					end: Number(silence.start.toFixed(3)),
					duration: Number(dur.toFixed(3)),
				});
			}
		}
		currentPos = Math.max(currentPos, silence.end);
	}

	if (currentPos < totalDuration) {
		const dur = totalDuration - currentPos;
		if (dur > 0.01) {
			keepers.push({
				start: Number(currentPos.toFixed(3)),
				end: Number(totalDuration.toFixed(3)),
				duration: Number(dur.toFixed(3)),
			});
		}
	}

	return keepers;
}
