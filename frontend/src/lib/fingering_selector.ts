import type { Fingering } from './api/fingerings';

export interface FingeringChange {
	previous: Fingering;
	current: Fingering;
}
