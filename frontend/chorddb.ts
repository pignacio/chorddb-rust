import { type RenderBit, renderInSingleLine, createSpan } from "./render"

// Public interface for HTML
class ChordDB {
  chordCount: number = 0;
  selectedChordIndex: number = 0;
  selectedFingeringIndex: number = 0;
  cachedFingerings: {[key:string]: string[];} = {};

  initTablature(lines: LineBit[][]) {
    this.buildLines("tablature", lines);
    document.querySelectorAll("[data-fingering-index]").forEach(f => {
      f.addEventListener('click', e => {
        if (!(e.target instanceof HTMLElement)) {
          return;
        }
        let index = e.target.dataset.fingeringIndex;
        this.updateSongDrawer(index != null ? parseInt(index) : null, true);
      }, false);
    });
  }

  buildLines(contentId: string, lines: LineBit[][]) {
    let content = document.getElementById(contentId);
    if (content == undefined) {
      console.error("Could not find element with id " + contentId);
      return;
    }
    let pre = document.createElement("div")
    pre.classList.add("tablature")
    pre.classList.add("font-mono")

    for (let line of lines) {
      this.buildLine(line).forEach(html => {
        pre.appendChild(html)
        pre.append("\n")
      })
    }

    content?.appendChild(pre)
  }

  updateSongDrawer(chordIndex: number | null, isOpen: boolean) {
    // Deselect current chord if needed
    if (!isOpen || (chordIndex != null && chordIndex != this.selectedChordIndex)) {
      findChord(this.selectedChordIndex)?.classList?.remove("chord-selected");
      findFingering(this.selectedChordIndex)?.classList?.remove("fingering-selected");
    }

    if (isOpen) {
      chordIndex = positiveModule(chordIndex ?? 0, this.chordCount);
      this.selectedChordIndex = chordIndex;
      findChord(this.selectedChordIndex)?.classList?.add("chord-selected");
      let fingering = findFingering(this.selectedChordIndex);
      if (fingering != null) {
        fingering.classList.add("fingering-selected");
        let currentFingering = fingering.dataset.fingering ?? "XXXXXX";
        let chord = fingering.dataset.chord ?? "UNKNOWN";
        let currentChord = document.getElementById("current-chord");
        if (!currentChord) {
          console.error("Could not find #current-chord")
          return;
        }
        currentChord.innerHTML = chord;
        this.getFingerings(chord)
          .then(ff => {
            if (this.selectedChordIndex != chordIndex) {
              // Selected chord has changed. Ignore update
              return;
            }
            if (!ff.includes(currentFingering)) {
              ff.unshift(currentFingering);
            }
            this.selectedFingeringIndex = ff.indexOf(currentFingering);
            document.getElementById("chord-options").innerHTML = JSON.stringify(ff)
            this.updateCurrentFingering(0);
          });
      }
    }

    // Update checkbox
    var checkbox = document.getElementById("song-drawer-checkbox")
    if (isInput(checkbox)) {
      checkbox.checked = isOpen;
    }

    // Open/close drawer
    var drawer = document.getElementById("drawer");
    if (!drawer) return;
    if (isOpen) {
      drawer.classList.remove("song-drawer-closed")
    } else {
      drawer.classList.add("song-drawer-closed")
    }
  }

  updateCurrentFingering(diff: number) {
    let chord = findFingering(this.selectedChordIndex)?.dataset?.chord;
    if (!chord) {
      console.error("Could not find chord for index", this.selectedChordIndex)
      return;
    }
    this.selectedFingeringIndex = positiveModule(this.selectedFingeringIndex + diff, this.cachedFingerings[chord].length);

    let fingering = this.cachedFingerings[chord][this.selectedFingeringIndex];
    document.getElementById("current-fingering").innerHTML = fingering;
    document.querySelectorAll(".fingering[data-chord='" + chord + "']").forEach(elem => elem.innerHTML = "(" + fingering + ")");
  }

  buildLine(line: LineBit[]): HTMLElement[] {
    let bits = [...line];
    bits.sort((a, b) => a.position - b.position);

    // Assign indices in forward order
    for (let bit of bits) {
      if (bit.chord != undefined) {
        bit.chordIndex = this.chordCount++;
      }
    }

    // Perform pre-rendering and overlap calculations in reverse order
    bits.reverse();

    let lines: RenderLine[] = [new RenderLine()]
    for (let linebit of bits) {
      let lineIndex : number = 0;
      let lastBitPosition = linebit.position + getBitSize(linebit)
      while (lastBitPosition > lines[lineIndex].lastPosition) {
        lineIndex++;
        while (lines.length <= lineIndex) {
          lines.push(new RenderLine())
        }
      }

      // Render arrow
      for (let i = 0; i < lineIndex; i++) {
        lines[i].addBit({
          html: createSpan(linebit.type == BitType.Chord ? "chord" : "lyric", i == 0 ? "v" : "|"),
          position: linebit.position,
          size: 1,
        })
      }

      // Render actual bit
      let currentLine = lines[lineIndex]
      let chord = linebit.chord;
      if (chord != undefined) {
        if (linebit.chordIndex == undefined) {
          console.error("Missing chordIndex", linebit);
          continue;
        }
        let chordSpan = createSpan("chord", chord.chord);
        chordSpan.dataset.chordIndex = linebit.chordIndex.toString();

        let fingering = createSpan("fingering", "(" + chord.fingering + ")");
        fingering.dataset.fingeringIndex = linebit.chordIndex.toString();
        fingering.dataset.chord = chord.chord;
        fingering.dataset.fingering = chord.fingering;

        currentLine.addBit({
          html: chordSpan,
          position: linebit.position,
          size: chord.chord.length,
        })
        currentLine.addBit({
          html: fingering,
          position: linebit.position + chord.chord.length,
          size: chord.fingering.length + 2,
        })
      } else {
        currentLine.addBit({
          html: createSpan("lyric", linebit.text),
          position: linebit.position,
          size: linebit.text.length,
        })
      }

    }

    // Finally render each line in forward order
    lines.reverse()
    let res = lines.map(line => renderInSingleLine(line.bits))
    return res
  }

  async getFingerings(chord: string) {
    let cached = this.cachedFingerings[chord];
    if (cached != null) {
      return cached;
    }
    let fingerings: string[] = await fetch("/chords/GUITAR_STANDARD/" + chord).then(r => r.json());
    this.cachedFingerings[chord]  = fingerings;
    return fingerings;
  }
}

declare global {
  interface Window { chorddb: ChordDB; }
}

window.chorddb = new ChordDB;

// Library follows

enum BitType {
  Text = "text",
  Chord = "chord",
}

interface Chord {
  chord: string,
  fingering: string,
}

interface LineBit {
  type: BitType,
  position: number,
  text: string,
  chord: Chord | undefined,
  chordIndex: number | undefined,
}

function getBitSize(bit: LineBit) {
  if (bit.type == BitType.Chord) {
    let chord = bit.chord;
    if (chord == undefined) {
      console.error("Found a chord bit without chord!", bit);
      return bit.text.length;
    }
    return chord.chord.length + chord.fingering.length + 2;
  }
  return bit.text.length;
}

class RenderLine {
  bits: RenderBit[] = [];
  lastPosition: number = Number.MAX_SAFE_INTEGER;

  addBit(bit: RenderBit) {
    this.bits.push(bit)
    this.lastPosition = this.lastPosition == undefined ? bit.position : Math.min(this.lastPosition, bit.position)
  }
}


function isInput(element: HTMLElement | null): element is HTMLInputElement {
  return element != null && element != undefined && element instanceof HTMLInputElement;
}

function findChord(index: number): HTMLElement | null {
  let element = findHtmlElement('[data-chord-index="' + index + '"]')
  if (!element) {
    console.error("Could not find chord for index", index);
  }
  return element;
}

function findFingering(index: number): HTMLElement | null {
  let element = findHtmlElement('[data-fingering-index="' + index + '"]');
  if (!element) {
    console.error("Could not find chord for index", index);
  }
  return element;
}

function findHtmlElement(query: string): HTMLElement | null {
  let element = document.querySelector(query);
  if (element != null && element instanceof HTMLElement) {
    return element;
  }
  return null;
}

function positiveModule(value: number, modulus: number) {
  let result = value % modulus;
  return result < 0 ? result + modulus : result;
}
