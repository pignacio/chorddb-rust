import { findOrFail, positiveModule } from "./utils";

export interface FingeringUpdate {
  chord: string,
  fingering: string,
}


export class ChordSelector {
  root: Element;
  display: Element;
  spinner: Element;
  onChange: (e: FingeringUpdate) => void;
  currentChord: string = "none";
  selectedFingeringIndex: number = 0;
  cachedFingerings: {[key:string]: string[];} = {};

  constructor(element: Element, onChange: (e: FingeringUpdate) => void) {
    this.root = element;
    this.display = findOrFail(".selector-display", this.root)
    this.spinner = findOrFail(".selector-spinner", this.root)
    this.onChange = onChange;
    findOrFail('.next-fingering', this.root).addEventListener('click', _e => this.updateSelection(1));
    findOrFail('.previous-fingering', this.root).addEventListener('click', _e => this.updateSelection(-1));
  }

  async load(chord: string, currentFingering: string) {
    this.currentChord = chord;
    let fingerings = this.cachedFingerings[chord];
    if (fingerings == null) {
      this.display.innerHTML = '';
      this.display.classList.add("hidden")
      this.spinner.classList.remove("hidden");
      fingerings = await fetch("/chords/GUITAR_STANDARD/" + chord).then(r => r.json());
      this.cachedFingerings[chord] = fingerings;
      if (this.currentChord != chord) {
        // Selected chord has changed. Ignore update
        return;
      }
    }
    if (!fingerings.includes(currentFingering)) {
      fingerings.unshift(currentFingering);
    }
    this.selectedFingeringIndex = fingerings.indexOf(currentFingering);
    this.spinner.classList.add("hidden");
    this.display.innerHTML = currentFingering;
    this.display.classList.remove("hidden");
  }

  updateSelection(diff: number) {
    let chordFingerings = this.cachedFingerings[this.currentChord];
    this.selectedFingeringIndex = positiveModule(this.selectedFingeringIndex + diff, chordFingerings.length);
    let fingering = chordFingerings[this.selectedFingeringIndex];
    this.display.innerHTML = fingering;
    this.onChange({chord: this.currentChord, fingering: fingering })
  }
}
