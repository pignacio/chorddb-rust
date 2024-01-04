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
    console.log("Building lines into #" + contentId);
    console.log(lines);
    let content = document.getElementById(contentId);
    if (content == undefined) {
        console.error("Could not find element with id " + contentId);
    }
    let pre = document.createElement("pre")
    pre.classList.add("tab")

    for (let line of lines) {
        buildLine(line).forEach(html => {
            pre.appendChild(html)
            pre.innerHTML += "\n"
        })

    }

    content?.appendChild(pre)
}

function getBitSize(bit: LineBit) {
    if (bit.type == BitType.Chord) {
        let chord = bit.chord;
        if (chord == undefined) {
            console.log("Found a chord bit without chord!", bit);
            return bit.text.length;
        }
        return chord.chord.length + chord.fingering.length + 2;
    }
    return bit.text.length;
}

function buildLine(line: LineBit[]): HTMLElement[] {
    console.log("Building line", line)

    let bits = [...line]
    bits.sort((a, b) => b.position - a.position)

    let lines: RenderBit[][] = [[]]
    let lastPosition: number | undefined = undefined
    let lastLineIndex: number = 0;

    for (let linebit of bits) {
        let currentLine: RenderBit[]
        if (lastPosition == undefined) {
            lastLineIndex = 0;
        } else {
            let lastBitPosition = linebit.position + getBitSize(linebit)
            if (lastBitPosition > lastPosition) {
                // Overlap
                lastLineIndex++
                while (lines.length <= lastLineIndex) {
                    lines.push([])
                }
                for (let i = 0; i < lastLineIndex; i++) {
                    lines[i].push({
                        html: createSpan(linebit.type == BitType.Chord ? "chord" : "lyric", "|"),
                        position: linebit.position,
                        size: 1,
                    })
                }
            } else {
                lastLineIndex = 0;
            }
        }
        currentLine = lines[lastLineIndex]

        lastPosition = linebit.position;

        let chord = linebit.chord;
        if (chord != undefined) {
            currentLine.push({
                html: createSpan("chord", chord.chord),
                position: linebit.position,
                size: chord.chord.length,
            })
            currentLine.push({
                html: createSpan("fingering", "(" + chord.fingering + ")"),
                position: linebit.position + chord.chord.length,
                size: chord.fingering.length + 2,
            })
        } else {
            currentLine.push({
                html: createSpan("lyric", linebit.text),
                position: linebit.position,
                size: linebit.text.length,
            })
        }

    }

    lines.reverse()
    console.log("Lines:", lines)
    let res = lines.map(line => renderInSingleLine(line))
    console.log("Result:", res)
    return res
}

window.buildLines = buildLines