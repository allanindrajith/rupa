"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from "@/components/ui/popover";
import { Slider } from "@/components/ui/slider";
import { Spinner } from "@/components/ui/spinner";
import {
	ScissorIcon,
	VolumeHighIcon,
	CheckmarkCircle02Icon,
	SparklesIcon,
} from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";
import { useEditor } from "@/hooks/use-editor";
import { extractTimelineAudio } from "@/lib/media/mediabunny";
import { decodeAudioToFloat32 } from "@/lib/media/audio";
import {
	detectSilenceIntervals,
	type SilenceInterval,
} from "@/services/audio/silence-detector";
import { trimSilenceFromSceneTracks } from "@/services/audio/trim-silence-timeline";

export function SilenceRemoverPopover() {
	const [isOpen, setIsOpen] = useState(false);
	const [thresholdDb, setThresholdDb] = useState(-35);
	const [minDurationMs, setMinDurationMs] = useState(400);
	const [paddingMs, setPaddingMs] = useState(50);
	const [isAnalyzing, setIsAnalyzing] = useState(false);
	const [isTrimming, setIsTrimming] = useState(false);
	const [detectedSilences, setDetectedSilences] = useState<SilenceInterval[] | null>(null);
	const [statusMessage, setStatusMessage] = useState<string | null>(null);

	const editor = useEditor();

	const handleScanSilence = async () => {
		const activeScene = editor.scenes.getActiveScene();
		const totalDuration = editor.timeline.getTotalDuration();
		if (totalDuration <= 0) {
			setStatusMessage("Timeline is empty. Add video or audio first.");
			return;
		}

		setIsAnalyzing(true);
		setStatusMessage("Extracting and analyzing audio...");
		setDetectedSilences(null);

		try {
			const audioBlob = await extractTimelineAudio({
				tracks: activeScene.tracks,
				mediaAssets: editor.media.getAssets(),
				totalDuration,
			});

			const { samples, sampleRate } = await decodeAudioToFloat32({
				audioBlob,
				sampleRate: 44100,
			});

			const silences = detectSilenceIntervals({
				samples,
				sampleRate,
				thresholdDb,
				minDurationSeconds: minDurationMs / 1000,
				paddingSeconds: paddingMs / 1000,
			});

			setDetectedSilences(silences);
			if (silences.length === 0) {
				setStatusMessage("No silence detected with current threshold.");
			} else {
				const totalSaved = silences.reduce((acc, s) => acc + s.duration, 0);
				setStatusMessage(
					`Found ${silences.length} pause${silences.length > 1 ? "s" : ""} (${totalSaved.toFixed(1)}s dead air to trim)!`,
				);
			}
		} catch (err) {
			console.error("Silence analysis error:", err);
			setStatusMessage(
				err instanceof Error ? err.message : "Error analyzing audio",
			);
		} finally {
			setIsAnalyzing(false);
		}
	};

	const handleApplyTrim = () => {
		if (!detectedSilences || detectedSilences.length === 0) return;

		setIsTrimming(true);
		try {
			const activeScene = editor.scenes.getActiveScene();
			const updatedTracks = trimSilenceFromSceneTracks({
				tracks: activeScene.tracks,
				silences: detectedSilences,
			});

			editor.timeline.updateTracks(updatedTracks);
			const totalSaved = detectedSilences.reduce((acc, s) => acc + s.duration, 0);
			setStatusMessage(`Trimmed ${totalSaved.toFixed(1)}s of silence! Pacing tightened.`);
			setDetectedSilences(null);
		} catch (err) {
			console.error("Silence trim error:", err);
			setStatusMessage("Failed to apply silence trim");
		} finally {
			setIsTrimming(false);
		}
	};

	return (
		<Popover open={isOpen} onOpenChange={setIsOpen}>
			<PopoverTrigger asChild>
				<Button
					variant="ghost"
					size="icon"
					title="Smart Silence Remover / Dead Air Trimmer"
					className="text-muted-foreground hover:text-foreground h-8 w-8"
				>
					<HugeiconsIcon icon={VolumeHighIcon} size={16} />
				</Button>
			</PopoverTrigger>
			<PopoverContent align="start" className="w-80 p-4 flex flex-col gap-3 shadow-xl">
				<div className="flex items-center justify-between border-b pb-2">
					<div className="flex items-center gap-1.5">
						<HugeiconsIcon icon={SparklesIcon} size={16} className="text-primary" />
						<h4 className="font-semibold text-sm">Silence Remover</h4>
					</div>
					<span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-primary/10 text-primary font-bold">
						PRO PACING
					</span>
				</div>

				<p className="text-xs text-muted-foreground leading-relaxed">
					Automatically detect dead air and pauses, then ripple-cut to maximize viewer retention.
				</p>

				{/* Controls */}
				<div className="flex flex-col gap-3 pt-1">
					<div className="flex flex-col gap-1">
						<div className="flex justify-between text-xs">
							<span className="text-muted-foreground">Sensitivity</span>
							<span className="font-mono font-medium">{thresholdDb} dB</span>
						</div>
						<Slider
							value={[thresholdDb]}
							min={-50}
							max={-20}
							step={1}
							onValueChange={([val]) => setThresholdDb(val)}
						/>
					</div>

					<div className="flex flex-col gap-1">
						<div className="flex justify-between text-xs">
							<span className="text-muted-foreground">Min Pause Length</span>
							<span className="font-mono font-medium">{minDurationMs} ms</span>
						</div>
						<Slider
							value={[minDurationMs]}
							min={200}
							max={1200}
							step={50}
							onValueChange={([val]) => setMinDurationMs(val)}
						/>
					</div>

					<div className="flex flex-col gap-1">
						<div className="flex justify-between text-xs">
							<span className="text-muted-foreground">Speech Edge Padding</span>
							<span className="font-mono font-medium">{paddingMs} ms</span>
						</div>
						<Slider
							value={[paddingMs]}
							min={20}
							max={150}
							step={10}
							onValueChange={([val]) => setPaddingMs(val)}
						/>
					</div>
				</div>

				{statusMessage && (
					<div className="p-2 rounded bg-secondary/50 border border-border/50 text-xs text-center font-medium">
						{statusMessage}
					</div>
				)}

				{/* Detected list summary */}
				{detectedSilences && detectedSilences.length > 0 && (
					<div className="max-h-24 overflow-y-auto rounded border border-border/40 p-1.5 flex flex-col gap-1 bg-background/50">
						{detectedSilences.slice(0, 5).map((s, idx) => (
							<div key={idx} className="flex justify-between text-[11px] font-mono px-1 py-0.5 rounded bg-secondary/30">
								<span>{s.start.toFixed(2)}s → {s.end.toFixed(2)}s</span>
								<span className="text-destructive font-bold">-{s.duration.toFixed(2)}s</span>
							</div>
						))}
						{detectedSilences.length > 5 && (
							<span className="text-[10px] text-muted-foreground text-center">
								+ {detectedSilences.length - 5} more pauses
							</span>
						)}
					</div>
				)}

				{/* Action Buttons */}
				<div className="flex flex-col gap-2 pt-1">
					<Button
						type="button"
						variant="outline"
						size="sm"
						onClick={handleScanSilence}
						disabled={isAnalyzing || isTrimming}
						className="w-full gap-1.5"
					>
						{isAnalyzing && <Spinner className="size-3" />}
						{isAnalyzing ? "Scanning Audio..." : "Scan Dead Air"}
					</Button>

					{detectedSilences && detectedSilences.length > 0 && (
						<Button
							type="button"
							size="sm"
							onClick={handleApplyTrim}
							disabled={isTrimming}
							className="w-full gap-1.5 bg-primary text-primary-foreground font-semibold"
						>
							{isTrimming ? (
								<Spinner className="size-3" />
							) : (
								<HugeiconsIcon icon={ScissorIcon} size={14} />
							)}
							{isTrimming ? "Trimming..." : "Ripple Trim All Silence"}
						</Button>
					)}
				</div>
			</PopoverContent>
		</Popover>
	);
}
