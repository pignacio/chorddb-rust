// render.ts
function renderInSingleLine(bits) {
  let html = createSpan("line");
  bits.sort((a, b) => a.position - b.position);
  let currentPosition = 0;
  for (let bit of bits) {
    while (currentPosition < bit.position) {
      html.innerHTML += "&nbsp;";
      currentPosition++;
    }
    console.log("Appending!", html, bit.html);
    var res = html.appendChild(bit.html);
    console.log("Appended!", html, res);
    currentPosition += bit.size;
  }
  return html;
}
function createSpan(cls, inner) {
  let span = document.createElement("span");
  span.classList.add(cls);
  if (inner != null) {
    span.innerHTML = inner;
  }
  return span;
}

// chorddb.ts
var getBitSize = function(bit) {
  if (bit.type == BitType.Chord) {
    let chord = bit.chord;
    if (chord == undefined) {
      console.error("Found a chord bit without chord!", bit);
      return bit.text.length;
    }
    return chord.chord.length + chord.fingering.length + 2;
  }
  return bit.text.length;
};
var isInput = function(element) {
  return element != null && element != null && element instanceof HTMLInputElement;
};

class ChordDB {
  chords = [];
  selectedChord = 0;
  initTablature(lines) {
    this.buildLines("tablature", lines);
  }
  buildLines(contentId, lines) {
    let content = document.getElementById(contentId);
    if (content == undefined) {
      console.error("Could not find element with id " + contentId);
    }
    let pre = document.createElement("div");
    pre.classList.add("tablature");
    pre.classList.add("font-mono");
    for (let line of lines) {
      this.buildLine(line).forEach((html) => {
        pre.appendChild(html);
        pre.appendChild(document.createElement("br"));
      });
    }
    content?.appendChild(pre);
  }
  updateSongDrawer(chord, isOpen) {
    console.log("updateSongDrawer:", chord, isOpen);
    if (!isOpen || chord != null && chord != this.selectedChord) {
      let selectedChord = this.chords[this.selectedChord];
      selectedChord.chord.classList.remove("chord-selected");
      selectedChord.fingering.classList.remove("fingering-selected");
    }
    if (chord != null) {
      this.selectedChord = chord % this.chords.length;
      let selectedChord = this.chords[this.selectedChord];
      selectedChord.chord.classList.add("chord-selected");
      selectedChord.fingering.classList.add("fingering-selected");
    }
    var checkbox = document.getElementById("song-drawer-checkbox");
    if (isInput(checkbox)) {
      checkbox.checked = isOpen;
    }
    var drawer = document.getElementById("drawer");
    if (!drawer)
      return;
    if (isOpen) {
      drawer.classList.remove("song-drawer-closed");
    } else {
      drawer.classList.add("song-drawer-closed");
    }
  }
  buildLine(line) {
    let bits = [...line];
    bits.sort((a, b) => b.position - a.position);
    let lines = [new RenderLine];
    for (let linebit of bits) {
      let lineIndex = 0;
      let lastBitPosition = linebit.position + getBitSize(linebit);
      while (lastBitPosition > lines[lineIndex].lastPosition) {
        lineIndex++;
        while (lines.length <= lineIndex) {
          lines.push(new RenderLine);
        }
      }
      for (let i = 0;i < lineIndex; i++) {
        lines[i].addBit({
          html: createSpan(linebit.type == BitType.Chord ? "chord" : "lyric", i == 0 ? "v" : "|"),
          position: linebit.position,
          size: 1
        });
      }
      let currentLine = lines[lineIndex];
      let chord = linebit.chord;
      if (chord != null) {
        let chordSpan = createSpan("chord", chord.chord);
        currentLine.addBit({
          html: chordSpan,
          position: linebit.position,
          size: chord.chord.length
        });
        let fingering = createSpan("fingering", "(" + chord.fingering + ")");
        let chordIndex = this.chords.length;
        console.log("Adding listenr for ", chord, chordIndex, fingering);
        fingering.dataset.index = chordIndex.toString();
        fingering.addEventListener("click", (e) => {
          console.log("Click!", e, e.target, chordIndex, this);
          this.updateSongDrawer(e.target.dataset.index, true);
        }, false);
        this.chords.push({ chord: chordSpan, fingering });
        currentLine.addBit({
          html: fingering,
          position: linebit.position + chord.chord.length,
          size: chord.fingering.length + 2
        });
        console.log("Added to line!", fingering, currentLine);
      } else {
        currentLine.addBit({
          html: createSpan("lyric", linebit.text),
          position: linebit.position,
          size: linebit.text.length
        });
      }
    }
    lines.reverse();
    let res = lines.map((line2) => renderInSingleLine(line2.bits));
    return res;
  }
}
window.chorddb = new ChordDB;
var BitType;
(function(BitType2) {
  BitType2["Text"] = "text";
  BitType2["Chord"] = "chord";
})(BitType || (BitType = {}));

class RenderLine {
  bits = [];
  lastPosition = Number.MAX_SAFE_INTEGER;
  addBit(bit) {
    this.bits.push(bit);
    this.lastPosition = this.lastPosition == undefined ? bit.position : Math.min(this.lastPosition, bit.position);
  }
}
