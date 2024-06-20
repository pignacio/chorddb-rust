type MutedCorda = {
	type: 'mute';
};

type NotedCorda = {
	type: 'noted';
	note_fret: number;
	note: string;
};

export type Corda = MutedCorda | NotedCorda;

export type DrawedChord = {
	start_fret: number;
	end_fret: number;
	cordas: Corda[];
};

export function drawChord(rawFrets: string[]): DrawedChord {
	const frets = rawFrets.map(parseFret);
	const [start, end] = calculateStartEnd(frets);

	return {
		start_fret: start,
		end_fret: end,
		cordas: frets.map((fret) => buildCorda(fret))
	};
}

function buildCorda(fret: Fret): Corda {
	if (fret === 'X') {
		return {
			type: 'mute'
		};
	}
	return {
		type: 'noted',
		note_fret: fret,
		note: 'A0' // TODO: fix
	};
}

export function buildChordLines(rawFrets: string[]): string[] {
	const frets = rawFrets.map(parseFret);
	const [start, end] = calculateStartEnd(frets);

	// ---|
	const lines: string[] = [];
	if (start > 0) {
		lines.push(`       ${start}`);
	}
	lines.push(...frets.map((fret) => buildSingleLine(fret, start, end)));
	return lines;
}

type Fret = number | 'X';

function parseFret(fret: string): Fret {
	if (fret === 'X') {
		return 'X';
	}
	return parseInt(fret);
}

function calculateStartEnd(frets: Fret[]): [number, number] {
	const numbers = frets.flatMap((fret) => {
		if (fret === 'X' || fret === 0) {
			return [];
		} else {
			return [fret];
		}
	});
	if (numbers.length === 0) {
		return [0, 3];
	}
	const max = Math.max(...numbers);
	const min = Math.min(...numbers);
	if (max < 5) {
		return [0, 4];
	}
	if (min < 2) {
		return [0, max];
	}
	return [min, max];
}

function buildSingleLine(fret: Fret, start: number, end: number): string {
	let line = '';
	if (fret === 'X') {
		line += '|X';
	} else if (fret === 0) {
		line += '|o';
	} else {
		line += '||';
	}

	if (start > 0) {
		line += '...|';
	}

	for (let pos = Math.max(start, 1); pos <= end; pos++) {
		if (fret === pos) {
			line += '-o-|';
		} else {
			line += '---|';
		}
	}

	return line;
}
