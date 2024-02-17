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
var findChord = function(index) {
  let element = document.querySelector('[data-chord-index="' + index + '"]');
  if (element != null && element instanceof HTMLElement) {
    return element;
  }
  console.log("Could not find chord for index", index);
  return null;
};
var findFingering = function(index) {
  let element = document.querySelector('[data-fingering-index="' + index + '"]');
  if (element != null && element instanceof HTMLElement) {
    return element;
  }
  console.log("Could not find chord for index", index);
  return null;
};

class ChordDB {
  chordCount = 0;
  selectedChord = 0;
  initTablature(lines) {
    this.buildLines("tablature", lines);
    document.querySelectorAll("[data-fingering-index]").forEach((f) => {
      console.log("Adding listener for ", f);
      f.addEventListener("click", (e) => {
        if (!(e.target instanceof HTMLElement)) {
          return;
        }
        let index = e.target.dataset.fingeringIndex;
        console.log("Click!", e, e.target, index, this);
        this.updateSongDrawer(index != null ? parseInt(index) : null, true);
      }, false);
    });
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
      findChord(this.selectedChord)?.classList?.remove("chord-selected");
      findFingering(this.selectedChord)?.classList?.remove("fingering-selected");
    }
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
    bits.sort((a, b) => a.position - b.position);
    for (let bit of bits) {
      if (bit.chord != null) {
        bit.chordIndex = this.chordCount++;
      }
    }
    bits.reverse();
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
        if (linebit.chordIndex == undefined) {
          console.error("Missing chordIndex", linebit);
          continue;
        }
        let chordSpan = createSpan("chord", chord.chord);
        chordSpan.dataset.chordIndex = linebit.chordIndex.toString();
        let fingering = createSpan("fingering", "(" + chord.fingering + ")");
        fingering.dataset.fingeringIndex = linebit.chordIndex.toString();
        fingering.dataset.fingeringChord = chord.chord;
        currentLine.addBit({
          html: chordSpan,
          position: linebit.position,
          size: chord.chord.length
        });
        currentLine.addBit({
          html: fingering,
          position: linebit.position + chord.chord.length,
          size: chord.fingering.length + 2
        });
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
