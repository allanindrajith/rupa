import type { SceneTracks, TimelineTrack, TimelineElement } from "@/lib/timeline";
import type { SilenceInterval } from "./silence-detector";
import { TICKS_PER_SECOND } from "@/lib/wasm";
import { generateUUID } from "@/utils/id";

function processTrackSilence<
	TElement extends TimelineTrack["elements"][number],
	TTrack extends TimelineTrack & { elements: TElement[] },
>(track: TTrack, silences: SilenceInterval[]): TTrack {
	// Sort silences in descending order so earlier timestamps remain unaltered
	const sorted = [...silences].sort((a, b) => b.start - a.start);

	let currentElements: TimelineElement[] = [...track.elements];

	for (const silence of sorted) {
		const startTicks = Math.round(silence.start * TICKS_PER_SECOND);
		const endTicks = Math.round(silence.end * TICKS_PER_SECOND);
		const cutTicks = endTicks - startTicks;
		if (cutTicks <= 0) continue;

		const nextElements: TimelineElement[] = [];

		for (const el of currentElements) {
			const elStart = el.startTime;
			const elEnd = el.startTime + el.duration;

			// Case 1: element is entirely before silence
			if (elEnd <= startTicks) {
				nextElements.push(el);
				continue;
			}

			// Case 2: element is entirely after silence => shift left by cutTicks
			if (elStart >= endTicks) {
				nextElements.push({
					...el,
					startTime: elStart - cutTicks,
				});
				continue;
			}

			// Case 3: element is completely inside the silence => discard
			if (elStart >= startTicks && elEnd <= endTicks) {
				continue;
			}

			// Case 4: element spans across the entire silence => split into left and right pieces
			if (elStart < startTicks && elEnd > endTicks) {
				const leftDuration = startTicks - elStart;
				const rightDuration = elEnd - endTicks;
				const rightTrimStart = (el.trimStart ?? 0) + (endTicks - elStart);

				// Left piece
				nextElements.push({
					...el,
					duration: leftDuration,
				});

				// Right piece (shifted left to startTicks)
				nextElements.push({
					...el,
					id: generateUUID(),
					startTime: startTicks,
					duration: rightDuration,
					trimStart: rightTrimStart,
				});
				continue;
			}

			// Case 5: element starts before silence and ends inside silence => trim end
			if (elStart < startTicks && elEnd > startTicks && elEnd <= endTicks) {
				nextElements.push({
					...el,
					duration: startTicks - elStart,
				});
				continue;
			}

			// Case 6: element starts inside silence and ends after silence => trim start and shift left
			if (elStart >= startTicks && elStart < endTicks && elEnd > endTicks) {
				const rightDuration = elEnd - endTicks;
				const rightTrimStart = (el.trimStart ?? 0) + (endTicks - elStart);
				nextElements.push({
					...el,
					startTime: startTicks,
					duration: rightDuration,
					trimStart: rightTrimStart,
				});
				continue;
			}
		}

		currentElements = nextElements;
	}

	return {
		...track,
		elements: currentElements as TElement[],
	};
}

export function trimSilenceFromSceneTracks({
	tracks,
	silences,
}: {
	tracks: SceneTracks;
	silences: SilenceInterval[];
}): SceneTracks {
	if (silences.length === 0) return tracks;

	return {
		overlay: tracks.overlay.map((track) => processTrackSilence(track, silences)),
		main: processTrackSilence(tracks.main, silences),
		audio: tracks.audio.map((track) => processTrackSilence(track, silences)),
	};
}
