"use client";

import { useEditor } from "@/hooks/use-editor";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
} from "@/components/ui/select";
import {
	ASPECT_RATIO_PRESETS,
	getAspectRatioFromDimensions,
	getAspectRatioPreset,
	type AspectRatioId,
} from "@/lib/canvas/aspect-ratios";

export function AspectRatioSelector() {
	const editor = useEditor();
	const canvasSize = useEditor(
		(e) => e.project.getActive()?.settings.canvasSize,
	);

	const currentRatioId = getAspectRatioFromDimensions(
		canvasSize?.width,
		canvasSize?.height,
	);

	const currentPreset = getAspectRatioPreset(currentRatioId);

	const handleRatioChange = (val: string) => {
		const preset = getAspectRatioPreset(val as AspectRatioId);
		if (!preset) return;

		editor.project.updateSettings({
			canvasSize: {
				width: preset.width,
				height: preset.height,
			},
		});
	};

	return (
		<Select value={currentRatioId} onValueChange={handleRatioChange}>
			<SelectTrigger className="h-7 text-xs px-2.5 min-w-[70px] font-medium border-border/40 bg-background/50 hover:bg-accent/40">
				<span>{currentPreset?.label ?? "16:9"}</span>
			</SelectTrigger>
			<SelectContent align="end" className="text-xs min-w-[170px]">
				{ASPECT_RATIO_PRESETS.map((preset) => (
					<SelectItem
						key={preset.id}
						value={preset.id}
						className="flex items-center justify-between text-xs py-1.5"
					>
						<span className="font-semibold">{preset.label}</span>
						<span className="text-muted-foreground ml-2 text-[10px]">
							{preset.description}
						</span>
					</SelectItem>
				))}
			</SelectContent>
		</Select>
	);
}
