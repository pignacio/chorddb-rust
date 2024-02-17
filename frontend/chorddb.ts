import { type RenderBit, renderInSingleLine, createSpan } from "./render"

// Public interface for HTML
class ChordDB {
  chordCount: number = 0;
  selectedChord: number = 0;

  initTablature(lines: LineBit[][]) {
    this.buildLines("tablature", lines);
    document.querySelectorAll("[data-fingering-index]").forEach(f => {
      console.log("Adding listener for ", f);
      f.addEventListener('click', e => {
        if (!(e.target instanceof HTMLElement)) {
          return;
        }
        let index = e.target.dataset.fingeringIndex;
        console.log("Click!", e, e.target, index, this);
        this.updateSongDrawer(index != null ? parseInt(index) : null, true);
      }, false);
    });
  }

  buildLines(contentId: string, lines: LineBit[][]) {
    let content = document.getElementById(contentId);
    if (content == undefined) {
      console.error("Could not find element with id " + contentId);
    }
    let pre = document.createElement("div")
    pre.classList.add("tablature")
    pre.classList.add("font-mono")

    for (let line of lines) {
      this.buildLine(line).forEach(html => {
        pre.appendChild(html)
        pre.appendChild(document.createElement("br"))
      })
    }

    content?.appendChild(pre)
  }

  // toggleSongDrawer() {
  //   var checkbox = document.getElementById("song-drawer-checkbox");
  //   if (isInput(checkbox)) {
  //     checkbox.checked = !checkbox.checked
  //     this.updateSongDrawer(checkbox.checked ? this.firstChord : null);
  //   }
  // }

  updateSongDrawer(chord: number | null, isOpen: boolean) {
    console.log("updateSongDrawer:", chord, isOpen);

    // Deselect current chord if needed
    if (!isOpen || (chord != null && chord != this.selectedChord)) {
      findChord(this.selectedChord)?.classList?.remove("chord-selected");
      findFingering(this.selectedChord)?.classList?.remove("fingering-selected");
    }

    // Select current chord
    if (chord != null) {
      this.selectedChord = chord % this.chordCount;
    }
    if (isOpen) {
      findChord(this.selectedChord)?.classList?.add("chord-selected");
      let fingering = findFingering(this.selectedChord);
      if (fingering != null) {
        fingering.classList.add("fingering-selected");
        document.getElementById("current-chord").innerHTML = fingering.dataset.fingeringChord ?? "UNKNOWN";
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
        fingering.dataset.fingeringChord = chord.chord;

        // console.log("Adding listener for ", chord, linebit.chordIndex, fingering);
        // fingering.addEventListener('click', e => {
        //   if (!(e.target instanceof HTMLElement)) {
        //     return;
        //   }
        //   let index = e.target.dataset.fingeringIndex;
        //   console.log("Click!", e, e.target, index, this);
        //   this.updateSongDrawer(index != null ? parseInt(index) : null, true);
        // }, false);


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
  let element =  document.querySelector('[data-chord-index="' + index + '"]')
  if (element != null && element instanceof HTMLElement) {
    return element;
  }
  console.log("Could not find chord for index", index);
  return null;
}

function findFingering(index: number): HTMLElement | null {
  let element = document.querySelector('[data-fingering-index="' + index + '"]');
  if (element != null && element instanceof HTMLElement) {
    return element;
  }
  console.log("Could not find chord for index", index);
  return null;
}
