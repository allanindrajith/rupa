export function InstagramReelsLayout() {
	return (
		<svg
			width="100%"
			height="100%"
			viewBox="0 0 1080 1920"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
			className="pointer-events-none select-none opacity-85"
		>
			<title>Instagram Reels Safe Zone Layout</title>
			{/* Safe Area Box Guide */}
			<rect
				x="70"
				y="160"
				width="880"
				height="1500"
				stroke="#00E5FF"
				strokeWidth="2"
				strokeDasharray="8 8"
				fill="none"
				opacity="0.5"
			/>
			<text x="80" y="195" fill="#00E5FF" fontSize="24" fontFamily="sans-serif" opacity="0.8">
				Reels Safe Zone (Keep essential visuals & text inside)
			</text>

			{/* Top Header Mockup */}
			<g opacity="0.75" fill="white">
				<text x="70" y="110" fontSize="38" fontWeight="bold" fontFamily="sans-serif">
					Reels
				</text>
				{/* Audio / Camera icons on top right */}
				<circle cx="980" cy="100" r="22" stroke="white" strokeWidth="3" fill="none" />
			</g>

			{/* Right Sidebar Action Icons */}
			<g transform="translate(980, 1300)" opacity="0.85">
				{/* Heart / Like */}
				<path
					d="M0 -15 C-12 -35 -40 -15 -25 10 L0 35 L25 10 C40 -15 12 -35 0 -15 Z"
					fill="white"
				/>
				<text x="0" y="55" fill="white" fontSize="22" textAnchor="middle" fontFamily="sans-serif">
					128K
				</text>

				{/* Comment bubble */}
				<g transform="translate(0, 110)">
					<path
						d="M-22 -18 H22 C26 -18 28 -14 28 -10 V14 C28 18 26 22 22 22 H-8 L-22 32 V22 H-22 C-26 22 -28 18 -28 14 V-10 C-28 -14 -26 -18 -22 -18 Z"
						fill="white"
					/>
					<text x="0" y="55" fill="white" fontSize="22" textAnchor="middle" fontFamily="sans-serif">
						4.2K
					</text>
				</g>

				{/* Share / Paper Plane */}
				<g transform="translate(0, 220)">
					<path d="M-22 -20 L24 0 L-22 20 L-14 0 Z" fill="white" />
					<text x="0" y="45" fill="white" fontSize="22" textAnchor="middle" fontFamily="sans-serif">
						Share
					</text>
				</g>

				{/* 3 dots */}
				<g transform="translate(0, 310)" fill="white">
					<circle cx="-16" cy="0" r="4" />
					<circle cx="0" cy="0" r="4" />
					<circle cx="16" cy="0" r="4" />
				</g>

				{/* Audio album square */}
				<g transform="translate(0, 390)">
					<rect x="-20" y="-20" width="40" height="40" rx="8" stroke="white" strokeWidth="4" fill="#333" />
					<circle cx="0" cy="0" r="7" fill="white" />
				</g>
			</g>

			{/* Bottom Left Account & Caption Area */}
			<g transform="translate(70, 1720)" opacity="0.85">
				{/* Avatar */}
				<circle cx="28" cy="28" r="28" fill="#555" stroke="white" strokeWidth="2" />
				{/* Handle */}
				<text x="75" y="36" fill="white" fontSize="30" fontWeight="bold" fontFamily="sans-serif">
					@creator
				</text>
				{/* Follow Button */}
				<rect x="230" y="10" width="105" height="36" rx="6" stroke="white" strokeWidth="2" fill="none" />
				<text x="282" y="35" fill="white" fontSize="20" fontWeight="bold" textAnchor="middle" fontFamily="sans-serif">
					Follow
				</text>

				{/* Caption line */}
				<text x="0" y="85" fill="white" fontSize="26" fontFamily="sans-serif">
					Your viral video caption appears right here... #trending #reels
				</text>

				{/* Audio track info */}
				<g transform="translate(0, 125)">
					<path d="M0 5 L16 -4 V14 A6 6 0 1 1 6 8 V1 Z" fill="white" />
					<text x="28" y="12" fill="white" fontSize="22" opacity="0.9" fontFamily="sans-serif">
						Original audio · creator_sound
					</text>
				</g>
			</g>
		</svg>
	);
}
