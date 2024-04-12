type ChordBit = {
  type: 'chord',
  text: string,
  chord: string,
}

type FingeringBit = {
  type: 'fingering',
  text: string,
  chord: string,
  fingering: string,
}

type TextBit = {
  type: 'text',
  text: string,
  style?: string,
}

type ComboBit = {
  type: 'combo',
  bits: TabBit[]
}

export type TabBit = {
  position: number,
} & (ChordBit | FingeringBit | TextBit | ComboBit)


export function getBitSize(bit: TabBit): number {
  if (bit.type == "combo") {
    return bit.bits.map(getBitSize).reduce((a,b) => a + b, 0);
  }
  return bit.text.length;
}

export function getBitStyle(bit: TabBit): string | undefined {
  if (bit.type == "chord" || bit.type == "fingering") {
    return bit.type;
  } else if (bit.type == "text") {
    return bit.style;
  }
  return undefined;
}

export type TabLine = TabBit[]

export type Tablature = {
  lines: TabLine[]
}


function expandBits(tablature: Tablature, func: (bit: TabBit) => TabBit[]): Tablature {
  return expandLines(tablature, (line) => [line.flatMap(func)]);
}

function expandLines(tablature: Tablature, func: (line: TabBit[]) => TabBit[][]): Tablature {
  return {
    lines: tablature.lines.flatMap(func)
  };
}

export function expandAll(tablature: Tablature, fingerings: {[key:string] : string}): Tablature {
  tablature = expandBits(tablature, bit => expandFingerings(bit, fingerings));
  tablature = expandLines(tablature, expandCollisions);
  tablature = expandLines(tablature, expandWhitespace);
  tablature = expandBits(tablature, flattenCombos);
  return tablature;
}

export function expandFingerings(bit: TabBit, fingerings: {[key:string]: string}): TabBit[] {
  if (bit.type == "chord") {
    const chord = bit.chord;
    const fingering = fingerings[chord] ?? "UNKNOWN"
    const fingeringBit: TabBit = {
      type: "fingering",
      chord: chord,
      fingering: fingering,
      position: bit.position + bit.text.length,
      text: `(${fingering})`,
    };
    return [{
      type: 'combo',
      bits: [bit, fingeringBit],
      position: bit.position,
    }];
  } else {
    return [bit];
  }
}


export function expandWhitespace(line: TabBit[]): TabBit[][] {
  let currentPosition = 0;
  line.sort((a, b) => a.position - b.position);

  const newLine: TabBit[] = [];
  for (const bit of line) {
    if (currentPosition < bit.position) {
      const diff = bit.position - currentPosition
      newLine.push({
        position: currentPosition,
        text: " ".repeat(diff),
        type: "text"
      })
      currentPosition += diff;
    }

    newLine.push(bit);
    currentPosition += getBitSize(bit);
  }

  return [newLine];
}

class RenderLine {
  bits: TabBit[] = [];
  lastPosition: number = Number.MAX_SAFE_INTEGER;

  addBit(bit: TabBit) {
    this.bits.push(bit)
    this.lastPosition = this.lastPosition == undefined ? bit.position : Math.min(this.lastPosition, bit.position)
  }
}

export function expandCollisions(line: TabBit[]): TabBit[][] {
  const bits = [...line];
  // bits.sort((a, b) => a.position - b.position);

  // // Assign indices in forward order
  // for (let bit of bits) {
  //   if (bit.chord != undefined) {
  //     bit.chordIndex = this.chordCount++;
  //   }
  // }

  // Perform pre-rendering and overlap calculations in reverse order
  bits.sort((a, b) => b.position - a.position);

  const lines: RenderLine[] = [new RenderLine()]
  for (const bit of bits) {
    let lineIndex : number = 0;
    const lastBitPosition = bit.position + getBitSize(bit);
    while (lastBitPosition > lines[lineIndex].lastPosition) {
      lineIndex++;
      while (lines.length <= lineIndex) {
        lines.push(new RenderLine())
      }
    }

    lines[lineIndex].addBit(bit);

    // Render arrow
    for (let i = 0; i < lineIndex; i++) {
      lines[i].addBit({
        type: "text",
        text: i == 0 ? "v" : "|",
        position: bit.position,
        style: "chord",
      })
    }
  }

  // Finally render each line in forward order
  return lines.reverse().map(renderLine => renderLine.bits);
}


export function flattenCombos(bit: TabBit): TabBit[] {
  if (bit.type == 'combo') {
    return bit.bits.flatMap(flattenCombos);

  } else {
    return [bit];
  }
}



export function findFirstChord(tablature: Tablature) {
  for (const line of tablature.lines) {
    for (const bit of line) {
      if (bit.type == "chord") {
        return bit.chord;
      }
    }
  }
  console.error("Could not find first chord in tablature :(")
}
