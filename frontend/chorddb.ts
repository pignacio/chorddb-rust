import { type RenderBit, renderInSingleLine, createSpan } from "./render"

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

function buildLines(contentId: string, lines: LineBit[][]) {
    let content = document.getElementById(contentId);
    if (content == undefined) {
        console.error("Could not find element with id " + contentId);
    }
    let pre = document.createElement("div")
    pre.classList.add("tablature")
    pre.classList.add("font-mono")

    for (let line of lines) {
        buildLine(line).forEach(html => {
            pre.appendChild(html)
            pre.appendChild(document.createElement("br"))
        })
    }

    content?.appendChild(pre)
}
window.buildLines = buildLines

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

function buildLine(line: LineBit[]): HTMLElement[] {
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
            currentLine.addBit({
                html: createSpan("chord", chord.chord),
                position: linebit.position,
                size: chord.chord.length,
            })
            var fingering = createSpan("fingering", "(" + chord.fingering + ")")
            fingering.addEventListener('click', () => updateSongDrawer(true), false);

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

    lines.reverse()
    let res = lines.map(line => renderInSingleLine(line.bits))
    return res
}


function toggleSongDrawer() {
  var checkbox = document.getElementById("song-drawer-checkbox");
  checkbox.checked = !checkbox.checked
  updateSongDrawer(checkbox.checked)
}
window.toggleSongDrawer = toggleSongDrawer


function updateSongDrawer(isOpen: boolean) {
  console.log("updateSongDrawer: " + isOpen);
  document.getElementById("song-drawer-checkbox").checked = isOpen;
  var drawer = document.getElementById("drawer");
  if (isOpen) {
    drawer.classList.remove("song-drawer-closed")
  } else {
    drawer.classList.add("song-drawer-closed")
  }
}
