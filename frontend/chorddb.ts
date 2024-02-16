import { type RenderBit, renderInSingleLine, createSpan } from "./render"

// Public interface for HTML
class ChordDB {
  chords: {chord: HTMLElement, fingering: HTMLElement}[] = [];
  selectedChord: number = 0;

  initTablature(lines: LineBit[][]) {
    this.buildLines("tablature", lines);
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
      let selectedChord = this.chords[this.selectedChord];
      selectedChord.chord.classList.remove("chord-selected");
      selectedChord.fingering.classList.remove("fingering-selected");
    }

    // Select current chord
    if (chord != null) {
      this.selectedChord = chord % this.chords.length;
      let selectedChord = this.chords[this.selectedChord];
      selectedChord.chord.classList.add("chord-selected");
      selectedChord.fingering.classList.add("fingering-selected");
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

  // getChords(chord: string): [HTMLElement, HTMLElement][] {
  //   let chords = this.chords[chord];
  //   if (chords == undefined) {
  //     chords = []
  //     this.chords[chord] = chords;
  //   }
  //   return chords;
  // }

  buildLine(line: LineBit[]): HTMLElement[] {
    let bits = [...line]
    bits.sort((a, b) => b.position - a.position)

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
        let chordSpan = createSpan("chord", chord.chord);
        currentLine.addBit({
          html: chordSpan,
          position: linebit.position,
          size: chord.chord.length,
        })
        let fingering = createSpan("fingering", "(" + chord.fingering + ")");
        let chordIndex = this.chords.length;
        console.log("Adding listenr for ", chord, chordIndex, fingering);
        fingering.dataset.index = chordIndex.toString();
        fingering.addEventListener('click', e => {
          console.log("Click!", e, e.target, chordIndex, this);
          this.updateSongDrawer(e.target.dataset.index, true);
        }, false);

        this.chords.push({chord: chordSpan, fingering: fingering});

        currentLine.addBit({
          html: fingering,
          position: linebit.position + chord.chord.length,
          size: chord.fingering.length + 2,
        })
        console.log("Added to line!", fingering, currentLine);
      } else {
        currentLine.addBit({
          html: createSpan("lyric", linebit.text),
          position: linebit.position,
          size: linebit.text.length,
        })
      }

    }

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

