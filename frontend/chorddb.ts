import { type RenderBit, renderInSingleLine, createSpan } from "./render"

enum LineType {
    Text = "text",
    Chord = "chord",
}

interface Chord {
    chord: string,
    fingering: string,
    position: number
}

interface Line {
    type: LineType,
    text: string,
    chords?: Chord[]
}

function buildLines(contentId: string, lines: Line[]) {
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

function buildLine(line: Line): HTMLElement[] {
    switch (line.type) {
        case LineType.Text:
            return [buildLyricLine(line.text)]
        case LineType.Chord:
            return buildChordLine(line)
    }
    console.error("Invalid line type:" + line.type)
}

function buildLyricLine(text: string) {
    return createSpan("lyric", text)
}

function buildChordLine(line: Line): HTMLElement[] {
    console.log("Building chord line", line)
    if (line.chords == undefined) {
        console.error("Attempted to build a chord line without chords: " + line);
        return [buildLyricLine(line.text)];
    }

    let chords = [...line.chords]
    chords.sort((a, b) => b.position - a.position)

    let lines: RenderBit[][] = [[]]
    let lastPosition: number | undefined = undefined
    let lastLineIndex: number = 0;

    for (let chord of chords) {
        let currentLine: RenderBit[]
        if (lastPosition == undefined) {
            lastLineIndex = 0;
        } else {
            let lastChordPosition = chord.position + chord.chord.length + chord.fingering.length + 2
            if (lastChordPosition > lastPosition) {
                // Overlap
                lastLineIndex++
                while (lines.length <= lastLineIndex) {
                    lines.push([])
                }
                for (let i = 0; i < lastLineIndex; i++) {
                    lines[i].push({
                        html: createSpan("chord", "|"),
                        position: chord.position,
                        size: 1,
                    })
                }
            } else {
                lastLineIndex = 0;
            }
        }
        currentLine = lines[lastLineIndex]
            
        lastPosition = chord.position;

        currentLine.push({
            html: createSpan("chord", chord.chord),
            position: chord.position,
            size: chord.chord.length,
        })
        currentLine.push({
            html: createSpan("fingering", "(" + chord.fingering + ")"),
            position: chord.position + chord.chord.length,
            size: chord.fingering.length + 2,
        })
    }

    lines.reverse()
    console.log("Lines:", lines)
    let res = lines.map(line => renderInSingleLine(line))
    console.log("Result:", res)
    return res
}

window.buildLines = buildLines